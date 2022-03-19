use std::{
    pin::Pin,
    sync::Arc,
    task::{Context, Poll},
};

use futures_lite::FutureExt;
use karma_p2p::P2pSocket;
use smol::channel::{unbounded, Receiver, Sender};
use webrtc::{
    api::{
        interceptor_registry::register_default_interceptors, media_engine::MediaEngine, APIBuilder,
    },
    data_channel::RTCDataChannel,
    interceptor::registry::Registry,
    peer_connection::{configuration::RTCConfiguration, RTCPeerConnection},
};

use crate::{Error, Result, WebrtcAddr, WebrtcStream};

pub struct WebrtcSocket {
    pc: Arc<RTCPeerConnection>,
    dc_rx: Receiver<Arc<RTCDataChannel>>,
    addr_tx: Sender<WebrtcAddr>,
    addr_rx: Receiver<WebrtcAddr>,
}

impl WebrtcSocket {
    async fn bind(bootstrap: WebrtcAddr) -> Result<Self> {
        let mut m = MediaEngine::default();

        m.register_default_codecs()?;

        let mut registry = Registry::default();

        registry = register_default_interceptors(registry, &mut m)?;

        let api = APIBuilder::new()
            .with_media_engine(m)
            .with_interceptor_registry(registry)
            .build();

        if let WebrtcAddr::Bootstrap(bs) = bootstrap {
            let pc = api
                .new_peer_connection(RTCConfiguration {
                    ice_servers: bs,
                    ..Default::default()
                })
                .await?;

            let (dc_tx, dc_rx) = unbounded();
            let (addr_tx, addr_rx) = unbounded();

            pc.on_data_channel(Box::new(move |d| {
                log::debug!("Receive data channel {} from remote", d.label());

                if let Err(e) = dc_tx.try_send(d) {
                    log::error!("Got error when send data channel: {:?}", e);
                }

                Box::pin(async move {})
            }))
            .await;

            let atc = addr_tx.clone();

            pc.on_ice_candidate(Box::new(move |ice| {
                let atc = addr_tx.clone();

                Box::pin(async move {
                    if let Some(i) = ice {
                        if let Ok(iii) = i.to_json().await {
                            if let Ok(e) = atc.try_send(WebrtcAddr::ICE(iii)) {
                                log::error!("Got error when send ice: {:?}", e);
                            }
                        }
                    }
                })
            }))
            .await;

            let s = Self {
                pc: Arc::new(pc),
                dc_rx,
                addr_tx: atc,
                addr_rx,
            };

            Ok(s)
        } else {
            Err(Error::ErrAddrType)
        }
    }

    async fn start(&self) -> Result<()> {
        let sdp = self.pc.create_offer(None).await?;
        self.pc.set_local_description(sdp.clone()).await?;
        if let Err(e) = self.addr_tx.send(WebrtcAddr::SDP(sdp)).await {
            log::error!("Send to channel addr_tx failed: {:?}", e);
            Err(Error::ErrChannelClosed)
        } else {
            Ok(())
        }
    }

    async fn connect(&self, label: WebrtcAddr) -> Result<WebrtcStream> {
        if let WebrtcAddr::Label(s) = label {
            let dc = self.pc.create_data_channel(&s, None).await?;
            let (data_tx, data_rx) = unbounded();
            dc.on_message(Box::new(move |m| {
                if let Err(e) = data_tx.try_send(m.data) {
                    log::error!("Got error when send data: {:?}", e);
                }
                Box::pin(async move {})
            }))
            .await;
            Ok(WebrtcStream { dc, data_rx })
        } else {
            Err(Error::ErrAddrType)
        }
    }

    async fn accept(&self) -> Result<WebrtcStream> {
        let dc = self.dc_rx.recv().await?;

        let (data_tx, data_rx) = unbounded();

        dc.on_message(Box::new(move |m| {
            if let Err(e) = data_tx.try_send(m.data) {
                log::error!("Got error when send data: {:?}", e);
            }
            Box::pin(async move {})
        }))
        .await;

        Ok(WebrtcStream { dc, data_rx })
    }

    async fn fetch_local_addr(&self) -> Result<WebrtcAddr> {
        let addr = self.addr_rx.recv().await?;
        Ok(addr)
    }

    async fn set_remote_addr(&self, remote: WebrtcAddr) -> Result<()> {
        match remote {
            WebrtcAddr::SDP(s) => {
                self.pc.set_remote_description(s).await?;
            }
            WebrtcAddr::ICE(i) => self.pc.add_ice_candidate(i).await?,
            _ => return Err(Error::ErrAddrType),
        }

        Ok(())
    }
}

impl P2pSocket for WebrtcSocket {
    type Addr = WebrtcAddr;

    type Stream = WebrtcStream;

    type Error = Error;

    fn poll_bind(cx: &mut Context<'_>, bootstrap: Self::Addr) -> Poll<Result<Self>> {
        let mut fu = Box::pin(async move { WebrtcSocket::bind(bootstrap).await });

        fu.poll(cx)
    }

    fn poll_connect(
        self: Pin<&Self>,
        cx: &mut Context<'_>,
        label: Self::Addr,
    ) -> Poll<Result<Self::Stream>> {
        let mut fu = Box::pin(async move { self.connect(label).await });

        fu.poll(cx)
    }
}
