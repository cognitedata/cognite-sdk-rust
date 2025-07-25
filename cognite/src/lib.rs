#![warn(missing_docs)]
#![doc = include_str!("../README.md")]
mod cognite_client;

mod api;
mod auth;
mod dto;
mod error;
mod retry;

/// SDK library version.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Utility methods and tooling.
pub mod utils;

/// Common types for DTOs.
pub mod dto_common {
    pub use super::dto::core::common::*;
}

/// Assets represent objects or groups of objects from the physical world.
/// Assets are organized in hierarchies. For example, a water pump asset can
/// be part of a subsystem asset on an oil platform asset.
pub mod assets {
    pub use super::api::core::assets::*;
    pub use super::dto::core::asset::*;
}

/// A time series consists of a sequence of data points connected to a single asset.
/// For example, a water pump asset can have a temperature time series that records a data point in
/// units of °C every second.
pub mod time_series {
    pub use super::api::core::time_series::*;
    pub use super::dto::core::{datapoint::*, time_series::*};
}

/// Event objects store complex information about multiple assets over a time period.
/// Typical types of events might include Alarms, Process Data, and Logs.
///
/// For storage of low volume, manually generated, schedulable activities such as
/// maintenance schedules, work orders, or other "appointment" type activities. The Data Modelling
/// service is now recommended.
///
/// For storage of very high volume discrete events, consider using time series.
pub mod events {
    pub use super::api::core::events::*;
    pub use super::dto::core::event::*;
}

/// Files store documents, binary blobs, and other file data and relate it to assets.
pub mod files {
    pub use super::api::core::files::*;
    pub use super::dto::core::files::*;
}

/// Raw is a NoSQL JSON store. Each project can have a variable number of databases,
/// each of which will have a variable number of tables, each of which will have a variable
/// number of key-value objects. Only queries on key are supported through this API.
pub mod raw {
    pub use super::api::data_ingestion::raw::*;
    pub use super::dto::data_ingestion::raw::*;
}

/// Extraction pipelines represent applications and software running outside CDF.
pub mod extpipes {
    pub use super::api::data_ingestion::extpipes::*;
    pub use super::dto::data_ingestion::extpipes::*;
}

/// Data sets let you document and track data lineage, as well as
/// restrict access to data.
///
/// Data sets group and track data by its source.
/// For example, a data set can contain all work orders originating from SAP.
/// Typically, an organization will have one data set for each of its data ingestion pipelines in CDF.
pub mod datasets {
    pub use super::api::data_organization::datasets::*;
    pub use super::dto::data_organization::datasets::*;
}

/// Labels let you annotate resources such as assets and time series.
pub mod labels {
    pub use super::api::data_organization::labels::*;
    pub use super::dto::data_organization::labels::*;
}

/// Relationships lets you create custom links between different resources.
pub mod relationships {
    pub use super::api::data_organization::relationships::*;
    pub use super::dto::data_organization::relationships::*;
}

/// A sequence stores a table with up to 400 columns indexed by row number. There can be at most
/// 400 numeric columns and 200 string columns. Each of the columns has a pre-defined type:
/// a string, integer, or floating point number.
pub mod sequences {
    pub use super::api::core::sequences::*;
    pub use super::dto::core::sequences::*;
}

/// Data modeling lets you create complex data models to model industrial knowledge graphs.
pub mod models {
    pub use super::api::data_modeling::*;
    /// A container represents a bag of properties, each property has a type.
    /// Containers can have indexes, constraints, and default values.
    pub mod containers {
        pub use crate::dto::data_modeling::containers::*;
    }
    /// A data model is a collection of views. Use the data model to group and structure views into a
    /// recognizable and understood model. The model represents a reusable collection of data.
    pub mod data_models {
        pub use crate::dto::data_modeling::data_models::*;
    }
    /// Instances are nodes and edges in a data model. These contain the actual data in the data model.
    pub mod instances {
        pub use crate::dto::data_modeling::instances::*;
        pub use crate::dto::data_modeling::query::*;
    }
    /// Spaces group and namespace data modeling resources.
    pub mod spaces {
        pub use crate::dto::data_modeling::spaces::*;
    }
    /// Views provide a view into data in containers.
    pub mod views {
        pub use crate::dto::data_modeling::views::*;
    }
    pub use super::dto::data_modeling::common::*;
    /// Structures and methods for creating complex data modeling filters.
    pub mod filter {
        pub use crate::dto::filter::filter_methods::*;
        pub use crate::dto::filter::*;
    }

    /// Records are event-like items contained in a stream, but modelled using data modelling.
    pub mod records {
        pub use crate::dto::data_modeling::records::*;
        pub use crate::dto::data_modeling::streams::*;
        /// Aggregates on records.
        pub mod aggregates {
            pub use crate::dto::data_modeling::records::aggregates::*;
        }
    }
}

/// Groups are used to give principals the capabilities to access CDF resources. One principal
/// can be a member of multiple groups, and one group can have multiple members.
///
/// Security categories can be used to
/// restrict access to a resource. Applying a security category to a resource means that
/// only principals (users or service accounts) that also have this security category
/// can access the resource.
///
/// Sessions are used to maintain access to CDF resources for an extended period of time.
pub mod iam {
    pub use super::api::iam::{groups::*, security_categories::*, sessions::*};
    pub use super::dto::iam::{group::*, security_category::*, session::*};
}

pub use self::{
    api::{api_client::*, authenticator::*, request_builder::*, resource::*, utils::*},
    auth::*,
    cognite_client::*,
    dto::{filter::*, filter_types::*, identity::*, items::*, params::*, patch_item::*, utils::*},
    error::*,
    retry::*,
};

/// Structures and methods for creating complex filters.
pub mod filter {
    pub use super::dto::filter::filter_methods::*;
}

/// Middleware used by the cognite HTTP client.
pub mod middleware {
    pub use crate::auth::AuthenticatorMiddleware;
    pub use crate::retry::CustomRetryMiddleware;
}

/// Prelude containing common types and traits used when working with the SDK. This can be
/// glob-imported for convenience.
pub mod prelude {
    pub use super::api::resource::*;
    pub use super::cognite_client::*;
    pub use super::dto::filter_types::*;
    pub use super::dto::identity::*;
    pub use super::dto::items::*;
    pub use super::dto::patch_item::*;
}
