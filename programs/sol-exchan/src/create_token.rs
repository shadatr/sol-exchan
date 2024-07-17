
use anchor_lang::prelude::*;
use anchor_spl::token;
use anchor_spl::token::spl_token::instruction::AuthorityType;
use anchor_spl::{
    associated_token::AssociatedToken,
    metadata::{
        create_metadata_accounts_v3, mpl_token_metadata::types::DataV2, CreateMetadataAccountsV3,
        Metadata,
    },
    token::{mint_to, set_authority, Mint, MintTo, SetAuthority, Token, TokenAccount},
};

pub const TOKEN_SUPPLY : u64 = 10_000_000_000;

pub const TOKEN_DECIMALS : u32 = 6;

pub fn handle_create_token(
    ctx: Context<CreateToken>,
    name: String,
    symbol: String,
    uri: String,
) -> Result<()> {
    msg!("starting in there");

    mint_to(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            MintTo {
                authority: ctx.accounts.creator.to_account_info(),
                to: ctx.accounts.token_custody.to_account_info(),
                mint: ctx.accounts.mint.to_account_info(),
            },
        ),
        TOKEN_SUPPLY * 10u64.pow(TOKEN_DECIMALS),
    )?;

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
        false,
        None,
    )?;

    // setting the mint authority
    set_authority(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            SetAuthority {
                account_or_mint: ctx.accounts.mint.to_account_info(),
                current_authority: ctx.accounts.creator.to_account_info(),
            },
        ),
        AuthorityType::MintTokens,
        None,
    )?;


    msg!("Token created successfully");
    println!("name: {}, symbol: {}, uri: {} token: {}, creator {}", name, symbol, uri, *ctx.accounts.mint.to_account_info().key,*ctx.accounts.creator.to_account_info().key);


    Ok(())
}

#[derive(Accounts)]
pub struct CreateToken<'info> {
    #[account(mut)]
    creator: Signer<'info>,
    #[account(
    mut,
    seeds = [b"metadata", token_metadata_program.key().as_ref(), mint.key().as_ref()],
    bump,
    seeds::program = token_metadata_program.key(),
    )]
    /// CHECK: The metadata account for the token, checked through the Metaplex program CPI
    pub metadata_account: UncheckedAccount<'info>,
    #[account(
        init,
        payer = creator,
        associated_token::mint = mint,
        associated_token::authority = creator
        )]
    pub token_custody: Account<'info, TokenAccount>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_metadata_program: Program<'info, Metadata>,
    #[account(init, payer = creator, space = Mint::LEN)]
    pub mint: Account<'info, Mint>,
    #[account(address = token::ID)]
    pub token_program: Program<'info, Token>,
    pub rent: Sysvar<'info, Rent>,
    pub system_program: Program<'info, System>,

}