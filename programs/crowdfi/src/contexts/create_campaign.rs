use anchor_lang::prelude::*;
// use anchor_spl::token_interface::{Mint, TokenInterface};
// use anchor_spl::metadata::{MetadataAccount, Metadata};

use crate::constant::{
    ANCHOR_DISCRIMINATOR, 
    MAX_CAMPAIGN_TITLE, 
    MAX_CAMPAIGN_DESCR, 
    MAX_CAMPAIGN_URL,
};
use crate::errors::CrowdfiError;
use crate::state::{Campaign, Config};


#[derive(Accounts)]
#[instruction(title: String, description: String, url: String)]
pub struct CreateCampaign<'info> {
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
        init,
        payer = user,
        // this way each account takes just the exact amount of space needed to store it data
        space = ANCHOR_DISCRIMINATOR + Campaign::INIT_SPACE + title.len() + description.len() + url.len(),
        seeds = [b"campaign", user.key().as_ref(), title.as_bytes()],
        bump,
    )]
    pub campaign: Account<'info, Campaign>,
    #[account(
        init,
        payer = user,
        space = ANCHOR_DISCRIMINATOR,
        seeds = [b"campaign_vault", campaign.key().as_ref()],
        bump,
    )]
    /// CHECK: THis is safe
    pub campaign_vault: AccountInfo<'info>,
    // Create a Token that would represent Participation in this Campaign
    // #[account(
    //     init,
    //     payer = user,
    //     mint::decimals = 6,
    //     mint::authority = config,
    //     seeds = [b"reward_mint", campaign.key().as_ref()],
    //     bump,
    // )]
    // pub reward_mint: InterfaceAccount<'info, Mint>,
    // #[account(
    //     seeds = [
    //         b"metadata",
    //         metadata_program.key().as_ref(),
    //         reward_mint.key().as_ref(),
    //     ],
    //     seeds::program = metadata_program.key(),
    //     bump,
    // )]
    // pub reward_mint_metadata: Account<'info, MetadataAccount>,
    pub system_program: Program<'info, System>,
    // pub metadata_program: Program<'info, Metadata>,
    // pub token_program: Interface<'info, TokenInterface>,
}

impl<'info> CreateCampaign<'info> {
    // Initialize the campaign
    // Initialize the campaign reward mint
    // Initialize the campaign reward mint Metadata

    pub fn init(
        &mut self,
        title: String,
        description: String,
        url: String,
        target_amount: u64,
        start_timestamp: u64,
        end_timestamp: u64,
        bumps: &CreateCampaignBumps,
    ) -> Result<()> {
        require!(
            url.len() <= MAX_CAMPAIGN_URL,
            CrowdfiError::CAMPAIGNURLTOOLONG
        );
        require!(
            title.len() <= MAX_CAMPAIGN_TITLE,
            CrowdfiError::CAMPAIGNTITLETOOLONG
        );
        require!(
            description.len() <= MAX_CAMPAIGN_DESCR,
            CrowdfiError::CAMPAIGNDESCRTOOLONG
        );

        let config = &mut self.config;
        // this is allowed because the config is not reassignable but the data it references can be changed through it

        let campaign_duration = end_timestamp - start_timestamp;
        // add check that the start date is not in the past
        // add check that the end date is greater than the start date
        require!(
            target_amount <= config.max_amount,
            CrowdfiError::CAMPAIGNMAXAMOUNTEXCEEDED
        );
        require!(
            campaign_duration <= config.max_duration,
            CrowdfiError::CAMPAIGNDURATIONTOOLONG
        );

        // INITIALIZE THE CAMPAIGN ACCOUNT
        self.campaign.set_inner(Campaign {
            authority: self.user.key(),
            title,
            description,
            url,
            start_timestamp,
            end_timestamp,
            target_amount,
            current_amount: 0,
            bump: bumps.campaign,
            vault_bump: bumps.campaign_vault,
            // TODO: Change this to the reward mint bump
            reward_mint_bump: bumps.campaign_vault,
        });

        // INITIALIZE THE CAMPAING VAULT ACCOUNT

        Ok(())
    }
}
