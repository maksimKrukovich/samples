use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount};

use crate::{
    types::{Status, SMAC},
    util::transfer_tokens,
    ContractErrors,
};

#[derive(Accounts)]
pub struct SetToken<'info> {
    #[account(mut)]
    pub contract: Account<'info, SMAC>,
    #[account(
        mut,
        constraint = contract.owner_pubkey == signer.key(),
    )]
    pub signer: Signer<'info>,
    #[account(mut)]
    pub source_ata_owner: Signer<'info>,
    #[account(mut)]
    pub source_ata: Account<'info, TokenAccount>,
    #[account(
        init,
        payer = signer,
        seeds = [b"smac", mint.key().as_ref(), contract.key().as_ref()],
        bump,
        token::mint = mint,
        token::authority = contract,
    )]
    pub contract_ata: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
    pub mint: Account<'info, Mint>,
}

pub fn set_token(ctx: Context<SetToken>, amount: u64) -> Result<()> {
    require!(
        ctx.accounts.contract.status == Status::LfCto,
        ContractErrors::InvalidStatus
    );
    require!(
        ctx.accounts.source_ata.amount >= amount,
        ContractErrors::InsufficientTokens
    );
    require_keys_eq!(
        // NOTE: this is not the same accounts, look closer
        ctx.accounts.source_ata.owner.key(),
        ctx.accounts.source_ata_owner.key(),
        ContractErrors::InvalidSigner,
    );

    let contract = &mut ctx.accounts.contract;

    let size = contract.to_account_info().data_len() + std::mem::size_of::<Pubkey>();

    contract.to_account_info().realloc(size, false)?;

    contract.ata = ctx.accounts.contract_ata.key();
    contract.token_mint = Some(ctx.accounts.mint.key());

    let bump_vector = contract.bump.to_le_bytes();

    transfer_tokens(
        &ctx.accounts.contract_ata,
        &ctx.accounts.source_ata,
        &ctx.accounts.token_program,
        &ctx.accounts.source_ata_owner,
        amount,
        &[&[
            b"smac",
            contract.owner_pubkey.key().as_ref(),
            contract.smac_id.as_bytes().as_ref(),
            bump_vector.as_ref(),
        ]],
    )?;

    contract.token_amount = amount;

    contract.status = Status::Live;
    Ok(())
}
