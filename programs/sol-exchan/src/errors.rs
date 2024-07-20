use anchor_lang::error_code;


#[error_code]
pub enum NoRugErrors {
    #[msg("Curve is complete")]
    CurveComplete,
    #[msg("Market failed to reach target before lockup period, Refunds apply.")]
    LockupPeriodOver,
    #[msg("Invalid fees wallet")]
    InvalidFeesWallet,
    #[msg("Invalid token buy amount")]
    InvalidTokenBuyAmount,
    #[msg("Invalid token sell amount")]
    InvalidTokenSellAmount,
    #[msg("Unauthorized")]
    Unauthorized,
    #[msg("Curve not full yet")]
    CurveNotFull,
    #[msg("Already refunded")]
    AlreadyRefunded,
    #[msg("Insufficient Sol Reserves")]
    InsufficientSolReserves,
    #[msg("Insufficient Tokens")]
    InsufficientTokens,
    #[msg("Token live on Raydium")]
    TokenLive,
    #[msg("Lockup period not over yet")]
    LockupPeriodNotOver,
}