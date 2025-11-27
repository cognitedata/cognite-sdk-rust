//! Utility for leasing resources that need to be cleaned up, typically as part of tests.

use std::future::Future;

/// Utility for leasing resources that need to be cleaned up, typically as part of tests.
/// When the ResourceLease is dropped, it will attempt to delete all created resources
/// using the provided CleanResource implementation.
///
/// This will spawn a tokio task to perform the deletion, so it requires a tokio runtime to be active.
pub struct ResourceLease<R, T>
where
    R: CleanResource<T> + Send + Sync + 'static,
    T: Send + 'static,
{
    resource: Option<R>,
    created: Vec<T>,
    on_error: Option<Box<dyn Fn(crate::Error) + Send + Sync>>,
}

/// Trait for resources that can be cleaned up by deleting leased items.
/// Implement this trait for any resource type that supports deletion of items of type T.
pub trait CleanResource<T> {
    /// Delete the provided resources.
    fn clean_resource(
        &self,
        resources: Vec<T>,
    ) -> impl Future<Output = Result<(), crate::Error>> + Send;
}

impl<R, T> ResourceLease<R, T>
where
    R: CleanResource<T> + Send + Sync + 'static,
    T: Send + 'static,
{
    /// Constructor, with a callback for handling errors during cleanup.
    pub fn new(resource: R, on_error: impl Fn(crate::Error) + Send + Sync + 'static) -> Self {
        Self {
            resource: Some(resource),
            created: Vec::new(),
            on_error: Some(Box::new(on_error)),
        }
    }

    /// Constructor, with a default error handler that prints to stdout.
    pub fn new_println(resource: R) -> Self {
        Self::new(resource, |e| {
            println!("Error cleaning up leased resource: {}", e)
        })
    }

    /// Create resources using the provided async function, and add them to the lease if the request succeeds.
    pub async fn for_create<F, Fut>(&mut self, create_fn: F) -> Result<(), crate::Error>
    where
        F: Fn() -> Fut + Send,
        Fut: Future<Output = Result<Vec<T>, crate::Error>> + Send,
    {
        let created_resources = create_fn().await?;

        self.created.extend(created_resources);
        Ok(())
    }

    /// Add a list of resources to be deleted when the lease is dropped.
    pub fn add_resources(&mut self, resource: impl IntoIterator<Item = T>) {
        self.created.extend(resource);
    }

    /// Get a reference to the leased resources.
    pub fn resources(&self) -> &Vec<T> {
        &self.created
    }

    /// Immediately clean up the leased resources.
    /// Call this at the end of tests, to actually assert that the cleanup works.
    pub async fn clean(mut self) -> Result<(), crate::Error> {
        let resources = std::mem::take(&mut self.created);
        let Some(resource) = self.resource.take() else {
            return Ok(());
        };
        if resources.is_empty() {
            return Ok(());
        }
        resource.clean_resource(resources).await
    }
}

impl<R, T> Drop for ResourceLease<R, T>
where
    R: CleanResource<T> + Send + Sync + 'static,
    T: Send + 'static,
{
    fn drop(&mut self) {
        let resources = std::mem::take(&mut self.created);
        let Some(resource) = self.resource.take() else {
            return;
        };
        let on_error = self.on_error.take();

        if resources.is_empty() {
            return;
        }

        tokio::spawn(async move {
            let r = resource.clean_resource(resources).await;
            if let Err(e) = r {
                if let Some(on_error) = on_error {
                    on_error(e);
                }
            }
        });
    }
}
