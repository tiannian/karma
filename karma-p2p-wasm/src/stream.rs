use std::{
    pin::Pin,
    task::{Context, Poll},
};

use futures_lite::{AsyncRead, AsyncWrite};
use karma_p2p::P2pStream;

pub struct WebrtcStream {}

impl AsyncRead for WebrtcStream {
    fn poll_read(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut [u8],
    ) -> Poll<std::io::Result<usize>> {
        Poll::Pending
    }
}

impl AsyncWrite for WebrtcStream {
    fn poll_write(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &[u8],
    ) -> Poll<std::io::Result<usize>> {
        Poll::Pending
    }

    fn poll_close(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<std::io::Result<()>> {
        Poll::Pending
    }

    fn poll_flush(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<std::io::Result<()>> {
        Poll::Pending
    }
}

impl P2pStream for WebrtcStream {}
