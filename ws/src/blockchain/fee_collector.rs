use std::{pin::Pin, str::FromStr, sync::Arc};
use async_stream::try_stream;
use futures::Stream;
use eyre::Result;
use log::{error, info};
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::{
  commitment_config::CommitmentConfig, compute_budget::ComputeBudgetInstruction, message::Message,
  program_pack::Pack, pubkey::Pubkey, signer::Signer, transaction::Transaction,
};
use spl_associated_token_account::{
  get_associated_token_address_with_program_id, instruction::create_associated_token_account,
};
use spl_token_2022::{
  extension::transfer_fee::instruction::withdraw_withheld_tokens_from_accounts,
  instruction::transfer_checked, state::Mint,
};
use onlytax_blockchain::helius::helius_api::HeliusApi;
use crate::utils::config::{SolanaKeypair, SolanaPubkey};

const DEFAULT_PRIORITY_FEE: u64 = 50_000;

struct WithheldAccount {
  account: Pubkey,
  amount: u64,
}

pub struct FeeCollector {
  rpc_client: Arc<RpcClient>,
  operator_keypair: SolanaKeypair,
  treasury: SolanaPubkey,
  protocol_fee_bps: u64,
  helius_api: Arc<HeliusApi>,
}

const ACCOUNT_BATCH_SIZE: usize = 25;

impl FeeCollector {
  pub fn new(
    rpc_client: Arc<RpcClient>,
    operator_keypair: SolanaKeypair,
    treasury: SolanaPubkey,
    protocol_fee_bps: u64,
    helius_api: Arc<HeliusApi>,
  ) -> Self {
    Self {
      rpc_client,
      operator_keypair,
      treasury,
      protocol_fee_bps,
      helius_api,
    }
  }

  pub fn collect<'a>(
    &'a self,
    mint: &'a Pubkey,
    withdraw_withheld_authority: &'a Pubkey,
  ) -> Pin<Box<dyn Stream<Item = Result<(Vec<u8>, usize, usize)>> + Send + 'a>> {
    let stream = try_stream! {
      let mint_account = Mint::unpack_from_slice(
        &self.rpc_client.get_account_data(mint).await?,
      )?;

      let protocol_ata = self.create_protocol_ata(mint).await?;
      let withheld_token_accounts = self.get_withheld_token_accounts(mint).await?;
      let withheld_token_accounts_len = withheld_token_accounts.len();

      // Send tokens to the withdraw_withheld ascosciated token. In the future we will allow
      // users to choose the destination ata
      let withdraw_withheld_ata = get_associated_token_address_with_program_id(
        withdraw_withheld_authority,
        mint,
        &spl_token_2022::ID,
      );

      for batch_accounts in withheld_token_accounts.chunks(ACCOUNT_BATCH_SIZE) {
        let token_accounts: Vec<&Pubkey> = batch_accounts.iter().map(|a| &a.account).collect();
        let batch_fees: u64 = batch_accounts.iter().map(|a| &a.amount).sum();

        let ix = withdraw_withheld_tokens_from_accounts(
          &spl_token_2022::ID,
          &mint,
          &withdraw_withheld_ata,
          &withdraw_withheld_authority,
          &[],
          &token_accounts,
        )?;

        let protocol_fee = (batch_fees * self.protocol_fee_bps) / 100;
        let protocol_fee_ix = transfer_checked(
          &spl_token_2022::ID,
          &withdraw_withheld_ata,
          mint,
          &protocol_ata,
          withdraw_withheld_authority,
          &[withdraw_withheld_authority],
          protocol_fee,
          mint_account.decimals,
        )?;
    
        let message = Message::new(
          &[ComputeBudgetInstruction::set_compute_unit_price(DEFAULT_PRIORITY_FEE), ix, protocol_fee_ix],
          Some(withdraw_withheld_authority),
        );
        let tx = bincode::serialize(&Transaction::new_unsigned(message))?;
      
        yield (tx, ACCOUNT_BATCH_SIZE, withheld_token_accounts_len)
      }
    };

    Box::pin(stream)
  }

  async fn get_withheld_token_accounts(&self, mint: &Pubkey) -> Result<Vec<WithheldAccount>> {
    info!("Reading token accounts with withheld fees");
    
    let withheld_accounts = self.helius_api.fetch_token_accounts(&mint.to_string()).await?
    .iter()
    .filter_map(|ta| {
      let Some(token_extensions) = &ta.token_extensions else {
        return None
      };

      let withheld_amount = token_extensions.transfer_fee_amount.withheld_amount;

      if withheld_amount == 0 {
        return None
      }

      let account = Pubkey::from_str(&ta.address);
      if account.is_err() {
        return None
      }

      Some(WithheldAccount {
        account: account.unwrap(),
        amount: withheld_amount,
      })
    })
    .collect::<Vec<WithheldAccount>>();

    let total_fees: u64 = withheld_accounts.iter().map(|w| w.amount).sum();
    info!("Found total {} token accounts with withheld fees of {}", withheld_accounts.len(), total_fees);
  
    Ok(withheld_accounts)
  }

  async fn create_protocol_ata(&self, mint: &Pubkey) -> Result<Pubkey> {
    let ata = get_associated_token_address_with_program_id(
      &self.treasury,
      mint,
      &spl_token_2022::ID,
    );
    
    // check if it already exists
    if let Some(_) = self.rpc_client.get_account_with_commitment(&ata, CommitmentConfig::confirmed()).await?.value {
      return Ok(ata)
    }
    
    let operator_pubkey = self.operator_keypair.pubkey();
    let ix = create_associated_token_account(
      &operator_pubkey,
      &self.treasury,
      mint,
      &spl_token_2022::ID,
    );

    let priority_fee = self.helius_api.fetch_priority_fee().await;
    let priority_fee = match priority_fee {
      Ok(response) => response.priority_fee_levels.high as u64,
      Err(err) =>  {
        error!("Failed to get priority. Will use the default value {}: {}", DEFAULT_PRIORITY_FEE, err);
        DEFAULT_PRIORITY_FEE
      }
    };

    let message = Message::new(
      &[ComputeBudgetInstruction::set_compute_unit_price(priority_fee), ix],
      Some(&operator_pubkey)
    );
    let tx = Transaction::new(
      &[&self.operator_keypair],
      message,
      self.rpc_client.get_latest_blockhash().await?,
    );
  
    info!("Sending create protocol ata transaction for token {}", mint);
    let tx_id = self.rpc_client.send_and_confirm_transaction(&tx).await?;
    info!("Create protocol ata transaction for token {} executed {}", mint, tx_id); 

    Ok(ata)
  }
}
