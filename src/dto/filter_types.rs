use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct EpochTimestampRange {
    max: i64,
    min: i64,
}

impl EpochTimestampRange {
    pub fn new(min: i64, max: i64) -> EpochTimestampRange {
        EpochTimestampRange { min, max }
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct IntegerRange {
    max: i64,
    min: i64,
}

impl IntegerRange {
    pub fn new(min: i64, max: i64) -> IntegerRange {
        IntegerRange { min, max }
    }
}
