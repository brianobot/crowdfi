use anchor_lang::prelude::*;
use anchor_spl::token_interface::{Mint, TokenInterface};

use crate::state::Config;


#[derive(Accounts)]
#[instruction(seed: u8)]
pub struct Initialize<'info> {
    #[account(mut)] // needed because balance would be deducted from the account
    pub admin: Signer<'info>,
    #[account(
        init,
        payer = admin,
        space = 8 + Config::INIT_SPACE,
        seeds = [b"config", admin.key().as_ref(), seed.to_le_bytes().as_ref()],
        bump
    )]
    pub config: Account<'info, Config>,
    #[account(
        init,
        payer = admin,
        mint::decimals = 6,
        mint::authority = config, // since config is a pda, the address is it key, 
    )]
    // docs: https://www.anchor-lang.com/docs/tokens/basics/create-mint#account-types
    pub reward_mint: InterfaceAccount<'info, Mint>,

    pub system_program: Program<'info, System>,
    pub token_program: Interface<'info, TokenInterface>,
}


impl<'info> Initialize<'info> {
    pub fn init(&mut self, seed: u8, max_amount: u64, max_duration: u64, bumps: &InitializeBumps) -> Result<()> {
        let fee = self.calculate_fee(max_amount, max_duration);

        self.config.set_inner( Config {
            admin: self.admin.key(),
            max_duration,
            max_amount,
            fee,
            seed,
            reward_mint: self.reward_mint.key(),
            bump: bumps.config,
        });

        msg!("Config Initialized: {:?}", self.config);
        Ok(())
    }

    fn calculate_fee(&self, _max_amount: u64, _max_duration: u64) -> u16 {
        10
    }
}