use soroban_sdk::{contracttype, Address, Env};

use crate::types::Market;

// ── Storage key constants ────────────────────────────────────────────────
const TTL_MARKET_MIN: u32 = 100_000;
const TTL_MARKET_MAX: u32 = 500_000;
const TTL_USER_MIN: u32 = 50_000;
const TTL_USER_MAX: u32 = 200_000;

// ── Data key enumeration ─────────────────────────────────────────────────

#[contracttype]
pub enum DataKey {
    Admin,
    Oracle,
    FeeCollector,
    DefaultFeeBps,
    NextMarketId,
    Paused,
    Market(u64),
    OutcomeTotal(u64, u32),
    UserStake(Address, u64, u32),
    UserClaimed(Address, u64),
}

// ── Singleton storage helpers (instance) ─────────────────────────────────

pub fn get_admin(env: &Env) -> Option<Address> {
    env.storage().instance().get(&DataKey::Admin)
}

pub fn set_admin(env: &Env, admin: &Address) {
    env.storage().instance().set(&DataKey::Admin, admin);
}

pub fn get_oracle(env: &Env) -> Option<Address> {
    env.storage().instance().get(&DataKey::Oracle)
}

pub fn set_oracle(env: &Env, oracle: &Address) {
    env.storage().instance().set(&DataKey::Oracle, oracle);
}

pub fn get_fee_collector(env: &Env) -> Option<Address> {
    env.storage().instance().get(&DataKey::FeeCollector)
}

pub fn set_fee_collector(env: &Env, fee_collector: &Address) {
    env.storage()
        .instance()
        .set(&DataKey::FeeCollector, fee_collector);
}

pub fn get_default_fee_bps(env: &Env) -> Option<u32> {
    env.storage().instance().get(&DataKey::DefaultFeeBps)
}

pub fn set_default_fee_bps(env: &Env, fee_bps: u32) {
    env.storage()
        .instance()
        .set(&DataKey::DefaultFeeBps, &fee_bps);
}

pub fn get_next_market_id(env: &Env) -> u64 {
    env.storage()
        .instance()
        .get(&DataKey::NextMarketId)
        .unwrap_or(1)
}

pub fn set_next_market_id(env: &Env, id: u64) {
    env.storage().instance().set(&DataKey::NextMarketId, &id);
}

pub fn increment_and_get_next_market_id(env: &Env) -> u64 {
    let next = get_next_market_id(env);
    set_next_market_id(env, next + 1);
    next
}

pub fn is_paused(env: &Env) -> bool {
    env.storage()
        .instance()
        .get(&DataKey::Paused)
        .unwrap_or(false)
}

pub fn set_paused(env: &Env, paused: bool) {
    env.storage().instance().set(&DataKey::Paused, &paused);
}

// ── Market storage helpers (persistent) ──────────────────────────────────

pub fn has_market(env: &Env, id: u64) -> bool {
    env.storage().persistent().has(&DataKey::Market(id))
}

pub fn get_market(env: &Env, id: u64) -> Option<Market> {
    let key = DataKey::Market(id);
    let market: Option<Market> = env.storage().persistent().get(&key);
    if market.is_some() {
        env.storage()
            .persistent()
            .extend_ttl(&key, TTL_MARKET_MIN, TTL_MARKET_MAX);
    }
    market
}

pub fn set_market(env: &Env, id: u64, market: &Market) {
    let key = DataKey::Market(id);
    env.storage().persistent().set(&key, market);
    env.storage()
        .persistent()
        .extend_ttl(&key, TTL_MARKET_MIN, TTL_MARKET_MAX);
}

// ── Outcome total helpers (persistent) ───────────────────────────────────

pub fn get_outcome_total(env: &Env, market_id: u64, outcome: u32) -> i128 {
    let key = DataKey::OutcomeTotal(market_id, outcome);
    env.storage().persistent().get(&key).unwrap_or(0)
}

pub fn add_to_outcome_total(env: &Env, market_id: u64, outcome: u32, amount: i128) {
    let key = DataKey::OutcomeTotal(market_id, outcome);
    let current: i128 = env.storage().persistent().get(&key).unwrap_or(0);
    env.storage().persistent().set(&key, &(current + amount));
    env.storage()
        .persistent()
        .extend_ttl(&key, TTL_MARKET_MIN, TTL_MARKET_MAX);
}

// ── User stake helpers (persistent) ──────────────────────────────────────

pub fn get_user_stake(env: &Env, user: &Address, market_id: u64, outcome: u32) -> i128 {
    let key = DataKey::UserStake(user.clone(), market_id, outcome);
    env.storage().persistent().get(&key).unwrap_or(0)
}

pub fn add_to_user_stake(env: &Env, user: &Address, market_id: u64, outcome: u32, amount: i128) {
    let key = DataKey::UserStake(user.clone(), market_id, outcome);
    let current: i128 = env.storage().persistent().get(&key).unwrap_or(0);
    env.storage().persistent().set(&key, &(current + amount));
    env.storage()
        .persistent()
        .extend_ttl(&key, TTL_USER_MIN, TTL_USER_MAX);
}

// ── User claimed helpers (persistent) ────────────────────────────────────

pub fn get_user_claimed(env: &Env, user: &Address, market_id: u64) -> bool {
    let key = DataKey::UserClaimed(user.clone(), market_id);
    env.storage().persistent().get(&key).unwrap_or(false)
}

pub fn set_user_claimed(env: &Env, user: &Address, market_id: u64) {
    let key = DataKey::UserClaimed(user.clone(), market_id);
    env.storage().persistent().set(&key, &true);
    env.storage()
        .persistent()
        .extend_ttl(&key, TTL_USER_MIN, TTL_USER_MAX);
}
