pub mod ixs;
pub mod state;
pub mod consts;
use anchor_lang::program;
use anchor_lang::prelude::*;
use consts::GLOBAL_STATE_SEED;
use ixs::*;
use state::GlobalState;

declare_id!("8gm6UpJoDAawsGYrtGXhatokbFQDVZeBkUP1j8egxKTp");

#[program]
pub mod sol_exchan {

    use super::*; 

    pub fn initialize(ctx: Context<Initialize>, fees: u16, lockup_duration: u64) -> Result<()> {
        let global_state = &mut ctx.accounts.global_state;
        global_state.admin = *ctx.accounts.signer.key;
        global_state.fees_basis_points = fees;
        global_state.lockup_duration = lockup_duration;
        global_state.fees_wallet = *ctx.accounts.fees_wallet.to_account_info().key;
        Ok(())
    }

    pub fn create_token(ctx: Context<CreateToken>, name: String, symbol: String, uri: String) -> Result<()> {
        handle_create_token(ctx, name, symbol, uri)
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(init, payer = signer, space = std::mem::size_of::< GlobalState > () + 8, seeds = [GLOBAL_STATE_SEED], bump)]
    pub global_state: Account<'info, GlobalState>,
    
    pub fees_wallet : SystemAccount<'info>,

    pub system_program: Program<'info, System>,
}