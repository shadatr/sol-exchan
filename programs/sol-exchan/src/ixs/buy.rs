use crate::consts::GLOBAL_STATE_SEED;
use crate::consts::TOKEN_DECIMALS;
use crate::errors::NoRugErrors;
use crate::state::{GlobalState, NewTrade};
use anchor_lang::prelude::*;
use anchor_lang::system_program;
use anchor_spl::token::{mint_to,  Mint, MintTo, Token, TokenAccount};

pub fn handle_buy(ctx: Context<Buy>, tokens_to_buy: u64) -> Result<()> {
    let global_state = &ctx.accounts.global_state;
    
    // enforcing a minimum buy amount.
    if tokens_to_buy < 10u64 * 10u64.pow(TOKEN_DECIMALS) {
        return err!(NoRugErrors::InvalidTokenBuyAmount);
    }


    let total_buy_cost = 2;
    let fees = total_buy_cost as u128 * global_state.fees_basis_points as u128 / 10000;


    mint_to(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            MintTo {
                authority: ctx.accounts.signer.to_account_info(),
                to: ctx.accounts.user_token_account.to_account_info(),
                mint: ctx.accounts.mint.to_account_info(),
            },
        ),
        tokens_to_buy,
    )?;

    
    // transfer the cost to the bonding curve.
    system_program::transfer(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            system_program::Transfer {
                from: ctx.accounts.signer.to_account_info(),
                to: ctx.accounts.fees_wallet.to_account_info(),
            },
        ),
        total_buy_cost+fees as u64,
    )?;

    
    emit!(NewTrade {
        token: ctx.accounts.user_token_account.mint,
        sol_spent: total_buy_cost,
        tokens_bought: tokens_to_buy,
        buyer: *ctx.accounts.signer.to_account_info().key,
        timestamp: Clock::get()?.unix_timestamp as u64,
    });
    Ok(())
}

#[derive(Accounts)]
pub struct Buy<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(mut,seeds = [GLOBAL_STATE_SEED], bump)]
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