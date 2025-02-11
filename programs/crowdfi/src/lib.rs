use anchor_lang::prelude::*;

pub mod state;
pub mod contexts;
pub mod errors;

pub use contexts::*;


declare_id!("5msKK8UEpEJLyT8df2rQH76v9UzTdRkfrZ1cgHNE7647");

#[program]
pub mod crowdfi {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, max_amount: u64, max_duration: u64) -> Result<()> {
        ctx.accounts.init(max_amount, max_duration, &ctx.bumps)?;
        Ok(())
    }
}
