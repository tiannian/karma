use std::{
    cell::RefCell,
    collections::VecDeque,
    mem,
    pin::Pin,
    rc::Rc,
    task::{Context, Poll, Waker},
};

use futures_lite::FutureExt;
use js_sys::Reflect;
use karma_p2p::P2pSocket;
use wasm_bindgen::{prelude::Closure, JsCast, JsValue};
use wasm_bindgen_futures::JsFuture;
use web_sys::{
    RtcConfiguration, RtcDataChannelInit, RtcPeerConnection, RtcPeerConnectionIceEvent, RtcSdpType,
    RtcSessionDescriptionInit,
};

use crate::{Error, Result, WebrtcAddr, WebrtcStream};

struct AddressFutureInner {
    pub waker: Option<Waker>,
    pub address: VecDeque<WebrtcAddr>,
}

impl Default for AddressFutureInner {
    fn default() -> Self {
        Self {
            waker: None,
            address: VecDeque::new(),
        }
    }
}

impl AddressFutureInner {
    pub fn set_addr(&mut self, addr: WebrtcAddr) {
        self.address.push_back(addr);

        let waker = mem::replace(&mut self.waker, None);

        if let Some(waker) = waker {
            waker.wake();
        }
    }
}

pub struct WebrtcSocket {
    pc: RtcPeerConnection,
    inner: Rc<RefCell<AddressFutureInner>>,
}

impl WebrtcSocket {
    async fn _bind(bootstrap: WebrtcAddr) -> Result<Self> {
        if let WebrtcAddr::Bootstrap(addr) = bootstrap {
            let ice_servers = JsValue::from_serde(&addr)?;

            let mut config = RtcConfiguration::new();
            config.ice_servers(&ice_servers);

            let pc = RtcPeerConnection::new_with_configuration(&config)?;

            let inner = Rc::new(RefCell::new(AddressFutureInner::default()));

            let inner_clone = inner.clone();

            let on_ice_candidate = Closure::wrap(Box::new(move |ev: RtcPeerConnectionIceEvent| {
                if let Some(candidate) = ev.candidate() {
                    let inner = inner_clone.clone();
                    {
                        let mut re = inner.borrow_mut();
                        let addr = WebrtcAddr::ICE(candidate);

                        re.set_addr(addr);
                    }
                }
            })
                as Box<dyn FnMut(RtcPeerConnectionIceEvent)>);

            pc.set_onicecandidate(Some(on_ice_candidate.as_ref().unchecked_ref()));

            Ok(WebrtcSocket { pc, inner })
        } else {
            Err(Error::ErrAddrType)
        }
    }

    async fn _connect(&self, label: WebrtcAddr, port: u16) -> Result<WebrtcStream> {
        if let WebrtcAddr::Label(label) = label {
            let mut dc_init = RtcDataChannelInit::new();

            dc_init.id(port).negotiated(true);

            let dc = self
                .pc
                .create_data_channel_with_data_channel_dict(&label, &dc_init);

            let ws = WebrtcStream::new(dc);
            ws.init();

            Ok(ws)
        } else {
            Err(Error::ErrAddrType)
        }
    }

    async fn _start(&mut self) -> Result<()> {
        let offer = JsFuture::from(self.pc.create_offer()).await?;

        let offer_sdp = Reflect::get(&offer, &JsValue::from_str("sdp"))?
            .as_string()
            .unwrap();

        let mut offer_obj = RtcSessionDescriptionInit::new(RtcSdpType::Offer);
        offer_obj.sdp(&offer_sdp);

        JsFuture::from(self.pc.set_local_description(&offer_obj)).await?;

        let addr = WebrtcAddr::SDP(offer_obj);
        {
            let mut re = self.inner.borrow_mut();

            re.set_addr(addr);
        }

        Ok(())
    }

    async fn _set_remote_addr(&self, remote: WebrtcAddr) -> Result<()> {
        match remote {
            WebrtcAddr::SDP(s) => {
                JsFuture::from(self.pc.set_remote_description(&s)).await?;

                let answer = JsFuture::from(self.pc.create_answer()).await?;

                let sdp = Reflect::get(&answer, &JsValue::from_str("sdp"))?
                    .as_string()
                    .unwrap();

                let mut obj = RtcSessionDescriptionInit::new(RtcSdpType::Answer);
                obj.sdp(&sdp);

                JsFuture::from(self.pc.set_local_description(&obj)).await?;
            }
            WebrtcAddr::ICE(ice) => {
                JsFuture::from(
                    self.pc
                        .add_ice_candidate_with_opt_rtc_ice_candidate(Some(&ice)),
                )
                .await?;
            }
            _ => return Err(Error::ErrAddrType),
        }

        Ok(())
    }
}

impl P2pSocket for WebrtcSocket {
    type Error = Error;

    type Stream = WebrtcStream;

    type Addr = WebrtcAddr;

    fn poll_bind(cx: &mut Context<'_>, bootstrap: Self::Addr) -> Poll<Result<Self>> {
        let mut fu = Box::pin(async move { WebrtcSocket::_bind(bootstrap).await });

        fu.poll(cx)
    }

    fn poll_connect(
        self: std::pin::Pin<&Self>,
        cx: &mut Context<'_>,
        label: Self::Addr,
        port: u16,
    ) -> Poll<Result<Self::Stream>> {
        let mut fu = Box::pin(async move { self._connect(label, port).await });

        fu.poll(cx)
    }

    fn poll_start(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<()>> {
        let mut this = self;

        let mut fu = Box::pin(async move { this._start().await });

        fu.poll(cx)
    }

    fn poll_fetch_local_addr(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Result<Self::Addr>> {
        let mut re = self.inner.borrow_mut();

        let _ = mem::replace(&mut re.waker, Some(cx.waker().clone()));

        if let Some(addr) = re.address.pop_front() {
            Poll::Ready(Ok(addr))
        } else {
            Poll::Pending
        }
    }

    fn poll_set_remote_addr(
        self: Pin<&Self>,
        cx: &mut Context<'_>,
        remote: Self::Addr,
    ) -> Poll<Result<()>> {
        let mut fu = Box::pin(async move { self._set_remote_addr(remote).await });

        fu.poll(cx)
    }
}
