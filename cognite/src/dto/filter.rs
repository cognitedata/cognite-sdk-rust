use serde::{Deserialize, Serialize};

use crate::models::SourceReference;

mod query_value;
pub use query_value::*;
mod aggregate;
pub use aggregate::*;

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
/// Advanced filter
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
        gte: Option<QueryValue>,
        /// Greater than
        gt: Option<QueryValue>,
        /// Less than or equal to
        lte: Option<QueryValue>,
        /// Less than
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
        gte: Option<QueryValue>,
        /// Greater than
        gt: Option<QueryValue>,
        /// Less than or equal to
        lte: Option<QueryValue>,
        /// Less than
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

impl AdvancedFilter {
    /// Create an `Equals` filter. `property = value`
    ///
    /// # Arguments
    ///
    /// * `property` - Left hand side property.
    /// * `value` - Right hand side value.
    pub fn equals(property: impl PropertyIdentifier, value: impl Into<QueryValue>) -> Self {
        Self::Equals {
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
    pub fn is_in(property: impl PropertyIdentifier, values: impl Into<QueryValue>) -> Self {
        Self::In {
            property: property.into_identifier(),
            values: values.into(),
        }
    }

    /// Create a `Range` filter.
    ///
    /// # Arguments
    ///
    /// * `property` - Property to filter.
    /// * `gte` - Greater than or equal to.
    /// * `gt` - Greater than.
    /// * `lte` - Less than or equal to.
    /// * `lt` - Less than.
    pub fn range(
        property: impl PropertyIdentifier,
        gte: Option<impl Into<QueryValue>>,
        gt: Option<impl Into<QueryValue>>,
        lte: Option<impl Into<QueryValue>>,
        lt: Option<impl Into<QueryValue>>,
    ) -> Self {
        Self::Range {
            property: property.into_identifier(),
            gte: gte.map(|v| v.into()),
            gt: gt.map(|v| v.into()),
            lte: lte.map(|v| v.into()),
            lt: lt.map(|v| v.into()),
        }
    }

    /// Create a `Prefix` filter, `property STARTS WITH value`
    ///
    /// # Arguments
    ///
    /// * `property` - Property to filter.
    /// * `value` - Prefix string.
    pub fn prefix(property: impl PropertyIdentifier, value: impl Into<String>) -> Self {
        Self::Prefix {
            property: property.into_identifier(),
            value: value.into(),
        }
    }

    /// Create an `Exists` filter.
    ///
    /// # Arguments
    ///
    /// * `property` - Property that must exist.
    pub fn exists(property: impl PropertyIdentifier) -> Self {
        Self::Exists {
            property: property.into_identifier(),
        }
    }

    /// Create a `ContainsAny` filter.
    ///
    /// # Arguments
    ///
    /// * `property` - Multivalued property reference.
    /// * `values` - List of values.
    pub fn contains_any(property: impl PropertyIdentifier, values: impl Into<QueryValue>) -> Self {
        Self::ContainsAny {
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
    pub fn contains_all(property: impl PropertyIdentifier, values: impl Into<QueryValue>) -> Self {
        Self::ContainsAll {
            property: property.into_identifier(),
            values: values.into(),
        }
    }

    /// Create an empty `MatchAll` filter.
    pub fn match_all() -> Self {
        Self::MatchAll {}
    }

    /// Create a nested filter.
    ///
    /// # Arguments
    ///
    /// * `scope` - Direct relation property to query through.
    /// * `filter` - Filter to apply to referenced node.
    pub fn nested(scope: impl PropertyIdentifier, filter: AdvancedFilter) -> Self {
        Self::Nested {
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
    /// * `gte` - Greater than or equal to.
    /// * `gt` - Greater than.
    /// * `lte` - Less than or equal to.
    /// * `lt` - Less than.
    pub fn overlaps(
        start_property: impl PropertyIdentifier,
        end_property: impl PropertyIdentifier,
        gte: Option<impl Into<QueryValue>>,
        gt: Option<impl Into<QueryValue>>,
        lte: Option<impl Into<QueryValue>>,
        lt: Option<impl Into<QueryValue>>,
    ) -> Self {
        Self::Overlaps {
            start_property: start_property.into_identifier(),
            end_property: end_property.into_identifier(),
            gte: gte.map(|v| v.into()),
            gt: gt.map(|v| v.into()),
            lte: lte.map(|v| v.into()),
            lt: lt.map(|v| v.into()),
        }
    }

    /// Construct a `HasData` filter.
    ///
    /// # Arguments
    ///
    /// * `references` - List of sources that the results must have data in.
    pub fn has_data(references: Vec<SourceReference>) -> Self {
        Self::HasData(references)
    }

    #[allow(clippy::should_implement_trait)]
    /// Construct a `not` filter.
    ///
    /// # Arguments
    ///
    /// * `filter` - Filter to invert.
    pub fn not(filter: AdvancedFilter) -> Self {
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
