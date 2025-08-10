use anchor_lang::{account, prelude::*, AnchorDeserialize, AnchorSerialize};

pub const LAMPORT_RENT_BUFFER: u64 = 1_000_000;

pub type Record = (Pubkey, u64, u8);

#[account]
pub struct SMAC {
    pub smac_id: String,
    pub bump: u8,
    // authorities {
    pub owner_pubkey: Pubkey,
    pub milestone_tracker_pubkey: Pubkey,
    pub token_acquisition_pubkey: Pubkey,
    pub liquidity_boost_pool_pubkey: Pubkey,
    pub token_fund_pubkey: Pubkey,
    pub platform_fees_pubkey: Pubkey,
    // }
    pub distribution_amounts: [u64; 4],
    pub ata: Pubkey,
    pub max_amount: u64,
    pub total_invested: u64,
    pub status: Status,

    pub token_mint: Option<Pubkey>,
    pub token_amount: u64,
    pub vesting_periods: Vec<u64>,
    pub current_period: u8,
    pub start_exp_time: u64,
    pub vesting_period_exp_time: u64,
    pub registered_investors: Vec<Record>,
    pub is_expired: bool,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub enum Status {
    Created,
    LfCto,
    Live,
    Closed,
}
