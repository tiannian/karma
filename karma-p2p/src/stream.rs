use futures_lite::{AsyncRead, AsyncWrite};

pub trait P2pStream: AsyncWrite + AsyncRead {}

impl<T: AsyncRead + AsyncWrite> P2pStream for T {}

