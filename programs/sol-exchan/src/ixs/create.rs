use anchor_lang::prelude::*;
use anchor_spl::token::spl_token::instruction::AuthorityType;
use anchor_spl::{
    associated_token::AssociatedToken,
    metadata::{
        create_metadata_accounts_v3, mpl_token_metadata::types::DataV2, CreateMetadataAccountsV3,
        Metadata,
    },
    token::{mint_to, set_authority, Mint, MintTo, SetAuthority, Token, TokenAccount},
};
use rust_decimal::{
    prelude::{FromPrimitive, ToPrimitive},
    Decimal,
};

pub const BONDING_CURVE_SEED: &[u8] = b"bonding_curve";

pub const TOKEN_SUPPLY: u64 = 10_000_000_000;
pub const TOKEN_DECIMALS: u32 = 6;

pub fn handle_create_token(
    ctx: Context<CreateToken>,
    name: String,
    symbol: String,
    uri: String,
) -> Result<()> {
    msg!("starting in there");

    let bonding_curve = &mut ctx.accounts.bonding_curve;

    // minting token's initial supply
    mint_to(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            MintTo {
                authority: ctx.accounts.creator.to_account_info(),
                to: ctx.accounts.token_custody.to_account_info(),
                mint: ctx.accounts.mint.to_account_info(),
            },
        ),
        TOKEN_SUPPLY * (10u64).pow(TOKEN_DECIMALS),
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

    bonding_curve.token_reserves = 8_000_000_000 * (10u64).pow(TOKEN_DECIMALS);
    bonding_curve.start = Clock::get()?.unix_timestamp as u64;

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

    // will keep track of the bonding curve's state, and also work as sol custody, no need for a separate account
    #[account(init, payer = creator, space = std::mem::size_of::< BondingCurve > () + 8, seeds=[BONDING_CURVE_SEED, mint.key().as_ref()], bump)]
    pub bonding_curve: Account<'info, BondingCurve>,

    #[account(
    init,
    payer = creator,
    associated_token::mint = mint,
    associated_token::authority = bonding_curve
    )]
    pub token_custody: Account<'info, TokenAccount>,
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


#[account]
pub struct BondingCurve {
    pub token_reserves: u64,
    pub sol_reserves: u64,
    pub start: u64,
    /// marked as true when the seeding funds are withdrawn
    pub complete: bool,
}

impl BondingCurve {
    pub fn lockup_period_over(&self, lockup_duration: u64) -> Result<bool> {
        let ended = self.start + lockup_duration < (Clock::get()?.unix_timestamp as u64);
        Ok(ended)
    }
}

impl BondingCurve {
    // Linear bonding curve constants
    const INITIAL_PRICE: Decimal = Decimal::from_parts(46875, 0, 0, false, 19); // 0.0000000046875
    const FINAL_PRICE: Decimal = Decimal::from_parts(140625, 0, 0, false, 19); // 0.0000000140625
    const TOTAL_TOKENS: u64 = 8_000_000_000 * 1_000_000; // Total subunits

    // Convert SOL to lamports (1 SOL = 1e9 lamports)
    fn sol_to_lamports(sol: Decimal) -> u64 {
        (sol * Decimal::new(1_000_000_000, 0)).to_u64().unwrap()
    }

    // Calculate the price of the token at a given reserve level
    fn price_at_reserve(&self, token_reserves: u64) -> Decimal {
        let progress = Decimal::from_u64(token_reserves).unwrap()
            / Decimal::from_u64(Self::TOTAL_TOKENS).unwrap();
        let progress = Decimal::new(1, 0) - progress;
        Self::INITIAL_PRICE + (Self::FINAL_PRICE - Self::INITIAL_PRICE) * progress
    }

    // Calculate the total cost in lamports for a given number of tokens (in subunits)
    pub fn calculate_cost(&self, tokens: u64) -> u64 {
        let initial_price = self.price_at_reserve(self.token_reserves);
        let final_price = self.price_at_reserve(self.token_reserves - tokens);

        // Average price over the linear bonding curve
        let average_price = (initial_price + final_price) / Decimal::new(2, 0);
        // Calculate the total cost in SOL
        let total_cost_sol = average_price * Decimal::from_u64(tokens).unwrap();
        println!("total cost sol {}", total_cost_sol);
        Self::sol_to_lamports(total_cost_sol)
    }
}

#[event]
pub struct NewToken {
    pub name: String,
    pub symbol: String,
    pub uri: String,
    pub token: Pubkey,
    pub creator: Pubkey,
}
