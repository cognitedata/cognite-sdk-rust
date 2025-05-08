pub(crate) mod aggregates;

use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::time_series::TimestampOrRelative;

/// Matches records with the last updated time within the provided range.
///
/// The range must include at least a left (gt or gte) bound.
/// It is not allowed to specify two upper or lower bounds, e.g. gte and gt,
/// in the same filter.
#[skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LastUpdatedTimeFilter {
    /// Greater than or equal to
    pub gte: Option<TimestampOrRelative>,
    /// Greater than
    pub gt: Option<TimestampOrRelative>,
    /// Less than or equal to
    pub lte: Option<TimestampOrRelative>,
    /// Less than
    pub lt: Option<TimestampOrRelative>,
}
