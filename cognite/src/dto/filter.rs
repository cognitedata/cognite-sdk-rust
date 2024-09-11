use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::models::SourceReference;

mod query_value;
pub use query_value::*;
mod aggregate;
pub use aggregate::*;

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
#[skip_serializing_none]
/// Advanced filter. The `filter` module contains useful tools for
/// building filters.
///
/// # Example
///
/// This creates a filter matching nodes where "prop" is 15 and
/// externalId is not test, or "prop" is between 1 (inclusive) and 5 (exclusive)
///
/// ```rust
/// use cognite::filter::*;
/// equals(["space", "view/1", "prop"], 15)
///     .and(not(equals(["node", "externalId"], "test")))
///     .or(range(["space", "view/1", "prop"], 1..5));
/// ```
pub enum AdvancedFilter {
    /// Require the value of `property` to be equal to `value`
    Equals {
        /// Left hand side property.
        property: Vec<String>,
        /// Right hand side value.
        value: QueryValue,
    },
    /// Require the value to be in the list of `values`
    In {
        /// Left hand side property.
        property: Vec<String>,
        /// Right hand side list of values.
        values: QueryValue,
    },
    /// Require the value to be greater than `gt`, greater than or equal to `gte`,
    /// less than `lt` and less than or equal to `lte`
    Range {
        /// Left hand side property.
        property: Vec<String>,
        /// Greater than or equal to
        #[serde(skip_serializing_if = "Option::is_none")]
        gte: Option<QueryValue>,
        /// Greater than
        #[serde(skip_serializing_if = "Option::is_none")]
        gt: Option<QueryValue>,
        /// Less than or equal to
        #[serde(skip_serializing_if = "Option::is_none")]
        lte: Option<QueryValue>,
        /// Less than
        #[serde(skip_serializing_if = "Option::is_none")]
        lt: Option<QueryValue>,
    },
    /// Require the value to be text and start with `value`
    Prefix {
        /// Left hand side property.
        property: Vec<String>,
        /// Prefix value
        value: String,
    },
    /// Require this property to exist.
    Exists {
        /// Property that must exist.
        property: Vec<String>,
    },
    /// Matches items where the property contains one or more of the given values.
    /// This filter can only be applied to multivalued properties.
    ContainsAny {
        /// Multivalued property.
        property: Vec<String>,
        /// List of values.
        values: QueryValue,
    },
    /// Matches items where the property contains all the given values.
    /// This filter can only be applied to multivalued properties.
    ContainsAll {
        /// Multivalued property.
        property: Vec<String>,
        /// List of values.
        values: QueryValue,
    },
    /// An open filter that matches anything.
    MatchAll {},
    /// Use nested to apply the properties of the direct relation as the filter.
    /// `scope` specifies the direct relation property you want use as the filtering property.
    Nested {
        /// Direct relation property to query through.
        scope: Vec<String>,
        /// Filter to apply to nested property.
        filter: Box<AdvancedFilter>,
    },
    /// Matches items where the range made up of the two properties overlap with the provided range.
    Overlaps {
        /// Start property reference.
        start_property: Vec<String>,
        /// End property reference.
        end_property: Vec<String>,
        /// Greater than or equal to
        #[serde(skip_serializing_if = "Option::is_none")]
        gte: Option<QueryValue>,
        /// Greater than
        #[serde(skip_serializing_if = "Option::is_none")]
        gt: Option<QueryValue>,
        /// Less than or equal to
        #[serde(skip_serializing_if = "Option::is_none")]
        lte: Option<QueryValue>,
        /// Less than
        #[serde(skip_serializing_if = "Option::is_none")]
        lt: Option<QueryValue>,
    },
    /// Require items to have data in the referenced views, or containers.
    HasData(Vec<SourceReference>),
    /// Require all these filters to match.
    And(Vec<AdvancedFilter>),
    /// Require at least one of these filters to match.
    Or(Vec<AdvancedFilter>),
    /// Require this filter _not_ to match.
    Not(Box<AdvancedFilter>),
}

impl Default for AdvancedFilter {
    fn default() -> Self {
        Self::MatchAll {}
    }
}

