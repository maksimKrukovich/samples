use anchor_lang::prelude::*;

use crate::{
    types::{Record, Status, SMAC},
    util::transfer_lamports,
    ContractErrors,
};

#[derive(Accounts)]
pub struct RegisterInvestor<'info> {
    #[account(mut)]
    pub contract: Account<'info, SMAC>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

pub fn register_investor(ctx: Context<RegisterInvestor>, amount: u64) -> Result<()> {
    // min 0.1 SOL
    require!(amount >= 100_000_000, ContractErrors::TooSmallInputAmount);
    require!(
        ctx.accounts.contract.status == Status::Created,
        ContractErrors::InvestmentClosed
    );
    require!(
        ctx.accounts.contract.start_exp_time > Clock::get()?.unix_timestamp as u64,
        ContractErrors::StartPeriodExpired
    );

    let contract = &mut ctx.accounts.contract;

    let user_idx: Option<usize> = contract
        .registered_investors
        .iter()
        .position(|(k, _, _)| *k == ctx.accounts.signer.key());

    match user_idx {
        Some(idx) => {
            let lamports_res = transfer_lamports(
                &ctx.accounts.signer.to_account_info(),
                &contract.to_account_info(),
                &ctx.accounts.system_program,
                amount,
            );

            if let Err(err) = lamports_res {
                return Err(err.into());
            }

            contract.total_invested += amount;
            contract.registered_investors[idx].1 += amount;
        }
        None => {
            let contract_len = contract.to_account_info().data_len();
            let current_cost = Rent::get()?.minimum_balance(contract_len);
            let new_cost =
                Rent::get()?.minimum_balance(contract_len + std::mem::size_of::<Record>());
            let realloc_cost = new_cost - current_cost;

            require!(
                **ctx.accounts.signer.lamports.borrow() >= amount + realloc_cost,
                ContractErrors::NotEnoughLamports
            );

            let new_size = contract.to_account_info().data_len() + std::mem::size_of::<Pubkey>();
            contract.to_account_info().realloc(new_size, false)?;

            let lamports_res = transfer_lamports(
                &ctx.accounts.signer.to_account_info(),
                &contract.to_account_info(),
                &ctx.accounts.system_program,
                amount + realloc_cost,
            );

            if let Err(err) = lamports_res {
                return Err(err.into());
            }

            contract.total_invested += amount;
            contract
                .registered_investors
                .push((ctx.accounts.signer.key(), amount, 0));
        }
    }

    Ok(())
}
