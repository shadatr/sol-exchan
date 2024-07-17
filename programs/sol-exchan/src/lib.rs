
pub mod create_token;

use anchor_lang::prelude::*;
use anchor_lang::solana_program::entrypoint::ProgramResult;
use anchor_spl::token::{self, Token, TokenAccount, Transfer};

declare_id!("3ZyRNToaabrVgjLU9Ti8dzgk16Q7x2nMwJshLkXF2161");

#[program]
pub mod sol_exchan {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        // Your initialization logic here
        Ok(())
    }

    pub fn swap_tokens(ctx: Context<SwapTokens>,amount: u64) -> ProgramResult {
        // Token approval
        let cpi_accounts = Transfer {
            from: ctx.accounts.token_account.to_account_info(),
            to: ctx.accounts.buyer_wallet.to_account_info(),
            authority: ctx.accounts.authority.to_account_info(),
        };

        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        token::transfer(cpi_ctx, amount)?;


        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}

#[derive(Accounts)]
pub struct SwapTokens<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(mut)]
    pub token_account:  Account<'info, TokenAccount>,
    #[account(mut)]
    pub buyer_wallet: Account<'info, TokenAccount>,
    #[account(address = token::ID)]
    pub token_program: Program<'info,Token>,
}