/// Trait for values that can be converted into a property identifier.
pub trait PropertyIdentifier {
    /// Convert the value into an identifier.
    fn into_identifier(self) -> Vec<String>;
}

impl PropertyIdentifier for Vec<String> {
    fn into_identifier(self) -> Vec<String> {
        self
    }
}

impl PropertyIdentifier for &[String] {
    fn into_identifier(self) -> Vec<String> {
        self.to_owned()
    }
}

impl PropertyIdentifier for &[&str] {
    fn into_identifier(self) -> Vec<String> {
        self.iter().map(|&s| s.to_owned()).collect()
    }
}

impl<const N: usize> PropertyIdentifier for &[String; N] {
    fn into_identifier(self) -> Vec<String> {
        self.to_vec()
    }
}

impl<const N: usize> PropertyIdentifier for &[&str; N] {
    fn into_identifier(self) -> Vec<String> {
        self.iter().map(|&s| s.to_owned()).collect()
    }
}

impl<const N: usize> PropertyIdentifier for [String; N] {
    fn into_identifier(self) -> Vec<String> {
        self.to_vec()
    }
}

impl<const N: usize> PropertyIdentifier for [&str; N] {
    fn into_identifier(self) -> Vec<String> {
        self.iter().map(|&s| s.to_owned()).collect()
    }
}

/// Start or end of a range.
pub enum RangeItem<T> {
    /// Inclusive end point.
    Inclusive(T),
    /// Exclusive end point.
    Exclusive(T),
    /// Unbounded end point.
    Empty,
}

impl<T> RangeItem<T> {
    /// Map the inner value.
    pub fn map<R>(self, map: impl FnOnce(T) -> R) -> RangeItem<R> {
        match self {
            RangeItem::Inclusive(r) => RangeItem::Inclusive(map(r)),
            RangeItem::Exclusive(r) => RangeItem::Exclusive(map(r)),
            RangeItem::Empty => RangeItem::Empty,
        }
    }
}

/// Trait for types that can be converted into a range for a DMS query.
pub trait IntoQueryRange {
    /// Create a query range.
    fn into_query_range(self) -> (RangeItem<QueryValue>, RangeItem<QueryValue>);
}

impl<T> IntoQueryRange for std::ops::Range<T>
where
    T: Into<QueryValue>,
{
    fn into_query_range(self) -> (RangeItem<QueryValue>, RangeItem<QueryValue>) {
        (
            RangeItem::Inclusive(self.start.into()),
            RangeItem::Exclusive(self.end.into()),
        )
    }
}

impl<T> IntoQueryRange for std::ops::RangeFrom<T>
where
    T: Into<QueryValue>,
{
    fn into_query_range(self) -> (RangeItem<QueryValue>, RangeItem<QueryValue>) {
        (RangeItem::Inclusive(self.start.into()), RangeItem::Empty)
    }
}

impl IntoQueryRange for std::ops::RangeFull {
    fn into_query_range(self) -> (RangeItem<QueryValue>, RangeItem<QueryValue>) {
        (RangeItem::Empty, RangeItem::Empty)
    }
}

impl<T> IntoQueryRange for std::ops::RangeInclusive<T>
where
    T: Into<QueryValue> + Clone,
{
    fn into_query_range(self) -> (RangeItem<QueryValue>, RangeItem<QueryValue>) {
        (
            RangeItem::Inclusive(self.start().clone().into()),
            RangeItem::Inclusive(self.end().clone().into()),
        )
    }
}

impl<T> IntoQueryRange for std::ops::RangeTo<T>
where
    T: Into<QueryValue>,
{
    fn into_query_range(self) -> (RangeItem<QueryValue>, RangeItem<QueryValue>) {
        (RangeItem::Empty, RangeItem::Exclusive(self.end.into()))
    }
}

impl<T> IntoQueryRange for std::ops::RangeToInclusive<T>
where
    T: Into<QueryValue>,
{
    fn into_query_range(self) -> (RangeItem<QueryValue>, RangeItem<QueryValue>) {
        (RangeItem::Empty, RangeItem::Exclusive(self.end.into()))
    }
}

