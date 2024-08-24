use std::{pin::Pin, sync::Arc};
use async_stream::try_stream;
use futures::Stream;
use eyre::Result;
use log::info;
use solana_client::{
  nonblocking::rpc_client::RpcClient, rpc_config::{RpcAccountInfoConfig, RpcProgramAccountsConfig},
  rpc_filter::{Memcmp, MemcmpEncodedBytes, RpcFilterType}
};
use solana_account_decoder::{
  parse_account_data::SplTokenAdditionalData, parse_token::{parse_token_v2, TokenAccountType},
  parse_token_extension::UiExtension, UiAccountEncoding, UiDataSliceConfig,
};
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

use crate::{blockchain::priority_fee::create_priority_fee_ix, utils::config::{SolanaKeypair, SolanaPubkey}};

use super::priority_fee::DEFAULT_PRIORITY_FEE;

struct WithheldAccount {
  account: Pubkey,
  amount: u64,
}

pub struct FeeCollector {
  rpc_client: Arc<RpcClient>,
  operator_keypair: SolanaKeypair,
  treasury: SolanaPubkey,
  protocol_fee_bps: u64,
  priority_fee_rpc: String,
}

// We're using transfer fee extentions so the token acount is not the classic 165 bytes account.
// TODO: find a better way to find the correct size of the token size
const TOKEN_ACCOUNT_SIZE: u64 = 346;
const ACCOUNT_BATCH_SIZE: usize = 1;

impl FeeCollector {
  pub fn new(
    rpc_client: Arc<RpcClient>,
    operator_keypair: SolanaKeypair,
    treasury: SolanaPubkey,
    protocol_fee_bps: u64,
    priority_fee_rpc: String,
  ) -> Self {
    Self {
      rpc_client,
      operator_keypair,
      treasury,
      protocol_fee_bps,
      priority_fee_rpc,
    }
  }

  pub fn collect<'a, 'b: 'a>(
    &'a self,
    mint: &'b Pubkey,
    withdraw_withheld_authority: &'b Pubkey,
  ) -> Pin<Box<dyn Stream<Item = Result<(Vec<u8>, usize)>> + Send + 'a>> {
    let stream = try_stream! {
      let mint_account = Mint::unpack_from_slice(
        &self.rpc_client.get_account_data(mint).await?,
      )?;

      let protocol_ata = self.create_protocol_ata(mint).await?;
      let withheld_token_accounts = self.get_withheld_token_accounts(mint, mint_account.decimals).await?;
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
      
        yield (tx, withheld_token_accounts_len)
      }
    };

    Box::pin(stream)
  }

  async fn get_withheld_token_accounts(&self, mint: &Pubkey, decimals: u8) -> Result<Vec<WithheldAccount>> {
    info!("Reading token accounts with withheld fees");
    let memcmp = RpcFilterType::Memcmp(Memcmp::new(0, MemcmpEncodedBytes::Base58(mint.to_string())));
    let slot = self.rpc_client.get_slot_with_commitment(CommitmentConfig::confirmed()).await?;

    let config = RpcProgramAccountsConfig {
      filters: Some(vec![memcmp,]),
      account_config: RpcAccountInfoConfig {
        encoding: Some(UiAccountEncoding::Base64),
        data_slice:  Some(UiDataSliceConfig {
          offset: 0,
          length: TOKEN_ACCOUNT_SIZE as usize,
      }),
        commitment: Some(CommitmentConfig::processed()),
        min_context_slot: Some(slot),
      },
      with_context: Some(true),
      sort_results: None,
    };
  
    let token_accounts = self.rpc_client.get_program_accounts_with_config(&spl_token_2022::ID, config).await?;
    info!("Found total {} token accounts", token_accounts.len());
  
    let mut pending_fees = 0;
    let source_accounts: Vec<WithheldAccount> = token_accounts
    .into_iter()
    .filter_map(|(pk, a)| {
      let Ok(ta_type) = parse_token_v2(&a.data, Some(&SplTokenAdditionalData::with_decimals(decimals))) else {
        return None
      };
  
      Some((pk, ta_type))
    })
    .filter_map(|(pk, ta_type)| {
      let TokenAccountType::Account(ta) = ta_type else {
        return None
      };
  
      let withheld_amount = ta.extensions.iter().find(|e| {
        let UiExtension::TransferFeeAmount(fee_amount) = e else {
          return false
        };
  
        pending_fees += fee_amount.withheld_amount;
        fee_amount.withheld_amount > 0
      })
      .map(|e| {
        let UiExtension::TransferFeeAmount(fee_amount) = e else {
          panic!("Should be a transfer fee extension");
        };

        fee_amount.withheld_amount
      });
  
      let Some(amount) = withheld_amount else {
        return None
      };

      Some(WithheldAccount {account: pk, amount})
    })
    .collect();
  
    info!("Found total {} token accounts with withheld fees of {}", source_accounts.len(), pending_fees);
  
    Ok(source_accounts)
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

    let message = Message::new(
      &[create_priority_fee_ix(&self.priority_fee_rpc).await, ix],
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
