//! Shims for send-ness of futures in wasm.
//! WASM futures are currently non-send, due to their use of JSValue.
//! Since we depend quite heavily on async traits with Send bounds in the
//! return type, we create our own "Send" and "Sync" traits for trait bounds
//! that only impose a Send bound on non-wasm targets.

#[cfg(not(target_arch = "wasm32"))]
mod imp {
    use futures::{stream::BoxStream, Stream, StreamExt};

    pub use Send as CondSend;
    pub use Sync as CondSync;

    pub trait CondBoxedStream: Sized + Stream + Send {
        fn boxed_cond<'a>(self) -> BoxStream<'a, Self::Item>
        where
            Self: 'a,
        {
            self.boxed()
        }
    }

    impl<T: Stream + Sized + Send> CondBoxedStream for T {}

    pub type CondBoxFuture<'a, T> = futures::future::BoxFuture<'a, T>;
}

#[cfg(target_arch = "wasm32")]
mod imp {
    use futures::{stream::LocalBoxStream, Stream, StreamExt};
    pub trait CondSend {}
    impl<T: ?Sized> CondSend for T {}
    pub trait CondSync {}
    impl<T: ?Sized> CondSync for T {}

    pub trait CondBoxedStream: Sized + Stream {
        fn boxed_cond<'a>(self) -> LocalBoxStream<'a, Self::Item>
        where
            Self: 'a,
        {
            self.boxed_local()
        }
    }

    impl<T: Stream + Sized> CondBoxedStream for T {}

    pub type CondBoxFuture<'a, T> = futures::future::LocalBoxFuture<'a, T>;
}

pub(crate) use imp::*;
