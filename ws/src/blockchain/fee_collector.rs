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
  parse_account_data::SplTokenAdditionalData, parse_token::{parse_token_v2, TokenAccountType}, parse_token_extension::UiExtension, UiAccountEncoding, UiDataSliceConfig
};
use solana_sdk::{
  commitment_config::CommitmentConfig, compute_budget::ComputeBudgetInstruction,
  instruction::Instruction, message::Message, pubkey::Pubkey, transaction::Transaction,
};
use spl_associated_token_account::get_associated_token_address_with_program_id;
use spl_token_2022::extension::transfer_fee::instruction::withdraw_withheld_tokens_from_accounts;

struct WitheldAccount {
  account: Pubkey,
  amount: u64,
}

pub struct FeeCollector {
  rpc_client: Arc<RpcClient>,
}

// We're using extentions so the mint acount is not the classic 165 bytes account.
// TODO: we need to dynamcally load this number since we're dealing with multiple mint accounts
const MINT_ACCOUNT_SIZE: u64 = 346;
const ACCOUNT_BATCH_SIZE: usize = 10;

impl FeeCollector {
  pub fn new(rpc_client: Arc<RpcClient>) -> Self {
    Self {rpc_client}
  }

  pub fn collect<'a, 'b: 'a>(
    &'a self,
    mint: &'b Pubkey,
    decimals: u8,
    withdraw_withheld_authority: &'b Pubkey,
  ) -> Pin<Box<dyn Stream<Item = Result<Vec<u8>>> + Send + 'a>> {
    let stream = try_stream! {
      let source_accounts = self.get_source_accounts(mint, decimals).await?;
      let source_accounts: Vec<&Pubkey> = source_accounts.iter().map(|a| a).collect();

      // Send tokens to the withdraw_withheld ascosciated token. In the future we will allow
      // users to choose the destination ata
      let withdraw_withheld_ata = get_associated_token_address_with_program_id(
        withdraw_withheld_authority,
        mint,
        &spl_token_2022::ID,
      );

      for batch_accounts in source_accounts.chunks(ACCOUNT_BATCH_SIZE) {
        let ix = withdraw_withheld_tokens_from_accounts(
          &spl_token_2022::ID,
          &mint,
          &withdraw_withheld_ata,
          &withdraw_withheld_authority,
          &[],
          batch_accounts,
        )?;
    
        let message = Message::new(
          // TODO: use https://marketplace.quicknode.com/add-on/solana-priority-fee to get the real value
          &[Self::create_priority_fee_ix(1000), ix],
          Some(withdraw_withheld_authority),
        );
        let tx = bincode::serialize(&Transaction::new_unsigned(message))?;
      
        yield tx
      }
    };

    Box::pin(stream)
  }

  async fn get_source_accounts(&self, mint: &Pubkey, decimals: u8) -> Result<Vec<WitheldAccount>> {
    info!("Reading token accounts with withheld fees");
    let memcmp = RpcFilterType::Memcmp(Memcmp::new(0, MemcmpEncodedBytes::Base58(mint.to_string())));
    let slot = self.rpc_client.get_slot_with_commitment(CommitmentConfig::confirmed()).await?;
    let config = RpcProgramAccountsConfig {
      filters: Some(vec![memcmp,]),
      account_config: RpcAccountInfoConfig {
        encoding: Some(UiAccountEncoding::Base64),
        data_slice:  Some(UiDataSliceConfig {
          offset: 0,
          length: MINT_ACCOUNT_SIZE as usize,
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
    let source_accounts: Vec<WitheldAccount> = token_accounts
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
  
      let witheld_amount = ta.extensions.iter().find(|e| {
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
  
      let Some(amount) = witheld_amount else {
        return None
      };

      Some(WitheldAccount {account: pk, amount})
    })
    .collect();
  
    info!("Found total {} token accounts with withheld fees of {}", source_accounts.len(), pending_fees);
  
    Ok(source_accounts)
  }

  /// There are 10^6 micro-lamports in one lamport. 2_500_000 micro lamport is => 2_500_000 / 1_000_000 = 2.5 Lamports
  /// The total fees will be: fees = compute budget * U = 200,000 * 2.5 = 500,000 lamport or 0.0005 SOL.
  /// This is 100 higher than the default fee which is 0.000005 SOL
  fn create_priority_fee_ix(priority_fee_micro_lamports: u64) -> Instruction {
    ComputeBudgetInstruction::set_compute_unit_price(priority_fee_micro_lamports)
  }
}
