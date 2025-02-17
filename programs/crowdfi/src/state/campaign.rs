use anchor_lang::prelude::*;

#[account]
#[derive(Debug)]
pub struct Campaign {
    pub authority: Pubkey, // the admin behind the config
    pub title: String,
    pub description: String,
    pub url: String,
    pub start_timestamp: u64, // the max duration campaigns under this config should run
    pub end_timestamp: u64, // the max duration campaigns under this config should run
    pub target_amount: u64, // the max amount campaingns under this config should process
    pub current_amount: u64, // the max amount campaingns under this config should process
    pub bump: u8,
    pub vault_bump: u8, // the bump of the vault that is linked to the campaign
    pub reward_mint_bump: u8, // the bump of the reward mint for the campaign
}

impl Space for Campaign {
    // Add all the fields but allow the string fields to calculated in the account struct
    // for efficient use of space for each account
    const INIT_SPACE: usize = 32 + 8 + 8 + 8 + 8 + 1 + 1 + 1;
}