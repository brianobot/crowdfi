use anchor_lang::prelude::*;

pub mod state;
pub mod contexts;
pub mod errors;
pub mod constant;

pub use constant::*;
pub use contexts::*;
pub use state::*;


declare_id!("5msKK8UEpEJLyT8df2rQH76v9UzTdRkfrZ1cgHNE7647");

#[program]
pub mod crowdfi {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, seed: u8, max_amount: u64, max_duration: u64) -> Result<()> {
        ctx.accounts.init(seed, max_amount, max_duration, &ctx.bumps)?;
        Ok(())
    }

    pub fn create_campaign(ctx: Context<CreateCampaign>, title: String, description: String, url: String, target_amount: u64, start_timestamp: u64, end_timestamp: u64) -> Result<()> {
        ctx.accounts.init(title, description, url, target_amount, start_timestamp, end_timestamp, &ctx.bumps)?;
        Ok(())
    }

    pub fn update_campaign(ctx: Context<UpdateCampaign>, description: Option<String>, url: Option<String>) -> Result<()> {
        ctx.accounts.update(description, url)?;
        Ok(())
    }
}
