use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::RawValue;

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
/// Filter for aggregating values.
pub enum AggregateFilter {
    /// Require the value to be in the list of `values`
    In {
        /// List of values.
        values: RawValue,
    },
    /// Require the value to be text and start with `prefix`.
    Prefix {
        /// String prefix.
        prefix: String,
    },
    /// Require the value to be greater than `gt`, greater than or equal to `gte`,
    /// less than `lt` and less than or equal to `lte`
    Range {
        /// Greater than or equal to
        gte: Option<RawValue>,
        /// Greater than
        gt: Option<RawValue>,
        /// Less than or equal to
        lte: Option<RawValue>,
        /// Less than
        lt: Option<RawValue>,
    },
    /// Require all these filters to match
    And(Vec<AggregateFilter>),
    /// Require at least one of these filters to match.
    Or(Vec<AggregateFilter>),
    /// Require this filter _not_ to match.
    Not(Box<AggregateFilter>),
}

impl AggregateFilter {
    /// Construct an `In` filter.
    ///
    /// # Arguments
    ///
    /// * `values` - List of values.
    pub fn is_in(values: impl Into<RawValue>) -> Self {
        Self::In {
            values: values.into(),
        }
    }
    /// Construct a `Prefix` filter.
    ///
    /// # Arguments
    ///
    /// * `prefix` - String prefix.
    pub fn prefix(prefix: impl Into<String>) -> Self {
        Self::Prefix {
            prefix: prefix.into(),
        }
    }
    /// Construct a `range` filter.
    ///
    /// # Arguments
    ///
    /// * `gte` - Greater than or equal to.
    /// * `gt` - Greater than.
    /// * `lte` - Less than or equal to.
    /// * `lt` - Less than.
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
    /// Construct a `not` filter.
    ///
    /// # Arguments
    ///
    /// * `filter` - Filter to invert.
    pub fn not(filter: AggregateFilter) -> Self {
        match filter {
            Self::Not(n) => *n,
            _ => Self::Not(Box::new(filter)),
        }
    }
    /// Construct an `and` filter from this filter and another filter.
    ///
    /// # Arguments
    ///
    /// * `filter` - AND with this filter.
    pub fn and(mut self, filter: AggregateFilter) -> Self {
        match &mut self {
            Self::And(a) => {
                a.push(filter);
                self
            }
            _ => Self::And(vec![self, filter]),
        }
    }

    /// Construct an `or` filter from this filter and another filter.
    ///
    /// # Arguments
    ///
    /// * `filter` - OR with this filter.
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
