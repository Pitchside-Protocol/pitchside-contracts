use crate::PredictionMarketContract;
use soroban_sdk::{Env, String};

#[test]
fn test_hello() {
    let env = Env::default();
    let result = PredictionMarketContract::hello(env.clone());
    assert_eq!(result, String::from_str(&env, "Pitchside"));
}
