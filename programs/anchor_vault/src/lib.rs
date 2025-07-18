use anchor_lang::prelude::*;

declare_id!("FGxVc2HAfo2bDARNMtDRzKwCbRCT8XpvBiYUYqPjLhqt");

#[program]
pub mod anchor_vault {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
