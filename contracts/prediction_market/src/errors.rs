use soroban_sdk::contracterror;

/// Contract errors.
#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Error {
    // Initialization
    NotInitialized = 1,
    AlreadyInitialized = 2,
    Unauthorized = 3,
    Paused = 4,
    // Market lifecycle
    MarketNotFound = 10,
    MarketNotOpen = 11,
    MarketNotLocked = 12,
    MarketNotSettled = 13,
    MarketAlreadySettled = 14,
    KickoffReached = 15,
    KickoffNotReached = 16,
    ResolutionDeadlinePassed = 17,
    ResolutionDeadlineNotReached = 18,
    // Input validation
    InvalidOutcome = 20,
    InvalidAmount = 21,
    InvalidKickoffTime = 22,
    InvalidResolutionDeadline = 23,
    // Claims
    AlreadyClaimed = 30,
    NoWinningsToClaim = 31,
    NoRefundToClaim = 32,
    // Config
    InvalidFeeBps = 40,
    InvalidMarketType = 41,
}
