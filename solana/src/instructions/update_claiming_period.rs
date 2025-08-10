use anchor_lang::prelude::*;

use crate::{
    types::{Status, SMAC},
    ContractErrors,
};

#[derive(Accounts)]
pub struct UpdateClaimingPeriod<'info> {
    #[account(mut)]
    pub contract: Account<'info, SMAC>,
    pub signer: Signer<'info>,
}

pub fn update_claiming_period(ctx: Context<UpdateClaimingPeriod>, update_value: u8) -> Result<()> {
    require_keys_eq!(
        ctx.accounts.signer.key(),
        ctx.accounts.contract.milestone_tracker_pubkey,
        ContractErrors::InvalidSigner
    );
    require!(
        ctx.accounts.contract.status == Status::Live,
        ContractErrors::InvalidStatus
    );
    require!(
        ctx.accounts.contract.vesting_period_exp_time >= Clock::get()?.unix_timestamp as u64,
        ContractErrors::VestingPeriodExpired
    );
    require!(
        update_value <= ctx.accounts.contract.vesting_periods.len() as u8,
        ContractErrors::InvalidPeriodFlip
    );

    ctx.accounts.contract.current_period = update_value;

    Ok(())
}
