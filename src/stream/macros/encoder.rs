macro_rules! encoder {
    ($(#[$attr:meta])* $name:ident) => {
        $(#[$attr])*
        #[pin_project::unsafe_project(Unpin)]
        #[derive(Debug)]
        ///
        /// This structure implements a [`Stream`](futures::stream::Stream) interface and will read
        /// uncompressed data from an underlying stream and emit a stream of compressed data.
        pub struct $name<S: futures::stream::Stream<Item = std::io::Result<bytes::Bytes>>> {
            #[pin]
            inner: crate::stream::Encoder<S, crate::codec::$name>,
        }

        impl<S: futures::stream::Stream<Item = std::io::Result<bytes::Bytes>>> $name<S> {
            /// Acquires a reference to the underlying stream that this encoder is wrapping.
            pub fn get_ref(&self) -> &S {
                self.inner.get_ref()
            }

            /// Acquires a mutable reference to the underlying stream that this encoder is
            /// wrapping.
            ///
            /// Note that care must be taken to avoid tampering with the state of the stream which
            /// may otherwise confuse this encoder.
            pub fn get_mut(&mut self) -> &mut S {
                self.inner.get_mut()
            }

            /// Acquires a pinned mutable reference to the underlying stream that this encoder is
            /// wrapping.
            ///
            /// Note that care must be taken to avoid tampering with the state of the stream which
            /// may otherwise confuse this encoder.
            pub fn get_pin_mut<'a>(self: std::pin::Pin<&'a mut Self>) -> std::pin::Pin<&'a mut S> {
                self.project().inner.get_pin_mut()
            }

            /// Consumes this encoder returning the underlying stream.
            ///
            /// Note that this may discard internal state of this encoder, so care should be taken
            /// to avoid losing resources when this is called.
            pub fn into_inner(self) -> S {
                self.inner.into_inner()
            }
        }

        impl<S: futures::stream::Stream<Item = std::io::Result<bytes::Bytes>>>
            futures::stream::Stream for $name<S>
        {
            type Item = std::io::Result<bytes::Bytes>;

            fn poll_next(
                self: std::pin::Pin<&mut Self>,
                cx: &mut std::task::Context<'_>,
            ) -> std::task::Poll<Option<std::io::Result<bytes::Bytes>>> {
                self.project().inner.poll_next(cx)
            }
        }

        const _: () = {
            fn _assert() {
                use std::{pin::Pin, io::Result};
                use bytes::Bytes;
                use futures::stream::Stream;
                use crate::util::{_assert_send, _assert_sync};

                _assert_send::<$name<Pin<Box<dyn Stream<Item = Result<Bytes>> + Send>>>>();
                _assert_sync::<$name<Pin<Box<dyn Stream<Item = Result<Bytes>> + Sync>>>>();
            }
        };
    }
}