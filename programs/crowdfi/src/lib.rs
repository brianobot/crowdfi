use anchor_lang::prelude::*;

pub mod constant;
pub mod contexts;
pub mod errors;
pub mod state;

pub use constant::*;
pub use contexts::*;
pub use errors::*;
pub use state::*;


declare_id!("62hgGBySaNh6mTqBN5rRLMDRxJ4T1wfcZL2X3quWij6R");

#[program]
pub mod crowdfi {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, seed: u64, max_amount: u64, max_duration: u64) -> Result<()> {
        ctx.accounts.init(seed, max_amount, max_duration, &ctx.bumps)?;
        Ok(())
    }

    // pub fn create_campaign(ctx: Context<CreateCampaign>, title: String, description: String, url: String, target_amount: u64, start_timestamp: u64, end_timestamp: u64) -> Result<()> {
    //     ctx.accounts.init_campaign(title, description, url, target_amount, start_timestamp, end_timestamp, &ctx.bumps)?;
    //     Ok(())
    // }
    
    pub fn create_campaign_v2(ctx: Context<CreateCampaignV2>) -> Result<()> {
        ctx.accounts.init_campaign()?;
        Ok(())
    }

    pub fn update_campaign(ctx: Context<UpdateCampaign>, description: Option<String>, url: Option<String>) -> Result<()> {
        ctx.accounts.update(description, url)?;
        Ok(())
    }

    pub fn donate(ctx: Context<Donate>, amount: u64) -> Result<()> {
        ctx.accounts.deposit_to_vault(amount)?;
        ctx.accounts.reward_donation(amount)?;
        Ok(())
    }
    
    pub fn refund(ctx: Context<Refund>, amount: u64) -> Result<()> {
        ctx.accounts.withdraw_from_vault(amount)?;
        ctx.accounts.burn_reward_mint(amount)?;
        Ok(())
    }
}
