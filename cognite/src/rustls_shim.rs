#[cfg(feature = "rustls-022")]
mod dep {
    pub use http::Extensions;
    pub use reqwest_012 as reqwest;
    pub use reqwest_middleware_04 as reqwest_middleware;
}

#[cfg(feature = "rustls-021")]
mod dep {
    pub use reqwest_011 as reqwest;
    pub use reqwest_middleware_02 as reqwest_middleware;
    pub use task_local_extensions::Extensions;
}

pub use dep::*;
