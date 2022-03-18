use futures_lite::{AsyncRead, AsyncWrite};

pub trait P2pStream: AsyncWrite + AsyncRead {}
