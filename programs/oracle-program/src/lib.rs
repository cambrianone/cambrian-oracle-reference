#![allow(unexpected_cfgs)]

mod cambrian_accounts;

use crate::cambrian_accounts::PoAState;
use crate::cambrian_accounts::ProposalStorage;
use crate::cambrian_accounts::CAMBRIAN_ID;
use crate::cambrian_accounts::HANDLE_PROPOSAL_DISCRIMINATOR;
use anchor_lang::prelude::*;
use anchor_lang::solana_program::sysvar::{instructions, instructions::ID as InstructionId};
use anchor_lang::Accounts;
use anchor_lang::Discriminator;

cfg_if::cfg_if! {
    if #[cfg(feature = "devnet")] {
        declare_id!("ECb6jyKXDTE8NjVjsKgNpjSjcv4h2E7JQ42yKqWihBQE");
    } else if #[cfg(feature = "mainnet")] {
        declare_id!("2B68RRPaApitczb89EhDT18Cxeobz4NG66nyYoSgdQDD");
    } else {
        declare_id!("2B68RRPaApitczb89EhDT18Cxeobz4NG66nyYoSgdQDD");
    }
}

#[error_code]
#[derive(PartialEq, Eq)]
pub enum OracleError {
    #[msg(Oracle storage not found)]
    OracleStorageNotFound,
    #[msg(Failed to load handle proposal instruction)]
    FailedToLoadHandleProposalInstruction,
}

const STORAGE_ACCOUNT_INDEX: usize = 12;
const POA_STATE_INDEX: usize = 0;

#[program]
pub mod timestamp_oracle_program {
    use std::str::FromStr;
    use super::*;

    #[derive(Accounts)]
    #[instruction(poa_state_key: Vec<u8>)]
    pub struct CheckOracle<'info> {
        /// CHECK: nyd
        // TODO: Add storage account check
        proposal_storage: AccountInfo<'info>,
        /// CHECK: nyd
        poa_state: Account<'info, PoAState>,
        /// CHECK: nyd
        #[account(address = InstructionId)]
        pub instructions: AccountInfo<'info>,
    }

    pub fn check_oracle(ctx: Context<CheckOracle>, poa_state_key: Vec<u8>) -> Result<()> {
        let handle_proposal_ix =
            instructions::load_current_index_checked(&ctx.accounts.instructions)
                .map_err(|_| error!(OracleError::OracleStorageNotFound))?;

        let ix = instructions::load_instruction_at_checked(
            handle_proposal_ix as usize,
            &ctx.accounts.instructions,
        )?;

        assert_eq!(CAMBRIAN_ID, ix.program_id);

        assert_eq!(HANDLE_PROPOSAL_DISCRIMINATOR, ix.data[0..8]);
        assert_eq!(
            ctx.accounts.proposal_storage.key,
            &ix.accounts[STORAGE_ACCOUNT_INDEX].pubkey
        );
        assert_eq!(
            ctx.accounts.poa_state.key(),
            ix.accounts[POA_STATE_INDEX].pubkey
        );

        //Check poa_state
        assert_eq!(
            Pubkey::find_program_address(&[PoAState::SEED, &poa_state_key], &CAMBRIAN_ID).0,
            ctx.accounts.poa_state.key()
        );
        assert_eq!(
            Pubkey::find_program_address(&[PoAState::SEED, &poa_state_key], &CAMBRIAN_ID).1,
            ctx.accounts.poa_state.bump
        );

        if *ctx.accounts.proposal_storage.owner != CAMBRIAN_ID {
            msg!("No provided data in this proposal");
            return Ok(());
        }

        let data = ctx.accounts.proposal_storage.try_borrow_mut_data()?;

        let proposal_storage = ProposalStorage::try_deserialize(&mut &**data)?;

        let mut oracle_results = vec![];
        for (voter_index, voter) in proposal_storage.voters.iter().enumerate() {
            let (start_offset, end_offset) = proposal_storage
                .get_voter_storage(voter, data.len(), proposal_storage.voters.len())
                .ok_or(error!(OracleError::OracleStorageNotFound))?;

            let voter_data = data[start_offset..end_offset].to_vec();

            let data_as_utf = String::from_utf8(voter_data.clone().into_iter().filter(|x| x.is_ascii_digit()).collect::<Vec<_>>());

            let time_data = data_as_utf.as_ref().map(|raw_time| u64::from_str(raw_time.as_str()));

            msg!(
                "Timestamp Oracle Voter: index={:?}, pubkey={:?}, local time={:?}, data as utf={:?}, data_raw:={:?}",
                voter_index,
                voter,
                time_data.as_ref().map(|res| res.as_ref().map(|timestamp| chrono::DateTime::from_timestamp(*timestamp as i64,0))),
                data_as_utf,
                voter_data
            );

            match time_data {
                Ok(Ok(time_data)) => oracle_results.push(time_data),
                _ => (),
            }
        }

        if oracle_results.len() != 0 {
            let min = *oracle_results.iter().min().unwrap();
            let max = *oracle_results.iter().max().unwrap();
            let sum: u64 = oracle_results.iter().sum();
            let count = oracle_results.len() as f64;
            let mean = sum as f64 / count;

            let median = calculate_median(&oracle_results);
            let variance = calculate_variance(&oracle_results, mean);
            let std_dev = variance.sqrt();

            let statistics = Statistics {
                min,
                max,
                mean,
                median,
                variance,
                std_dev,
            };

            msg!("Oracle data statistics: {:?}", statistics);
        }

        Ok(())
    }

}


fn calculate_median(data: &[u64]) -> f64 {
    let mut sorted_data = data.to_vec();
    sorted_data.sort_unstable();
    let mid = sorted_data.len() / 2;

    if sorted_data.len() % 2 == 0 {
        (sorted_data[mid - 1] as f64 + sorted_data[mid] as f64) / 2.0
    } else {
        sorted_data[mid] as f64
    }
}

fn calculate_variance(data: &[u64], mean: f64) -> f64 {
    let sum_squared_diffs: f64 = data.iter()
        .map(|&value| (value as f64 - mean).powi(2))
        .sum();

    sum_squared_diffs / data.len() as f64
}

#[derive(Debug)]
struct Statistics {
    min: u64,
    max: u64,
    mean: f64,
    median: f64,
    variance: f64,
    std_dev: f64,
}