use crate::consts::GLOBAL_STATE_SEED;
use crate::consts::BONDING_CURVE_SEED;
use crate::errors::NoRugErrors;
use crate::state::{BondingCurve, GlobalState, NewTrade, UserPosition};
use anchor_lang::prelude::*;
use anchor_lang::system_program;
use anchor_spl::token::{transfer, Token, TokenAccount, Transfer, Mint};

pub fn handle_sell(ctx: Context<Sell>, tokens_to_sell: u64) -> Result<()> {
    let bonding_curve = &mut ctx.accounts.bonding_curve;
    let global_state = &ctx.accounts.global_state;
    let user_position = &mut ctx.accounts.user_position;

    require!(tokens_to_sell > 0, NoRugErrors::InvalidTokenSellAmount);
    require!(user_position.tokens_bought >= tokens_to_sell, NoRugErrors::InsufficientTokens);

    let total_sell_value = bonding_curve.calculate_cost(tokens_to_sell);
    let fees = total_sell_value as u128 * global_state.fees_basis_points as u128 / 10000;
    let total_payout = total_sell_value - fees as u64;
    
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
                from: ctx.accounts.user_token_account.to_account_info(),
                to: ctx.accounts.token_custody.to_account_info(),
                authority: ctx.accounts.signer.to_account_info(),
            },
            signer
        ),
        tokens_to_sell,
    )?;

    // transfer the payout to the user.
    system_program::transfer(
        CpiContext::new(
            ctx.accounts.system_program.to_account_info(),
            system_program::Transfer {
                from: bonding_curve.to_account_info(),
                to: ctx.accounts.signer.to_account_info(),
            },
        ),
        total_payout,
    )?;

    // transfer the fees to the fees wallet.
    system_program::transfer(
        CpiContext::new(
            ctx.accounts.system_program.to_account_info(),
            system_program::Transfer {
                from: bonding_curve.to_account_info(),
                to: ctx.accounts.fees_wallet.to_account_info(),
            },
        ),
        fees as u64,
    )?;

    bonding_curve.sol_reserves -= total_sell_value;
    bonding_curve.token_reserves += tokens_to_sell;

    user_position.sol_spent -= total_sell_value;
    user_position.tokens_bought -= tokens_to_sell;

    emit!(NewTrade {
        token: ctx.accounts.user_token_account.mint,
        sol_spent: total_payout,
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
