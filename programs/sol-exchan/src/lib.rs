pub mod ray_token;

use anchor_lang::prelude::*;

declare_id!("5umccz9MSnMxV2UDbQWEoxarCk7UFaipFkcgzW5ggKs2");

// #[program]
pub mod sol_exchan {
    // use super::*;

    // pub fn handle_sol_transfer(ctx: Context<HandleSolTransfer>) -> Result<()> {
    //     // Assuming the SOL transfer already happened, we transfer the token
    //     let ix = transfer(
    //         &spl_token::ID,
    //         &ctx.accounts.from_token_account.key(),
    //         &ctx.accounts.to_token_account.key(),
    //         &ctx.accounts.token_authority.key(),
    //         &[],
    //         1, // Number of tokens to transfer
    //     )?;
    //     anchor_lang::solana_program::program::invoke(
    //         &ix,
    //         &[
    //             ctx.accounts.from_token_account.to_account_info(),
    //             ctx.accounts.to_token_account.to_account_info(),
    //             ctx.accounts.token_authority.to_account_info(),
    //             ctx.accounts.token_program.to_account_info(),
    //         ],
    //     )?;
    //     Ok(())
    // }
}

// #[derive(Accounts)]
// pub struct HandleSolTransfer<'info> {
//     #[account(mut)]
//     pub from_token_account: AccountInfo<'info>,
//     #[account(mut)]
//     pub to_token_account: AccountInfo<'info>,
//     #[account(signer)]
//     pub token_authority: AccountInfo<'info>,
//     pub token_program: AccountInfo<'info>,
// }