use std::{sync::Arc, task::Poll};

use bytes::Bytes;
use futures_lite::{AsyncRead, AsyncWrite, FutureExt};
use karma_p2p::P2pStream;
use smol::channel::Receiver;
use webrtc::data_channel::RTCDataChannel;

pub struct WebrtcStream {
    pub(crate) dc: Arc<RTCDataChannel>,
    pub(crate) data_rx: Receiver<Bytes>,
}

impl AsyncRead for WebrtcStream {
    fn poll_read(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
        buf: &mut [u8],
    ) -> Poll<std::io::Result<usize>> {
        let mut fu = Box::pin(async {
            match self.data_rx.recv().await {
                Ok(b) => {
                    buf.copy_from_slice(&b);
                    Ok(b.len())
                }
                Err(e) => Err(std::io::Error::new(std::io::ErrorKind::Other, e)),
            }
        });
        fu.poll(cx)
    }
}

impl AsyncWrite for WebrtcStream {
    fn poll_write(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
        buf: &[u8],
    ) -> Poll<std::io::Result<usize>> {
        let mut fu = Box::pin(async {
            let bytes = Bytes::copy_from_slice(buf);

            self.dc
                .send(&bytes)
                .await
                .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))
        });

        fu.poll(cx)
    }

    fn poll_flush(
        self: std::pin::Pin<&mut Self>,
        _cx: &mut std::task::Context<'_>,
    ) -> Poll<std::io::Result<()>> {
        Poll::Ready(Ok(()))
    }

    fn poll_close(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> Poll<std::io::Result<()>> {
        let mut fu = Box::pin(async {
            self.dc
                .close()
                .await
                .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))
        });

        fu.poll(cx)
    }
}

impl P2pStream for WebrtcStream {}
