use anchor_lang::prelude::*;
// use anchor_spl::token::TokenAccount;

use crate::state::{Campaign, Config};
use crate::constant::{
    ANCHOR_DISCRIMINATION,
    MAX_CAMPAIGN_DESCR,
    MAX_CAMPAIGN_URL,
};
use crate::errors::CrowdfiError;

/*
Only the Campaign Description and Url can be updated in the update instruction, all other variables are static
for the duration (lifetime) of the campaign
*/

#[derive(Accounts)]
#[instruction(description: String, url: String)]
pub struct UpdateCampaign<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(
        seeds = [b"config", config.admin.as_ref(), config.seed.to_le_bytes().as_ref()],
        bump = config.bump
    )]
    pub config: Account<'info, Config>,
    #[account(
        mut,
        realloc = ANCHOR_DISCRIMINATION + Campaign::INIT_SPACE + campaign.title.len() + campaign.description.len() + campaign.url.len(),
        realloc::payer = user,
        realloc::zero = false,
        seeds = [b"campaign", user.key().as_ref(), campaign.title.as_bytes()],
        bump = campaign.bump,
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

impl<'info> UpdateCampaign<'info> {
    pub fn update(&mut self, description: Option<String>, url: Option<String>) -> Result<()> {
        let description = match description {
            Some(value) => value,
            None => self.campaign.description.clone(),
        };
        
        let url = match url {
            Some(value) => value,
            None => self.campaign.url.clone(),
        };

        require!(url.len() <= MAX_CAMPAIGN_URL, CrowdfiError::CAMPAIGNURLTOOLONG);
        require!(description.len() <= MAX_CAMPAIGN_DESCR, CrowdfiError::CAMPAIGNDESCRTOOLONG);

        self.campaign.description = description;
        self.campaign.url = url;
        
        Ok(())
    }
}