pub mod ixs;
pub mod state;
pub mod consts;
pub mod errors;
use anchor_lang::prelude::*;
use consts::GLOBAL_STATE_SEED;
use ixs::*;
use state::GlobalState;

declare_id!("AKHSTGMauhTHZWTbshqQefVFUBJDG3hqMtqtErjfMyNz");

#[program]
pub mod sol_exchan {

    use super::*;

    pub fn initialize_instaction(ctx: Context<InitializeInstraction>, fees: u16, lockup_duration: u64) -> Result<()> {
        let global_state = &mut ctx.accounts.global_state;
        global_state.admin = *ctx.accounts.signer.key;
        global_state.fees_basis_points = fees;
        global_state.lockup_duration = lockup_duration;
        global_state.fees_wallet = *ctx.accounts.fees_wallet.to_account_info().key;
        Ok(())
    }

    pub fn create_token(
        ctx: Context<CreateToken>,
        name: String,
        symbol: String,
        uri: String
    ) -> Result<()> {
        handle_create_token(ctx, name, symbol, uri)
    }

    pub fn buy(ctx: Context<Buy>, tokens_to_buy: u64) -> Result<()> {
        handle_buy(ctx, tokens_to_buy)
    }

    pub fn sell(ctx: Context<Sell>, tokens_to_sell: u64) -> Result<()> {
        
        handle_sell(ctx, tokens_to_sell)
    }

    pub fn swap_input(ctx: Context<ProxySwapBaseInput>, amount_in: u64, minimum_amount_out: u64) -> Result<()> {
        proxy_swap_base_input(ctx, amount_in, minimum_amount_out)
    }

    pub fn swap_output(ctx: Context<ProxySwapBaseOutput>, amount_out: u64, maximum_amount_in: u64) -> Result<()> {
        proxy_swap_base_output(ctx, amount_out, maximum_amount_in)
    }

    pub fn withdraw(ctx: Context<ProxyWithdraw>, lp_token_amount: u64, minimum_token_0_amount: u64, minimum_token_1_amount: u64) -> Result<()> {
        proxy_withdraw(ctx, lp_token_amount, minimum_token_0_amount, minimum_token_1_amount)
    }

    pub fn deposit(ctx: Context<ProxyDeposit>, lp_token_amount: u64, maximum_token_0_amount: u64, maximum_token_1_amount: u64) -> Result<()> {
        proxy_deposit(ctx, lp_token_amount, maximum_token_0_amount, maximum_token_1_amount)
    }

    pub fn initialize_rad(ctx: Context<ProxyInitialize>,init_amount_0: u64, init_amount_1: u64, open_time: u64) -> Result<()> {
        proxy_initialize(ctx, init_amount_0, init_amount_1, open_time)
    }
    
}

#[derive(Accounts)]
pub struct InitializeInstraction<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(
        init,
        payer = signer,
        space = std::mem::size_of::<GlobalState>() + 8,
        seeds = [GLOBAL_STATE_SEED],
        bump
    )]
    pub global_state: Account<'info, GlobalState>,

    pub fees_wallet: SystemAccount<'info>,

    pub system_program: Program<'info, System>,
}
