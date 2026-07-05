use soroban_sdk::{symbol_short, Address, Env, Symbol};

use crate::types::MarketType;

/// Emitted when a new market is created.
pub fn emit_market_created(
    env: &Env,
    market_id: u64,
    creator: &Address,
    market_type: &MarketType,
    kickoff_time: u64,
) {
    let topics: (Symbol, Address, MarketType) = (
        symbol_short!("mkt_new"),
        creator.clone(),
        market_type.clone(),
    );
    env.events().publish(topics, (market_id, kickoff_time));
}

/// Emitted when a user places a stake.
pub fn emit_stake_placed(
    env: &Env,
    market_id: u64,
    user: &Address,
    outcome: u32,
    amount: i128,
    new_pool_total: i128,
) {
    let topics: (Symbol, Address, u64, u32) =
        (symbol_short!("stake"), user.clone(), market_id, outcome);
    env.events().publish(topics, (amount, new_pool_total));
}

/// Emitted when a market is locked (kickoff reached).
pub fn emit_market_locked(env: &Env, market_id: u64, total_pool: i128) {
    let topics: (Symbol, u64) = (symbol_short!("locked"), market_id);
    env.events().publish(topics, total_pool);
}

/// Emitted when the oracle submits a result.
pub fn emit_result_submitted(env: &Env, market_id: u64, winning_outcome: u32, oracle: &Address) {
    let topics: (Symbol, u64, Address) = (symbol_short!("result"), market_id, oracle.clone());
    env.events().publish(topics, winning_outcome);
}

/// Emitted when a user claims winnings.
pub fn emit_winnings_claimed(env: &Env, market_id: u64, user: &Address, amount: i128) {
    let topics: (Symbol, Address, u64) = (symbol_short!("claim"), user.clone(), market_id);
    env.events().publish(topics, amount);
}

/// Emitted when a market is cancelled by admin.
pub fn emit_market_cancelled(env: &Env, market_id: u64) {
    let topics: (Symbol, u64) = (symbol_short!("cancel"), market_id);
    env.events().publish(topics, ());
}

/// Emitted when a user claims a refund (cancelled or refunded market).
pub fn emit_refund_claimed(env: &Env, market_id: u64, user: &Address, amount: i128) {
    let topics: (Symbol, Address, u64) = (symbol_short!("refund"), user.clone(), market_id);
    env.events().publish(topics, amount);
}

/// Emitted when protocol fees are collected.
pub fn emit_fee_collected(env: &Env, market_id: u64, amount: i128) {
    let topics: (Symbol, u64) = (symbol_short!("fee"), market_id);
    env.events().publish(topics, amount);
}
