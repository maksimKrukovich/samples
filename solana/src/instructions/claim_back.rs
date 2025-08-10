use anchor_lang::prelude::*;

use crate::{
    types::{Status, SMAC},
    ContractErrors,
};

#[derive(Accounts)]
pub struct ClaimBack<'info> {
    #[account(mut)]
    pub contract: Account<'info, SMAC>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

pub fn claim_back(ctx: Context<ClaimBack>) -> Result<()> {
    require!(
        ctx.accounts.contract.status == Status::Created,
        ContractErrors::InvalidPeriod
    );
    require!(
        ctx.accounts.contract.start_exp_time <= Clock::get()?.unix_timestamp as u64,
        ContractErrors::StartPeriodNotExpired
    );

    let contract = &mut ctx.accounts.contract;
    let user_idx: Option<usize> = contract
        .registered_investors
        .iter()
        .position(|(k, _, _)| *k == ctx.accounts.signer.key());

    require!(user_idx.is_some(), ContractErrors::NoInvestment);

    let user_idx = user_idx.unwrap();
    let invested_amount = contract.registered_investors[user_idx].1;

    require!(invested_amount > 0, ContractErrors::AlreadySpent);

    **contract.to_account_info().try_borrow_mut_lamports()? -= invested_amount;
    **ctx.accounts.signer.try_borrow_mut_lamports()? += invested_amount;

    contract.registered_investors[user_idx].1 = 0;

    Ok(())
}
