use anchor_lang::prelude::*;


#[derive(Accounts)]
pub struct CreateCampaignV2<'info> {
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

impl<'info> CreateCampaignV2<'info> {
    pub fn init_campaign(&mut self) -> Result<()> {
        Ok(())
    }
}