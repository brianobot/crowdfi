use anchor_lang::prelude::*;
use anchor_lang::system_program::transfer;
use anchor_lang::system_program::Transfer;

use crate::state::{Campaign, Config};
use crate::errors::CrowdfiError;

#[derive(Accounts)]
pub struct Refund<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(
        seeds = [
            b"config", 
            config.admin.as_ref(), 
            config.seed.to_le_bytes().as_ref()
        ],
        bump = config.bump
    )]
    pub config: Account<'info, Config>,
    #[account(
        seeds = [b"campaign", campaign.authority.as_ref(), campaign.title.as_bytes()],
        bump = campaign.bump,
    )]
    pub campaign: Account<'info, Campaign>,
    #[account(
        mut,
        seeds = [b"campaign_vault", campaign.key().as_ref()],
        bump = campaign.vault_bump,
    )]
    /// CHECK: THis is safe
    pub campaign_vault: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}

impl<'info> Refund<'info> {
    pub fn withdraw_from_vault(&mut self, amount: u64) -> Result<()> {
        require!(amount > 0, CrowdfiError::AMOUNTISZERO);
        
        let cpi_program = self.system_program.to_account_info();

        let cpi_accounts = Transfer {
            from: self.campaign_vault.to_account_info(),
            to: self.user.to_account_info(),
        };

        let seeds = [
            b"campaign_vault",
            self.campaign.to_account_info().key.as_ref(),
            &[self.campaign.vault_bump],
        ];

        let signer_seeds = &[&seeds[..]];

        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);

        transfer(cpi_ctx, amount)?;
        Ok(())
    }

    pub fn burn_reward_mint(&mut self, _amount: u64) -> Result<()> {
        
        Ok(())
    }
}