use async_trait::async_trait;
use karma_p2p::P2pSocket;
use wasm_bindgen::JsValue;
use web_sys::RtcPeerConnection;

use crate::{Addr, WebrtcStream};

pub struct WebrtcSocket {
    pc: RtcPeerConnection,
}

#[async_trait(?Send)]
impl P2pSocket for WebrtcSocket {
    type Error = JsValue;

    type Stream = WebrtcStream;

    type Addr = Addr;

    async fn bind(bootstrap: Self::Addr) -> Result<Self, Self::Error> {
        Ok(Self {})
    }

    async fn connect(&self, label: Self::Addr) -> Result<Self::Stream, Self::Error> {
        Ok(Self::Stream {})
    }

    async fn start(&self) -> Result<(), Self::Error> {
        Ok(())
    }

    async fn accept(&self) -> Result<Self::Stream, Self::Error> {
        Ok(Self::Stream {})
    }

    async fn fetch_local_addr(&self) -> Result<Self::Addr, Self::Error> {}

    async fn set_remote_addr(&self, remote: Self::Addr) -> Result<(), Self::Error> {}
}
