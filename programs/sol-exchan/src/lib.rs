use anchor_lang::prelude::*;

declare_id!("5umccz9MSnMxV2UDbQWEoxarCk7UFaipFkcgzW5ggKs2");

#[program]
pub mod sol_exchan {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
