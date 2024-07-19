use anchor_lang::prelude::*;
use rust_decimal::{
    prelude::{FromPrimitive, ToPrimitive},
    Decimal,
};

#[account]
pub struct GlobalState {
    pub admin: Pubkey,
    // fees to be taken from every user trade
    pub fees_basis_points: u16,
    // time from the token creation till when users will need to get refunded
    pub lockup_duration: u64,
    pub fees_wallet: Pubkey,
}

#[account]
pub struct BondingCurve {
    pub token_reserves: u64,
    pub sol_reserves: u64,
    pub start: u64,
    /// marked as true when the seeding funds are withdrawn
    pub complete: bool,
}

#[account]
pub struct UserPosition {
    pub token: Pubkey,
    pub sol_spent: u64,
    pub tokens_bought: u64,
    pub refunded: bool,
}

impl BondingCurve {
    pub fn lockup_period_over(&self, lockup_duration: u64) -> Result<bool> {
        let ended = self.start + lockup_duration < Clock::get()?.unix_timestamp as u64;
        Ok(ended)
    }
}

#[event]
pub struct NewToken {
    pub name: String,
    pub symbol: String,
    pub uri: String,
    pub token: Pubkey,
    pub creator : Pubkey,
}

#[event]
pub struct NewTrade {
    pub token: Pubkey,
    pub sol_spent: u64,
    pub tokens_bought: u64,
    pub buyer: Pubkey,
    pub timestamp: u64,
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_price_at_reserve_initial() {
        let curve = BondingCurve {
            token_reserves: BondingCurve::TOTAL_TOKENS,
            sol_reserves: 0,
            start: 0,
            complete: false,
        };
        let price = curve.price_at_reserve(BondingCurve::TOTAL_TOKENS);
        assert!(
            (price - BondingCurve::INITIAL_PRICE).abs() < Decimal::new(1, 10),
            "Initial price mismatch, got: {}, expected: {}",
            price,
            BondingCurve::INITIAL_PRICE
        );
    }

    #[test]
    fn test_price_at_reserve_final() {
        let curve = BondingCurve {
            token_reserves: 0,
            sol_reserves: 0,
            start: 0,
            complete: false,
        };
        let price = curve.price_at_reserve(0);
        assert!(
            (price - BondingCurve::FINAL_PRICE).abs() < Decimal::new(1, 10),
            "Final price mismatch for 0 reserves, got: {}, expected: {}",
            price,
            BondingCurve::FINAL_PRICE
        );
    }

    #[test]
    fn test_calculate_cost_small_amount() {
        let curve = BondingCurve {
            token_reserves: BondingCurve::TOTAL_TOKENS,
            sol_reserves: 0,
            start: 0,
            complete: false,
        };
        let tokens_to_buy = 100_000_000; // 100 tokens in subunits
        let cost_in_lamports = curve.calculate_cost(tokens_to_buy);
        let expected_cost_sol = (curve.price_at_reserve(BondingCurve::TOTAL_TOKENS)
            + curve.price_at_reserve(BondingCurve::TOTAL_TOKENS - tokens_to_buy))
            / Decimal::new(2, 0)
            * Decimal::from_u64(tokens_to_buy).unwrap();
        let expected_cost_lamports = BondingCurve::sol_to_lamports(expected_cost_sol);
        assert!(
            (cost_in_lamports as i64 - expected_cost_lamports as i64).abs() < 1,
            "Cost mismatch for small amount of tokens, got: {}, expected: {}",
            cost_in_lamports,
            expected_cost_lamports
        );
    }

    #[test]
    fn test_calculate_cost_large_amount() {
        let curve = BondingCurve {
            token_reserves: BondingCurve::TOTAL_TOKENS,
            sol_reserves: 0,
            start: 0,
            complete: false,
        };
        let tokens_to_buy = 1_000_000_000_000; // 1 million tokens in subunits
        let cost_in_lamports = curve.calculate_cost(tokens_to_buy);
        let expected_cost_sol = (curve.price_at_reserve(BondingCurve::TOTAL_TOKENS)
            + curve.price_at_reserve(BondingCurve::TOTAL_TOKENS - tokens_to_buy))
            / Decimal::new(2, 0)
            * Decimal::from_u64(tokens_to_buy).unwrap();
        let expected_cost_lamports = BondingCurve::sol_to_lamports(expected_cost_sol);
        assert!(
            (cost_in_lamports as i64 - expected_cost_lamports as i64).abs() < 1,
            "Cost mismatch for large amount of tokens, got: {}, expected: {}",
            cost_in_lamports,
            expected_cost_lamports
        );
    }

    #[test]
    fn test_calculate_cost_partial_progress() {
        let curve = BondingCurve {
            token_reserves: BondingCurve::TOTAL_TOKENS / 2, // Half of the tokens already sold
            sol_reserves: 0,
            start: 0,
            complete: false,
        };
        let tokens_to_buy = 1_000_000_000_000; // Buy another 1 million tokens in subunits
        let cost_in_lamports = curve.calculate_cost(tokens_to_buy);
        let expected_cost_sol = (curve.price_at_reserve(BondingCurve::TOTAL_TOKENS / 2)
            + curve.price_at_reserve(BondingCurve::TOTAL_TOKENS / 2 - tokens_to_buy))
            / Decimal::new(2, 0)
            * Decimal::from_u64(tokens_to_buy).unwrap();
        let expected_cost_lamports = BondingCurve::sol_to_lamports(expected_cost_sol);
        assert!(
            (cost_in_lamports as i64 - expected_cost_lamports as i64).abs() < 1,
            "Cost mismatch for partial progress, got: {}, expected: {}",
            cost_in_lamports,
            expected_cost_lamports
        );
    }

    #[test]
    fn test_buy_entire_supply() {
        let curve = BondingCurve {
            token_reserves: BondingCurve::TOTAL_TOKENS,
            sol_reserves: 0,
            start: 0,
            complete: false,
        };
        let tokens_to_buy = BondingCurve::TOTAL_TOKENS;
        let cost_in_lamports = curve.calculate_cost(tokens_to_buy);
        assert_eq!(
            cost_in_lamports,
            75 * 1e9 as u64,
            "Cost mismatch for buying entire supply, got: {}, expected: {}",
            cost_in_lamports,
            75 * 1e9 as u64
        );
    }
}