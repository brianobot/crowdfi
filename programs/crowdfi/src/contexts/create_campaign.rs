use anchor_lang::prelude::*;
// use anchor_spl::token::TokenAccount;

use crate::state::{Campaign, Config};
use crate::constant::ANCHOR_DISCRIMINATION;


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