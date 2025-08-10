use anchor_lang::prelude::*;

use crate::{
    types::{Status, SMAC},
    ContractErrors,
};

#[derive(Accounts)]
#[instruction(smac_id: String)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
    #[account(
        init,
        payer = signer,
        space = std::mem::size_of::<SMAC>() + 8,
        seeds = [
            b"smac",
            signer.key().as_ref(),
            smac_id.as_bytes().as_ref()
        ],
        bump
    )]
    pub contract: Account<'info, SMAC>,
}

pub fn initialize(
    ctx: Context<Initialize>,
    _smac_id: String,
    milestone_tracker_pubkey: Pubkey,
    distribution_authorities: [Pubkey; 4],
    distribution_amounts: [u64; 4],
    max_amount: u64,
    vesting_periods: Vec<u64>,
    register_period_exp_time: u64,
    vesting_period_exp_time: u64,
) -> Result<()> {
    require!(
        vesting_periods.iter().sum::<u64>() == 100,
        ContractErrors::InvalidVestingPeriods
    );
    require!(
        vesting_periods.iter().all(|&x| x != 0),
        ContractErrors::InvalidVestingPeriods
    );
    require!(
        distribution_amounts.iter().sum::<u64>() == 100,
        ContractErrors::InvalidDistributionAmounts
    );
    require!(
        distribution_amounts.iter().all(|&x| x != 0),
        ContractErrors::InvalidDistributionAmounts
    );
    require!(
        register_period_exp_time > 0,
        ContractErrors::InvalidTimestamp
    );
    require!(
        vesting_period_exp_time > 0,
        ContractErrors::InvalidVestingExpirationTimestamp
    );
    require!(
        vesting_period_exp_time > register_period_exp_time,
        ContractErrors::InvalidVestingExpirationTimestamp
    );

    let contract = &mut ctx.accounts.contract;
    contract.bump = ctx.bumps.contract;

    contract.owner_pubkey = ctx.accounts.signer.key();
    contract.milestone_tracker_pubkey = milestone_tracker_pubkey;
    contract.token_acquisition_pubkey = distribution_authorities[0];
    contract.liquidity_boost_pool_pubkey = distribution_authorities[1];
    contract.token_fund_pubkey = distribution_authorities[2];
    contract.platform_fees_pubkey = distribution_authorities[3];

    contract.distribution_amounts = distribution_amounts;
    contract.max_amount = max_amount;
    contract.total_invested = 0;
    contract.vesting_periods = vesting_periods;
    contract.current_period = 0;
    contract.status = Status::Created;
    contract.is_expired = false;

    contract.registered_investors = Vec::new();

    contract.smac_id = _smac_id;

    let clock = Clock::get()?;
    contract.start_exp_time = (clock.unix_timestamp as u64)
        .checked_add(register_period_exp_time)
        .expect("IntegerAddOverflow when calculating start expiration time!");

    contract.vesting_period_exp_time = (clock.unix_timestamp as u64)
        .checked_add(vesting_period_exp_time)
        .expect("IntegerAddOverflow when calculating vesting period expiration!");

    Ok(())
}
