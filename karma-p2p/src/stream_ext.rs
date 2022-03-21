use futures_lite::{AsyncReadExt, AsyncWrite};

use crate::P2pStream;

pub trait P2pStreamExt: P2pStream + AsyncReadExt + AsyncWrite {}

impl<T: P2pStream> P2pStreamExt for T {}
