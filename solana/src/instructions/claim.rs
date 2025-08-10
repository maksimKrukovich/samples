use std::ops::Div;

use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::{get_associated_token_address, AssociatedToken},
    token::{Mint, Token, TokenAccount},
};

use crate::{
    types::{Status, SMAC},
    util::transfer_tokens,
    ContractErrors,
};

#[derive(Accounts)]
pub struct Claim<'info> {
    #[account(mut)]
    pub contract: Account<'info, SMAC>,
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(
        mut,
        seeds = [b"smac", mint.key().as_ref(), contract.key().as_ref()],
        bump,
        token::mint = contract.token_mint.unwrap(),
        token::authority = contract,
    )]
    pub from_ata: Account<'info, TokenAccount>,
    #[account(
        init_if_needed,
        payer = signer,
        associated_token::mint = mint,
        associated_token::authority = signer
    )]
    pub to_ata: Account<'info, TokenAccount>,
    pub mint: Account<'info, Mint>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}

pub fn claim(ctx: Context<Claim>) -> Result<()> {
    let contract = &mut ctx.accounts.contract;

    require!(
        contract.status == Status::Live,
        ContractErrors::InvalidStatus
    );

    let source = &ctx.accounts.from_ata;
    require_keys_eq!(
        contract.ata.key(),
        source.key(),
        ContractErrors::InvalidSource
    );

    let destination = &ctx.accounts.to_ata;
    require_keys_eq!(
        get_associated_token_address(&ctx.accounts.signer.key(), &contract.token_mint.unwrap()),
        ctx.accounts.to_ata.key(),
        ContractErrors::InvalidDestination
    );

    let user_idx: Option<usize> = contract
        .registered_investors
        .iter()
        .position(|(k, _, _)| *k == ctx.accounts.signer.key());

    require!(user_idx.is_some(), ContractErrors::NoInvestment);
    let user_idx = user_idx.unwrap();

    if !contract.is_expired
        && contract.vesting_period_exp_time < Clock::get()?.unix_timestamp as u64
    {
        contract.current_period = contract.vesting_periods.len() as u8;
        contract.is_expired = true;
    }

    require!(contract.current_period != 0, ContractErrors::NothingToClaim);

    require!(
        contract.registered_investors[user_idx].2 < contract.current_period,
        ContractErrors::AlreadyClaimed
    );

    let mut claim_part: u64 = 0;
    for i in contract.registered_investors[user_idx].2..contract.current_period {
        let part = contract.vesting_periods.get(i as usize).unwrap();
        claim_part += part;
    }
    require!(claim_part > 0, ContractErrors::AlreadyClaimed);

    let amount = u128::from(
        contract.token_amount as u128
            * contract.registered_investors[user_idx].1 as u128
            * claim_part as u128,
    )
    .div(contract.total_invested as u128 * 100u128);

    let bump_vector = contract.bump.to_le_bytes();

    let res = transfer_tokens(
        destination,
        source,
        &ctx.accounts.token_program,
        &contract.to_account_info(),
        amount as u64,
        &[&[
            b"smac",
            contract.owner_pubkey.key().as_ref(),
            contract.smac_id.as_bytes().as_ref(),
            bump_vector.as_ref(),
        ]],
    );

    if res.is_err() {
        return res;
    }

    contract.registered_investors[user_idx].2 = contract.current_period;

    Ok(())
}
