mod cognite_client;

mod api;
mod auth;
mod dto;
mod error;
mod retry;
pub mod utils;

pub mod assets {
    pub use super::api::core::assets::*;
    pub use super::dto::core::asset::*;
}

pub mod time_series {
    pub use super::api::core::time_series::*;
    pub use super::dto::core::{datapoint::*, time_serie::*};
}

pub mod events {
    pub use super::api::core::events::*;
    pub use super::dto::core::event::*;
}

pub mod files {
    pub use super::api::core::files::*;
    pub use super::dto::core::files::*;
}

pub mod raw {
    pub use super::api::data_ingestion::raw::*;
    pub use super::dto::data_ingestion::raw::*;
}

pub mod extpipes {
    pub use super::api::data_ingestion::extpipes::*;
    pub use super::dto::data_ingestion::extpipes::*;
}

pub mod datasets {
    pub use super::api::data_organization::datasets::*;
    pub use super::dto::data_organization::datasets::*;
}

pub mod labels {
    pub use super::api::data_organization::labels::*;
    pub use super::dto::data_organization::labels::*;
}

pub mod relationships {
    pub use super::api::data_organization::relationships::*;
    pub use super::dto::data_organization::relationships::*;
}

pub mod sequences {
    pub use super::api::core::sequences::*;
    pub use super::dto::core::sequences::*;
}

pub mod models {
    pub use super::api::data_modeling::*;
    pub use super::dto::data_modeling::common::*;
    pub use super::dto::data_modeling::containers::*;
    pub use super::dto::data_modeling::data_models::*;
    pub use super::dto::data_modeling::instances::*;
    pub use super::dto::data_modeling::query::*;
    pub use super::dto::data_modeling::spaces::*;
    pub use super::dto::data_modeling::value::*;
    pub use super::dto::data_modeling::views::*;
}

pub mod iam {
    pub use super::api::iam::{groups::*, security_categories::*, sessions::*};
    pub use super::dto::iam::{group::*, security_category::*, session::*};
}

pub use self::{
    api::{api_client::*, authenticator::*, resource::*, utils::*},
    auth::*,
    cognite_client::*,
    dto::{filter_types::*, identity::*, items::*, params::*, patch_item::*},
    error::*,
    retry::*,
};

pub mod middleware {
    pub use crate::auth::AuthenticatorMiddleware;
    pub use crate::retry::CustomRetryMiddleware;
}

pub mod prelude {
    pub use super::api::resource::*;
    pub use super::cognite_client::*;
    pub use super::dto::filter_types::*;
    pub use super::dto::identity::*;
    pub use super::dto::items::*;
    pub use super::dto::patch_item::*;
}
