#![no_std]
#![allow(dead_code)]
use soroban_sdk::{contract, contractimpl, Env};

mod errors;
mod events;
mod storage;
mod types;

pub use errors::Error;
pub use types::*;

#[contract]
pub struct PredictionMarketContract;

#[contractimpl]
impl PredictionMarketContract {
    pub fn hello(env: Env) -> soroban_sdk::String {
        soroban_sdk::String::from_str(&env, "Pitchside")
    }
}

#[cfg(test)]
mod test;
