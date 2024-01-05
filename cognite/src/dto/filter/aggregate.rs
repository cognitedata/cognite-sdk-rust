use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::RawValue;

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub enum AggregateFilter {
    In {
        values: RawValue,
    },
    Prefix {
        prefix: String,
    },
    Range {
        gte: Option<RawValue>,
        gt: Option<RawValue>,
        lte: Option<RawValue>,
        lt: Option<RawValue>,
    },
    And(Vec<AggregateFilter>),
    Or(Vec<AggregateFilter>),
    Not(Box<AggregateFilter>),
}

impl AggregateFilter {
    pub fn is_in(values: impl Into<RawValue>) -> Self {
        Self::In {
            values: values.into(),
        }
    }
    pub fn prefix(prefix: impl Into<String>) -> Self {
        Self::Prefix {
            prefix: prefix.into(),
        }
    }
    pub fn range(
        gte: Option<impl Into<RawValue>>,
        gt: Option<impl Into<RawValue>>,
        lte: Option<impl Into<RawValue>>,
        lt: Option<impl Into<RawValue>>,
    ) -> Self {
        Self::Range {
            gte: gte.map(|v| v.into()),
            gt: gt.map(|v| v.into()),
            lte: lte.map(|v| v.into()),
            lt: lt.map(|v| v.into()),
        }
    }
    #[allow(clippy::should_implement_trait)]
    pub fn not(filter: AggregateFilter) -> Self {
        match filter {
            Self::Not(n) => *n,
            _ => Self::Not(Box::new(filter)),
        }
    }

    pub fn and(mut self, filter: AggregateFilter) -> Self {
        match &mut self {
            Self::And(a) => {
                a.push(filter);
                self
            }
            _ => Self::And(vec![self, filter]),
        }
    }

    pub fn or(mut self, filter: AggregateFilter) -> Self {
        match &mut self {
            Self::Or(a) => {
                a.push(filter);
                self
            }
            _ => Self::Or(vec![self, filter]),
        }
    }
}
