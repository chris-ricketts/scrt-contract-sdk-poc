use scrt_contract_sdk::prelude::*;

// TODO: Can we simplify this derive and still use the serde imported from the sdk crate?
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(crate = "self::serde")] // Required because serde is rexported from sdk crate
pub struct Config {
    pub max_size: usize,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(crate = "self::serde")]
pub struct State {
    pub reminder_count: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(crate = "self::serde")]
pub struct Reminder {
    pub content: Vec<u8>,
    pub timestamp: u64,
}

impl StaticKey for Config {
    fn key() -> &'static [u8] {
        b"config"
    }
}

impl StaticKey for State {
    fn key() -> &'static [u8] {
        b"state"
    }
}
