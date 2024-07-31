use crate::consts::GLOBAL_STATE_SEED;
use crate::consts::{BONDING_CURVE_SEED, TOKEN_DECIMALS};
use crate::errors::NoRugErrors;
use crate::state::{BondingCurve, GlobalState, NewTrade, UserPosition};
use anchor_lang::prelude::*;
use anchor_lang::system_program;
use anchor_spl::token::{transfer, Token, TokenAccount, Transfer, Mint};

pub fn handle_buy(ctx: Context<Buy>, mut tokens_to_buy: u64) -> Result<()> {
    let bonding_curve = &mut ctx.accounts.bonding_curve;
    let global_state = &ctx.accounts.global_state;
    let user_position = &mut ctx.accounts.user_position;

    require!(!bonding_curve.lockup_period_over(global_state.lockup_duration)?, NoRugErrors::LockupPeriodNotOver);

    require!(!bonding_curve.complete, NoRugErrors::TokenLive);
    require!(bonding_curve.token_reserves > 0, NoRugErrors::InvalidTokenBuyAmount);
    
    
    // enforcing a minimum buy amount.
    if tokens_to_buy < 10u64 * 10u64.pow(TOKEN_DECIMALS) {
        return err!(NoRugErrors::InvalidTokenBuyAmount);
    }

    let tokens_left = bonding_curve.token_reserves;
    if tokens_to_buy > tokens_left {
        // if there are no enough tokens to fill in order with X amount, let them buy whatever that's left
        tokens_to_buy = tokens_left;
    }

    let total_buy_cost = bonding_curve.calculate_cost(tokens_to_buy);
    let fees = total_buy_cost as u128 * global_state.fees_basis_points as u128 / 10000;
    // transfer the token to the bonding curve.

    let seeds = &[
        BONDING_CURVE_SEED,
        ctx.accounts.user_token_account.mint.as_ref(),
        &[ctx.bumps.bonding_curve],
    ];
    let signer = &[&seeds[..]];

    transfer(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: ctx.accounts.token_custody.to_account_info(),
                to: ctx.accounts.user_token_account.to_account_info(),
                authority: bonding_curve.to_account_info(),
            },
            signer,
        ),
        tokens_to_buy,
    )?;

    
    // transfer the cost to the bonding curve.
    system_program::transfer(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            system_program::Transfer {
                from: ctx.accounts.signer.to_account_info(),
                to: bonding_curve.to_account_info(),
            },
        ),
        total_buy_cost,
    )?;

    // transfer the fees to the fees wallet.
    system_program::transfer(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            system_program::Transfer {
                from: ctx.accounts.signer.to_account_info(),
                to: ctx.accounts.fees_wallet.to_account_info(),
            },
        ),
        fees as u64,
    )?;

    bonding_curve.sol_reserves += total_buy_cost;
    bonding_curve.token_reserves -= tokens_to_buy;

    // this would check for if the account is initialized
    if user_position.token == Pubkey::default() {
        user_position.token = ctx.accounts.user_token_account.mint;
    }
    user_position.sol_spent += total_buy_cost;
    user_position.tokens_bought += tokens_to_buy;

    
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
    #[account(seeds = [GLOBAL_STATE_SEED], bump)]
    pub global_state: Account<'info, GlobalState>,
    pub mint : Account<'info, Mint>,
    #[account(mut, token::mint = mint, token::authority = signer)]
    pub user_token_account: Account<'info, TokenAccount>,
    #[account(mut, seeds = [BONDING_CURVE_SEED, user_token_account.mint.key().as_ref()], bump)]
    pub bonding_curve: Account<'info, BondingCurve>,
    #[account(mut, token::mint = mint, token::authority = bonding_curve)]
    pub token_custody: Account<'info, TokenAccount>,
    #[account(mut, constraint = global_state.fees_wallet == fees_wallet.key() @ NoRugErrors::InvalidFeesWallet)]
    pub fees_wallet: SystemAccount<'info>,
    #[account(init_if_needed, seeds=[signer.key().as_ref(), bonding_curve.key().as_ref()], bump, payer= signer, space= std::mem::size_of::<UserPosition>() + 8)]
    pub user_position: Account<'info, UserPosition>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}