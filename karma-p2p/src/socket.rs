// use async_trait::async_trait;

use std::{
    pin::Pin,
    task::{Context, Poll},
};

use crate::P2pStream;

// #[async_trait(?Send)]
pub trait P2pSocket: Sized {
    type Stream: P2pStream;

    type Addr;

    type Error;

    /// Create p2p socket.
    fn poll_bind(cx: &mut Context<'_>, bootstrap: Self::Addr) -> Poll<Result<Self, Self::Error>>;

    /// Connect to remote p2p socket and get p2p stream.
    fn poll_connect(
        self: Pin<&Self>,
        cx: &mut Context<'_>,
        label: Self::Addr,
        port: u16,
    ) -> Poll<Result<Self::Stream, Self::Error>>;

    fn poll_start(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>>;

    /// Get local address.
    fn poll_fetch_local_addr(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Result<Self::Addr, Self::Error>>;

    /// Set remote address.
    fn poll_set_remote_addr(
        self: Pin<&Self>,
        cx: &mut Context<'_>,
        remote: Self::Addr,
    ) -> Poll<Result<(), Self::Error>>;
}
