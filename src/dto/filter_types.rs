use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct EpochTimestampRange {
    max: u128,
    min: u128,
}

impl EpochTimestampRange {
    pub fn new(min: u128, max: u128) -> EpochTimestampRange {
        EpochTimestampRange { min, max }
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct IntegerRange {
    max: u64,
    min: u64,
}

impl IntegerRange {
    pub fn new(min: u64, max: u64) -> IntegerRange {
        IntegerRange { min, max }
    }
}
