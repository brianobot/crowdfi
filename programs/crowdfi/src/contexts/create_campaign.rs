use anchor_lang::prelude::*;
// use anchor_spl::token::TokenAccount;

use crate::state::{Campaign, Config};
use crate::constant::{
    ANCHOR_DISCRIMINATION,
    MAX_CAMPAIGN_TITLE,
    MAX_CAMPAIGN_DESCR,
    MAX_CAMPAIGN_URL,
};
use crate::errors::CrowdfiError;


#[derive(Accounts)]
#[instruction(title: String, description: String, url: String)]
pub struct CreateCampaign<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(
        seeds = [b"config", config.admin.as_ref(), config.seed.to_le_bytes().as_ref()],
        bump = config.bump
    )]
    pub config: Account<'info, Config>,
    #[account(
        init,
        payer = user,
        // this way each account takes just the exact amount of space needed to store it data
        space = ANCHOR_DISCRIMINATION + Campaign::INIT_SPACE + title.len() + description.len() + url.len(),
        seeds = [b"campaign", user.key().as_ref(), title.as_bytes()],
        bump,
    )]
    pub campaign: Account<'info, Campaign>,
    // #[account(
    //     init,
    //     payer = user,
    //     // TODO: how to access the key value of pda
    //     seeds = [b"campaign_vault", campaign.address.as_ref()],
    //     bump,
    //     // associated_token::mint 
    // )]
    // pub campaign_vault: Account<'info, TokenAccount>,

    pub system_program: Program<'info, System>,
}

impl<'info> CreateCampaign<'info> {
    pub fn init(&mut self, title: String, description: String, url: String, target_amount: u64, start_timestamp: u64, end_timestamp: u64, bumps: &CreateCampaignBumps) -> Result<()> {
        require!(url.len() <= MAX_CAMPAIGN_URL, CrowdfiError::CAMPAIGNURLTOOLONG);
        require!(title.len() <= MAX_CAMPAIGN_TITLE, CrowdfiError::CAMPAIGNTITLETOOLONG);
        require!(description.len() <= MAX_CAMPAIGN_DESCR, CrowdfiError::CAMPAIGNDESCRTOOLONG);

        let config = &mut self.config;
        // this is allowed because the config is not reassignable but the data it references can be changed through it

        let campaign_duration = end_timestamp - start_timestamp;
        // add check that the start date is not in the past
        // add check that the end date is greater than the start date
        require!(target_amount <= config.max_amount, CrowdfiError::CAMPAIGNMAXAMOUNTEXCEEDED);
        require!(campaign_duration <= config.max_duration, CrowdfiError::CAMPAIGNDURATIONTOOLONG);

        self.campaign.set_inner( Campaign {
            authority: self.user.key(),
            title,
            description,
            url,
            start_timestamp,
            end_timestamp,
            target_amount,
            current_amount: 0,
            bump: bumps.campaign,
        });

        Ok(())
    }
}