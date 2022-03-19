use async_trait::async_trait;

use crate::P2pStream;

#[async_trait(?Send)]
pub trait P2pSocket: Sized {
    type Stream: P2pStream;

    type Addr;

    type Error;

    /// Create p2p socket.
    async fn bind(bootstrap: Self::Addr) -> Result<Self, Self::Error>;

    /// Connect to remote p2p socket and get p2p stream.
    async fn connect(&self, label: Self::Addr) -> Result<Self::Stream, Self::Error>;

    async fn start(&self) -> Result<(), Self::Error>;

    /// Accept p2p connection and get p2p stream;
    async fn accept(&self) -> Result<Self::Stream, Self::Error>;

    /// Get local address.
    async fn fetch_local_addr(&self) -> Result<Self::Addr, Self::Error>;

    /// Set remote address.
    async fn set_remote_addr(&self, remote: Self::Addr) -> Result<(), Self::Error>;
}