impl<T: Into<QueryValue>> IntoQueryRange for (T, T) {
    fn into_query_range(self) -> (RangeItem<QueryValue>, RangeItem<QueryValue>) {
        (
            RangeItem::Inclusive(self.0.into()),
            RangeItem::Exclusive(self.1.into()),
        )
    }
}

impl<T> IntoQueryRange for (RangeItem<T>, RangeItem<T>)
where
    T: Into<QueryValue>,
{
    fn into_query_range(self) -> (RangeItem<QueryValue>, RangeItem<QueryValue>) {
        (self.0.map(Into::into), self.1.map(Into::into))
    }
}

/// Filter builder methods.
pub(crate) mod filter_methods {
    use super::*;
    /// Create an `Equals` filter. `property = value`
    ///
    /// # Arguments
    ///
    /// * `property` - Left hand side property.
    /// * `value` - Right hand side value.
    pub fn equals(
        property: impl PropertyIdentifier,
        value: impl Into<QueryValue>,
    ) -> AdvancedFilter {
        AdvancedFilter::Equals {
            property: property.into_identifier(),
            value: value.into(),
        }
    }
    /// Create an `In` filter. `property IN values`
    ///
    /// # Arguments
    ///
    /// * `property` - Left hand side property.
    /// * `value` - Right hand side list of values.
    pub fn is_in(
        property: impl PropertyIdentifier,
        values: impl Into<QueryValue>,
    ) -> AdvancedFilter {
        AdvancedFilter::In {
            property: property.into_identifier(),
            values: values.into(),
        }
    }

    /// Create a `Range` filter.
    ///
    /// # Arguments
    ///
    /// * `property` - Property to filter.
    /// * `range` - Range to check.
    ///
    /// `IntoQueryRange` is implemented for `(QueryValue, QueryValue)`, which interprets it as
    /// (inclusive, exclusive), as well as ranges in `std::ops`, such as `0..5`, `..`, `..=7`, etc.
    ///
    /// If you need fine control, use `(RangeItem<T>, RangeItem<T>)`
    pub fn range(property: impl PropertyIdentifier, range: impl IntoQueryRange) -> AdvancedFilter {
        let (start, end) = range.into_query_range();
        let mut lte = None;
        let mut gte = None;
        let mut lt = None;
        let mut gt = None;
        match start {
            RangeItem::Inclusive(i) => gte = Some(i),
            RangeItem::Exclusive(i) => gt = Some(i),
            RangeItem::Empty => (),
        }
        match end {
            RangeItem::Inclusive(i) => lte = Some(i),
            RangeItem::Exclusive(i) => lt = Some(i),
            RangeItem::Empty => (),
        }

        AdvancedFilter::Range {
            property: property.into_identifier(),
            gte,
            gt,
            lte,
            lt,
        }
    }

    /// Create a `Prefix` filter, `property STARTS WITH value`
    ///
    /// # Arguments
    ///
    /// * `property` - Property to filter.
    /// * `value` - Prefix string.
    pub fn prefix(property: impl PropertyIdentifier, value: impl Into<String>) -> AdvancedFilter {
        AdvancedFilter::Prefix {
            property: property.into_identifier(),
            value: value.into(),
        }
    }

    /// Create an `Exists` filter.
    ///
    /// # Arguments
    ///
    /// * `property` - Property that must exist.
    pub fn exists(property: impl PropertyIdentifier) -> AdvancedFilter {
        AdvancedFilter::Exists {
            property: property.into_identifier(),
        }
    }

    /// Create a `ContainsAny` filter.
    ///
    /// # Arguments
    ///
    /// * `property` - Multivalued property reference.
    /// * `values` - List of values.
    pub fn contains_any(
        property: impl PropertyIdentifier,
        values: impl Into<QueryValue>,
    ) -> AdvancedFilter {
        AdvancedFilter::ContainsAny {
            property: property.into_identifier(),
            values: values.into(),
        }
    }

