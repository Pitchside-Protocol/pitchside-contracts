use soroban_sdk::{contracttype, Address, String};

/// Market outcome type — determines the number of possible outcomes.
///
/// - `MatchResult` (3 outcomes): 0=Home, 1=Draw, 2=Away
/// - `OverUnder` (2 outcomes): 0=Under, 1=Over
/// - `BothTeamsScore` (2 outcomes): 0=No, 1=Yes
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum MarketType {
    MatchResult,
    OverUnder,
    BothTeamsScore,
}

impl MarketType {
    pub fn outcome_count(&self) -> u32 {
        match self {
            MarketType::MatchResult => 3,
            MarketType::OverUnder => 2,
            MarketType::BothTeamsScore => 2,
        }
    }
}

/// Lifecycle status of a prediction market.
///
/// - `Open`: accepting stakes
/// - `Locked`: event started / kickoff reached — no more stakes
/// - `Settled`: oracle submitted a result — payouts available
/// - `Cancelled`: admin cancelled the market
/// - `Refunded`: oracle failed to submit within grace period — all stakes refundable
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum MarketStatus {
    Open,
    Locked,
    Settled,
    Cancelled,
    Refunded,
}

/// A prediction market instance.
///
/// Each market is asset-agnostic and tracks its own token address.
/// The state machine is: `Open → Locked → Settled → (users claim)`.
/// Alternative terminal states: `Cancelled` (admin) or `Refunded` (oracle timeout).
#[contracttype]
#[derive(Clone, Debug)]
pub struct Market {
    /// Unique market identifier (auto-incremented by the contract).
    pub id: u64,
    /// Address that created the market.
    pub creator: Address,
    /// Type of market (MatchResult, OverUnder, BothTeamsScore).
    pub market_type: MarketType,
    /// Human-readable description of the event.
    pub description: String,
    /// Address of the token contract used for stakes and payouts.
    pub token: Address,
    /// Number of possible outcomes (derived from market_type, stored for convenience).
    pub outcome_count: u32,
    /// Extra data — e.g. goal line * 10 for OverUnder (2.5 → 25).
    pub extra_data: i128,
    /// Unix timestamp (seconds) — market locks automatically at this time.
    pub kickoff_time: u64,
    /// Unix timestamp (seconds) — oracle must submit result before this deadline.
    pub resolution_deadline: u64,
    /// Current lifecycle status.
    pub status: MarketStatus,
    /// Winning outcome index — meaningful only when `status == Settled`.
    pub winning_outcome: u32,
    /// Total pool of staked tokens (sum of all outcome totals).
    pub total_pool: i128,
    /// Fee in basis points (e.g. 250 = 2.5%) taken from winning payouts.
    pub fee_bps: u32,
}
