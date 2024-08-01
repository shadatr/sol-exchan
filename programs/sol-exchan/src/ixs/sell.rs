use crate::consts::{GLOBAL_STATE_SEED, TOKEN_DECIMALS};
use crate::errors::NoRugErrors;
use crate::state::{GlobalState, NewTrade};
use anchor_lang::prelude::*;
use anchor_spl::token::{burn, Burn, Mint, Token, TokenAccount};
use anchor_lang::system_program;

pub fn handle_sell(ctx: Context<Sell>, tokens_to_sell: u64) -> Result<()> {
    let global_state = &ctx.accounts.global_state;
    
    // Enforcing a minimum sell amount.
    if tokens_to_sell < 10u64 * 10u64.pow(TOKEN_DECIMALS) {
        return err!(NoRugErrors::InvalidTokenSellAmount);
    }

    // Calculate the value of the tokens to sell in SOL.
    let total_sell_value = 2; // For example, 2 SOL per token
    let fees = total_sell_value as u128 * global_state.fees_basis_points as u128 / 10000;

    // Burn the tokens from the user's account.
    burn(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            Burn {
                mint: ctx.accounts.mint.to_account_info(),
                from: ctx.accounts.user_token_account.to_account_info(),
                authority: ctx.accounts.signer.to_account_info(),
            },
        ),
        tokens_to_sell,
    )?;

    // Transfer the sell value minus fees to the user.
    system_program::transfer(
        CpiContext::new(
            ctx.accounts.system_program.to_account_info(),
            system_program::Transfer {
                from: ctx.accounts.fees_wallet.to_account_info(),
                to: ctx.accounts.signer.to_account_info(),
            },
        ),
        total_sell_value as u64 - fees as u64,
    )?;

    // Emit a trade event.
    emit!(NewTrade {
        token: ctx.accounts.user_token_account.mint,
        sol_spent: total_sell_value,
        tokens_bought: tokens_to_sell,
        buyer: *ctx.accounts.signer.to_account_info().key,
        timestamp: Clock::get()?.unix_timestamp as u64,
    });
    Ok(())
}

#[derive(Accounts)]
pub struct Sell<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(mut, seeds = [GLOBAL_STATE_SEED], bump)]
    pub global_state: Account<'info, GlobalState>,
    #[account(mut)]
    pub mint : Account<'info, Mint>,
    #[account(mut, token::mint = mint, token::authority = signer)]
    pub user_token_account: Account<'info, TokenAccount>,
    #[account(mut, constraint = global_state.fees_wallet == fees_wallet.key() @ NoRugErrors::InvalidFeesWallet)]
    pub fees_wallet: SystemAccount<'info>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}
