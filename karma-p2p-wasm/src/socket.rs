use std::task::{Context, Poll};

use futures_lite::FutureExt;
use karma_p2p::P2pSocket;
use wasm_bindgen::JsValue;
use web_sys::{RtcConfiguration, RtcPeerConnection};

use crate::{Error, Result, WebrtcAddr, WebrtcStream};

pub struct WebrtcSocket {
    pc: RtcPeerConnection,
}

impl WebrtcSocket {
    async fn bind(bootstrap: WebrtcAddr) -> Result<Self> {
        if let WebrtcAddr::Bootstrap(addr) = bootstrap {
            let ice_servers = JsValue::from_serde(&addr)?;

            let mut config = RtcConfiguration::new();
            config.ice_servers(&ice_servers);

            let pc = RtcPeerConnection::new_with_configuration(&config)?;

            Ok(WebrtcSocket { pc })
        } else {
            Err(Error::ErrAddrType)
        }
    }

    async fn connect(&self, label: WebrtcAddr) -> Result<WebrtcStream> {
        Ok(WebrtcStream {})
    }
}

impl P2pSocket for WebrtcSocket {
    type Error = Error;

    type Stream = WebrtcStream;

    type Addr = WebrtcAddr;

    fn poll_bind(cx: &mut Context<'_>, bootstrap: Self::Addr) -> Poll<Result<Self>> {
        let mut fu = Box::pin(async move { WebrtcSocket::bind(bootstrap).await });

        fu.poll(cx)
    }

    fn poll_connect(self: std::pin::Pin<&Self>, cx: &mut Context<'_>, label: Self::Addr) -> Poll<Result<Self::Stream>> {
        let mut fu = Box::pin(async move { self.connect(label).await });

        fu.poll(cx)
    }
}
