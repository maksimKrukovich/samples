use anchor_lang::error_code;

#[error_code]
pub enum ContractErrors {
    #[msg("Investments are currently closed")]
    InvestmentClosed,
    #[msg("Invalid status for this operation")]
    InvalidStatus,
    #[msg("Invalid signature")]
    InvalidSigner,
    #[msg("Cannot flip to an invalid period")]
    InvalidPeriodFlip,
    #[msg("Invalid claiming period")]
    InvalidPeriod,
    #[msg("User has already spent his solana")]
    AlreadySpent,
    #[msg("Claim period already started")]
    ClaimPeriodStarted,
    #[msg("Start period is not already expired")]
    StartPeriodNotExpired,
    #[msg("Start period is already expired")]
    StartPeriodExpired,
    #[msg("Vesting period is already expired")]
    VestingPeriodExpired,
    #[msg("Source token account contains less tokens than provided as an amount")]
    InsufficientTokens,
    #[msg("Source associated token account is invalid")]
    InvalidSource,
    #[msg("Destination associated token account is invalid")]
    InvalidDestination,
    #[msg("Mint is not defined")]
    NoMint,
    #[msg("Invalid authority")]
    InvalidAuthority,
    #[msg("Full amount is not collected yet")]
    FullAmountNotCollected,
    #[msg("Invalid vesting periods passed")]
    InvalidVestingPeriods,
    #[msg("Invalid distribution amounts passed")]
    InvalidDistributionAmounts,
    #[msg("User has not invested solana before")]
    NoInvestment,
    #[msg("Invalid timestamp passed")]
    InvalidTimestamp,
    #[msg("Invalid vesting period expiration timestamp passed")]
    InvalidVestingExpirationTimestamp,
    #[msg("User has already claimed tokens during current period")]
    AlreadyClaimed,
    #[msg("Not enough lamports for registration")]
    NotEnoughToRegister,
    #[msg("Not enough lamports for operation")]
    NotEnoughLamports,
    #[msg("Not enough lamports on the contract for operation")]
    NotEnoughLamportsOnContract,
    #[msg("Solana input amount is too small")]
    TooSmallInputAmount,
    #[msg("Nothing to claim")]
    NothingToClaim,
}
