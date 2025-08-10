use anchor_lang::{
    prelude::{Account, AccountInfo, CpiContext, Program, Result, System},
    system_program::Transfer,
    ToAccountInfo,
};
use anchor_spl::token::{self, Transfer as SplTransfer};
use anchor_spl::token::{Token, TokenAccount};

pub fn transfer_tokens<'a>(
    destination: &Account<'a, TokenAccount>,
    source: &Account<'a, TokenAccount>,
    token_program: &Program<'a, Token>,
    transfer_authority: &AccountInfo<'a>,
    amount: u64,
    seeds: &[&[&[u8]]],
) -> Result<()> {
    let cpi_context = CpiContext::new_with_signer(
        token_program.to_account_info(),
        SplTransfer {
            from: source.to_account_info(),
            to: destination.to_account_info(),
            authority: transfer_authority.to_account_info(),
        },
        &seeds,
    );

    token::transfer(cpi_context, amount)?;
    Ok(())
}

pub fn transfer_lamports<'a>(
    source: &AccountInfo<'a>,
    destination: &AccountInfo<'a>,
    system_program: &Program<'a, System>,
    amount: u64,
) -> Result<()> {
    let cpi_context = CpiContext::new(
        system_program.to_account_info(),
        Transfer {
            from: source.clone(),
            to: destination.clone(),
        },
    );

    anchor_lang::system_program::transfer(cpi_context, amount)?;
    Ok(())
}
