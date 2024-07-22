
use std::str::FromStr;

use anchor_lang::solana_program::pubkey::Pubkey;

pub fn get_ray_pool_id(market_address: &Pubkey) -> Pubkey {
    Pubkey::find_program_address(
        &[
            &Pubkey::from_str("675kPX9MHTjS2zt1qfr1NYHuzeLXfQM9H24wFSUt1Mp8").unwrap().to_bytes(),
            &market_address.to_bytes(),
            &b"amm_associated_seed".as_slice(),
        ],
        &Pubkey::from_str("675kPX9MHTjS2zt1qfr1NYHuzeLXfQM9H24wFSUt1Mp8").unwrap(),
    )
    .0
}
