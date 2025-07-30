//! Shims for send-ness of futures in wasm.
//! WASM futures are currently non-send, due to their use of JSValue.
//! Since we depend quite heavily on async traits with Send bounds in the
//! return type, we create our own "Send" and "Sync" traits for trait bounds
//! that only impose a Send bound on non-wasm targets.

#[cfg(not(target_arch = "wasm32"))]
mod imp {
    use futures::{stream::BoxStream, Stream, StreamExt};

    pub trait CondSend: Send {}
    impl<T> CondSend for T where T: Send {}
    pub trait CondSync: Sync {}
    impl<T> CondSync for T where T: Sync {}

    pub trait CondBoxedStream: Stream {
        fn boxed_cond<'a>(self) -> BoxStream<'a, Self::Item>
        where
            Self: 'a;
    }

    impl<T> CondBoxedStream for T
    where
        T: Stream + Sized + Send,
    {
        fn boxed_cond<'a>(self) -> BoxStream<'a, Self::Item>
        where
            Self: 'a,
        {
            self.boxed()
        }
    }
}

#[cfg(target_arch = "wasm32")]
mod imp {
    use futures::{stream::LocalBoxStream, Stream, StreamExt};
    pub trait CondSend {}
    impl<T> CondSend for T {}
    pub trait CondSync {}
    impl<T> CondSync for T {}

    pub trait CondBoxedStream: Stream {
        fn boxed_cond<'a>(self) -> LocalBoxStream<'a, Self::Item>
        where
            Self: 'a;
    }

    impl<T> CondBoxedStream for T
    where
        T: Stream + Sized,
    {
        fn boxed_cond<'a>(self) -> LocalBoxStream<'a, Self::Item>
        where
            Self: 'a,
        {
            self.boxed_local()
        }
    }
}

pub(crate) use imp::*;
