use anchor_lang::prelude::*;

pub mod state;
pub mod contexts;
pub mod errors;

declare_id!("5msKK8UEpEJLyT8df2rQH76v9UzTdRkfrZ1cgHNE7647");

#[program]
pub mod crowdfi {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
