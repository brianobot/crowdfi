use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace, Debug)]
pub struct Campaign {
    pub authority: Pubkey, // the admin behind the config
    #[max_len(256)]
    pub title: String,
    #[max_len(500)]
    pub description: String,
    #[max_len(256)]
    pub url: String,
    pub start_timestamp: u64, // the max duration campaigns under this config should run
    pub end_timestamp: u64, // the max duration campaigns under this config should run
    pub target_amount: u64, // the max amount campaingns under this config should process
    pub current_amount: u64, // the max amount campaingns under this config should process
    pub bump: u8,
}

impl Space for Campaign {
    // Add all the fields but allow the string fields to calculated in the account struct
    // for efficient use of space for each account
    const INIT_SPACE: usize = 32 + 8 + 8 + 8 + 8 + 1;
}