    /// Create a `ContainsAll` filter.
    ///
    /// # Arguments
    ///
    /// * `property` - Multivalued property reference.
    /// * `values` - List of values.
    pub fn contains_all(
        property: impl PropertyIdentifier,
        values: impl Into<QueryValue>,
    ) -> AdvancedFilter {
        AdvancedFilter::ContainsAll {
            property: property.into_identifier(),
            values: values.into(),
        }
    }

    /// Create an empty `MatchAll` filter.
    pub fn match_all() -> AdvancedFilter {
        AdvancedFilter::MatchAll {}
    }

    /// Create a nested filter.
    ///
    /// # Arguments
    ///
    /// * `scope` - Direct relation property to query through.
    /// * `filter` - Filter to apply to referenced node.
    pub fn nested(scope: impl PropertyIdentifier, filter: AdvancedFilter) -> AdvancedFilter {
        AdvancedFilter::Nested {
            scope: scope.into_identifier(),
            filter: Box::new(filter),
        }
    }

    /// Construct an `Overlaps` filter.
    ///
    /// # Arguments
    ///
    /// * `start_property` - Start property
    /// * `end_property` - End property.
    /// * `range` - Range to check for overlap with.
    ///
    /// `IntoQueryRange` is implemented for `(QueryValue, QueryValue)`, which interprets it as
    /// (inclusive, exclusive), as well as ranges in `std::ops`, such as `0..5`, `..`, `..=7`, etc.
    ///
    /// If you need fine control, use `(RangeItem<T>, RangeItem<T>)`
    pub fn overlaps(
        start_property: impl PropertyIdentifier,
        end_property: impl PropertyIdentifier,
        range: impl IntoQueryRange,
    ) -> AdvancedFilter {
        let (start, end) = range.into_query_range();
        let mut lte = None;
        let mut gte = None;
        let mut lt = None;
        let mut gt = None;
        match start {
            RangeItem::Inclusive(i) => gte = Some(i),
            RangeItem::Exclusive(i) => gt = Some(i),
            RangeItem::Empty => (),
        }
        match end {
            RangeItem::Inclusive(i) => lte = Some(i),
            RangeItem::Exclusive(i) => lt = Some(i),
            RangeItem::Empty => (),
        }
        AdvancedFilter::Overlaps {
            start_property: start_property.into_identifier(),
            end_property: end_property.into_identifier(),
            gte,
            gt,
            lte,
            lt,
        }
    }

    /// Construct a `HasData` filter.
    ///
    /// # Arguments
    ///
    /// * `references` - List of sources that the results must have data in.
    pub fn has_data(references: Vec<SourceReference>) -> AdvancedFilter {
        AdvancedFilter::HasData(references)
    }

    /// Construct a `not` filter.
    ///
    /// # Arguments
    ///
    /// * `filter` - Filter to invert.
    pub fn not(filter: AdvancedFilter) -> AdvancedFilter {
        filter.not()
    }

    /// Construct an `and` filter from a vector of filters.
    ///
    /// # Arguments
    ///
    /// * `filters` - Filters to `and` together.
    pub fn and(filters: Vec<AdvancedFilter>) -> AdvancedFilter {
        AdvancedFilter::And(filters)
    }

    /// Construct an `or` filter from a vector of filters.
    ///
    /// # Arguments
    ///
    /// * `filters` - Filters to `or` together.
    pub fn or(filters: Vec<AdvancedFilter>) -> AdvancedFilter {
        AdvancedFilter::Or(filters)
    }
}

impl AdvancedFilter {
    #[allow(clippy::should_implement_trait)]
    /// Construct a `not` filter.
    ///
    /// # Arguments
    ///
    /// * `filter` - Filter to invert.
    pub fn not(self) -> Self {
        match self {
            Self::Not(n) => *n,
            _ => Self::Not(Box::new(self)),
        }
    }
    /// Construct an `and` filter from this filter and another filter.
    ///
    /// # Arguments
    ///
    /// * `filter` - AND with this filter.
    pub fn and(mut self, filter: AdvancedFilter) -> Self {
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
    pub fn or(mut self, filter: AdvancedFilter) -> Self {
        match &mut self {
            Self::Or(a) => {
                a.push(filter);
                self
            }
            _ => Self::Or(vec![self, filter]),
        }
    }
}
