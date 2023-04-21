use serde::{Deserialize, Serialize};

use crate::models::SourceReference;

use super::value::QueryValue;

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub enum FdmFilter {
    Equals {
        property: Vec<String>,
        value: QueryValue,
    },
    In {
        property: Vec<String>,
        values: QueryValue,
    },
    Range {
        property: Vec<String>,
        gte: Option<QueryValue>,
        gt: Option<QueryValue>,
        lte: Option<QueryValue>,
        lt: Option<QueryValue>,
    },
    Prefix {
        property: Vec<String>,
        value: QueryValue,
    },
    Exists {
        property: Vec<String>,
    },
    ContainsAny {
        property: Vec<String>,
        values: QueryValue,
    },
    ContainsAll {
        property: Vec<String>,
        values: QueryValue,
    },
    MatchAll {},
    Nested {
        scope: Vec<String>,
        filter: Box<FdmFilter>,
    },
    Overlaps {
        start_property: Vec<String>,
        end_property: Vec<String>,
        gte: Option<QueryValue>,
        gt: Option<QueryValue>,
        lte: Option<QueryValue>,
        lt: Option<QueryValue>,
    },
    HasData(SourceReference),
    And(Vec<FdmFilter>),
    Or(Vec<FdmFilter>),
    Not(Box<FdmFilter>),
}

pub trait PropertyIdentifier {
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

impl FdmFilter {
    pub fn equals(property: impl PropertyIdentifier, value: impl Into<QueryValue>) -> Self {
        Self::Equals {
            property: property.into_identifier(),
            value: value.into(),
        }
    }

    pub fn is_in(property: impl PropertyIdentifier, values: impl Into<QueryValue>) -> Self {
        Self::In {
            property: property.into_identifier(),
            values: values.into(),
        }
    }

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

    pub fn prefix(property: impl PropertyIdentifier, value: impl Into<QueryValue>) -> Self {
        Self::Prefix {
            property: property.into_identifier(),
            value: value.into(),
        }
    }

    pub fn exists(property: impl PropertyIdentifier) -> Self {
        Self::Exists {
            property: property.into_identifier(),
        }
    }

    pub fn contains_any(property: impl PropertyIdentifier, values: impl Into<QueryValue>) -> Self {
        Self::ContainsAny {
            property: property.into_identifier(),
            values: values.into(),
        }
    }

    pub fn contains_all(property: impl PropertyIdentifier, values: impl Into<QueryValue>) -> Self {
        Self::ContainsAll {
            property: property.into_identifier(),
            values: values.into(),
        }
    }

    pub fn match_all() -> Self {
        Self::MatchAll {}
    }

    pub fn nested(scope: impl PropertyIdentifier, filter: FdmFilter) -> Self {
        Self::Nested {
            scope: scope.into_identifier(),
            filter: Box::new(filter),
        }
    }

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

    pub fn has_data(reference: SourceReference) -> Self {
        Self::HasData(reference)
    }

    #[allow(clippy::should_implement_trait)]
    pub fn not(filter: FdmFilter) -> Self {
        match filter {
            Self::Not(n) => *n,
            _ => Self::Not(Box::new(filter)),
        }
    }

    pub fn and(mut self, filter: FdmFilter) -> Self {
        match &mut self {
            Self::And(a) => {
                a.push(filter);
                self
            }
            _ => Self::And(vec![self, filter]),
        }
    }

    pub fn or(mut self, filter: FdmFilter) -> Self {
        match &mut self {
            Self::Or(a) => {
                a.push(filter);
                self
            }
            _ => Self::Or(vec![self, filter]),
        }
    }
}
