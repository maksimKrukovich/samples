pub mod error;
pub mod instructions;
pub mod types;
pub mod util;

use error::*;
use instructions::*;

use anchor_lang::prelude::*;

//localnet
declare_id!("3NyRcnPCuJ5maUeA8CXyaAPHpgX9mMLfGYLDzuqyePBd");

#[cfg(not(feature = "no-entrypoint"))]
solana_security_txt::security_txt! {
    name: "",
    project_url: "",
    contacts: "",
    policy: "https://github.com/solana-labs/solana/blob/master/SECURITY.md"
}

#[program]
pub mod smac {
    use super::*;

    /// # Initialize
    /// ## Arguments
    ///
    /// * `ctx`- The accounts needed by instruction.
    /// * `_smac_id` - The ID of SMAC (could be any string)
    /// * `milestone_tracker_pubkey` - the milestone tracker wallet pubkey
    /// * `distribution_authorities` - The array of distribution authorities pubkeys
    /// * `distribution_amounts` - The array of amounts that will be used to distribute lamports between authorities from previous parameter
    /// * `max_amount` - A minimal amount of lamports to start smac after expiration of the start period
    /// * `vesting_periods` - The array of shares how many percent of tokens will be distributed in each period
    /// * `register_period_exp_time` - An expiration time of the registration period in seconds
    /// * `vesting_period_exp_time` - An expiration time of the vesting period in seconds (should be grater than register_period_exp_time)
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
        return instructions::initialize(
            ctx,
            _smac_id,
            milestone_tracker_pubkey,
            distribution_authorities,
            distribution_amounts,
            max_amount,
            vesting_periods,
            register_period_exp_time,
            vesting_period_exp_time,
        );
    }

    /// # Register investor
    /// The function to register new investor
    /// ## Arguments
    ///
    /// * `ctx`- The accounts needed by instruction.
    /// * `amount` - amount of lamports to be transferred from signer to contract
    pub fn register_investor(ctx: Context<RegisterInvestor>, amount: u64) -> Result<()> {
        return instructions::register_investor(ctx, amount);
    }

    /// # Claim back
    /// The function to claim back lamports
    /// ## Arguments
    ///
    /// * `ctx`- The accounts needed by instruction.
    pub fn claim_back(ctx: Context<ClaimBack>) -> Result<()> {
        return instructions::claim_back(ctx);
    }

    /// # Set token
    /// The function to set token and start live period
    /// # Arguments
    ///
    /// * `ctx`- The accounts needed by instruction.
    /// * `amount` - amount of meme tokens to be transferred from owner to contract
    pub fn set_token(ctx: Context<SetToken>, amount: u64) -> Result<()> {
        return instructions::set_token(ctx, amount);
    }

    /// # Update claiming period
    /// The function to update milestone back or forward
    /// # Arguments
    ///
    /// * `ctx`- The accounts needed by instruction.
    /// * `update_value` - New period (>= 0 && <= vesting periods count)
    pub fn update_claiming_period(
        ctx: Context<UpdateClaimingPeriod>,
        update_value: u8,
    ) -> Result<()> {
        return instructions::update_claiming_period(ctx, update_value);
    }

    /// # Claim
    /// The function to claim meme token in live period. Will transfer all tokens that are available for you at the moment of the call
    /// ## Arguments
    ///
    /// * `ctx`- The accounts needed by instruction.
    pub fn claim(ctx: Context<Claim>) -> Result<()> {
        return instructions::claim(ctx);
    }

    /// # Distribute lamports
    /// The function to distribute lamports between distribution authorities
    /// ## Arguments
    ///
    /// * `ctx`- The accounts needed by instruction.
    pub fn distribute_lamports(ctx: Context<DistributeLamports>) -> Result<()> {
        return instructions::distribute_lamports(ctx);
    }
}

#[cfg(test)]
mod test {
    fn check(value: &String) -> bool {
        value.chars().enumerate().all(|(i, val)| (value.chars().nth(value.len() - 1 - i).unwrap().to_lowercase().eq(val.to_lowercase())))
    } 

    #[test]
    fn test() {
        let line1 = String::from("aAo aoA");
        let line2 = String::from("aOa A oa");
        
        assert_eq!(check(&line1), false);
        assert_eq!(check(&line2), true);
    } 
}