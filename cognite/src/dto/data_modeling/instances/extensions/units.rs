use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use super::{common::CogniteDescribable, CogniteExtendable, WithView};

/// Represents a single unit of measurement.
pub type CogniteUnit = CogniteExtendable<Unit>;

impl WithView for CogniteUnit {
    const SPACE: &'static str = "cdf_cdm";
    const EXTERNAL_ID: &'static str = "CogniteUnit";
    const VERSION: &'static str = "v1";
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Debug, Default)]
#[serde(rename_all = "camelCase")]
/// The properties of the file object.
pub struct Unit {
    #[serde(flatten)]
    /// Cognite describable.
    pub description: CogniteDescribable,
    /// The symbol for the unit of measurement.
    pub symbol: Option<String>,
    /// Specifies the physical quantity the unit measures.
    pub quantity: Option<String>,
    /// Source of the unit definition
    pub source: Option<String>,
    /// Reference to the source of the unit definition.
    pub source_reference: Option<String>,
}
