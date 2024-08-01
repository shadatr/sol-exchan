use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    metadata::{
        create_metadata_accounts_v3, mpl_token_metadata::types::DataV2, CreateMetadataAccountsV3,
        Metadata,
    },
    token::{Mint, Token},
};


pub fn handle_create_token(
    ctx: Context<CreateToken>,
    name: String,
    symbol: String,
    uri: String,
) -> Result<()> {
    msg!("starting in there");

    // attaching metadata to the coin
    create_metadata_accounts_v3(
        CpiContext::new(
            ctx.accounts.token_metadata_program.to_account_info(),
            CreateMetadataAccountsV3 {
                metadata: ctx.accounts.metadata_account.to_account_info(),
                mint: ctx.accounts.mint.to_account_info(),
                mint_authority: ctx.accounts.creator.to_account_info(),
                update_authority: ctx.accounts.creator.to_account_info(),
                payer: ctx.accounts.creator.to_account_info(),
                system_program: ctx.accounts.system_program.to_account_info(),
                rent: ctx.accounts.rent.to_account_info(),
            },
        ),
        DataV2 {
            name: name.clone(),
            symbol: symbol.to_uppercase(),
            uri: uri.clone(),
            seller_fee_basis_points: 0,
            creators: None,
            collection: None,
            uses: None,
        },
        false,
        true,
        None,
    )?;

    // // setting the mint authority
    // set_authority(
    //     CpiContext::new(
    //         ctx.accounts.token_program.to_account_info(),
    //         SetAuthority {
    //             account_or_mint: ctx.accounts.mint.to_account_info(),
    //             current_authority: ctx.accounts.creator.to_account_info(),
    //         },
    //     ),
    //     AuthorityType::MintTokens,
    //     None,
    // )?;

    emit!(NewToken {
        name,
        symbol,
        uri,
        token: *ctx.accounts.mint.to_account_info().key,
        creator: *ctx.accounts.creator.to_account_info().key,
    });
    Ok(())
}



#[derive(Accounts)]
pub struct CreateToken<'info> {
    #[account(mut)]
    creator: Signer<'info>,
    #[account(
    init,
    payer = creator,
    mint::decimals = 6,
    mint::authority = creator
    )]
    pub mint: Account<'info, Mint>,
    #[account(
    mut,
    seeds = [b"metadata", token_metadata_program.key().as_ref(), mint.key().as_ref()],
    bump,
    seeds::program = token_metadata_program.key(),
    )]
    /// CHECK: The metadata account for the token, checked through the Metaplex program CPI
    pub metadata_account: UncheckedAccount<'info>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_metadata_program: Program<'info, Metadata>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}


#[event]
pub struct NewToken {
    pub name: String,
    pub symbol: String,
    pub uri: String,
    pub token: Pubkey,
    pub creator: Pubkey,
}
