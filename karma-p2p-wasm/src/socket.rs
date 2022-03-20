use std::{
    pin::Pin,
    task::{Context, Poll, Waker}, collections::VecDeque,
    mem, rc::Rc, cell::RefCell
};

use futures_lite::FutureExt;
use js_sys::Reflect;
use karma_p2p::P2pSocket;
use wasm_bindgen::{prelude::Closure, JsCast, JsValue};
use wasm_bindgen_futures::JsFuture;
use web_sys::{RtcConfiguration, RtcPeerConnection, RtcDataChannelInit, RtcPeerConnectionIceEvent, RtcSessionDescriptionInit, RtcSdpType};

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

pub struct WebrtcSocket {
    pc: RtcPeerConnection,
    inner: Rc<RefCell<AddressFutureInner>>,
}

impl WebrtcSocket {
    async fn bind(bootstrap: WebrtcAddr) -> Result<Self> {
        if let WebrtcAddr::Bootstrap(addr) = bootstrap {
            let ice_servers = JsValue::from_serde(&addr)?;

            let mut config = RtcConfiguration::new();
            config.ice_servers(&ice_servers);

            let pc = RtcPeerConnection::new_with_configuration(&config)?;

            let inner = Rc::new(RefCell::new(AddressFutureInner::default()));

            let inner_clone = inner.clone();

            let on_ice_candidate = 
                Closure::wrap(Box::new(move |ev: RtcPeerConnectionIceEvent| {
                    if let Some(candidate) = ev.candidate() {
                        let inner = inner_clone.clone();
                        {
                            let mut re = inner.borrow_mut();
                            let addr = WebrtcAddr::ICE(candidate);
                            re.address.push_back(addr);

                            let waker = mem::replace(&mut re.waker, None);

                            if let Some(waker) = waker {
                                waker.wake();
                            }
                        }
                    }
                }) as Box<dyn FnMut(RtcPeerConnectionIceEvent)>);

            pc.set_onicecandidate(Some(on_ice_candidate.as_ref().unchecked_ref()));

            Ok(WebrtcSocket {
                pc,
                inner,
            })
        } else {
            Err(Error::ErrAddrType)
        }
    }

    async fn connect(&self, label: WebrtcAddr, port: u16) -> Result<WebrtcStream> {
        if let WebrtcAddr::Label(label) = label {
            let mut dc_init = RtcDataChannelInit::new();

            dc_init.id(port).negotiated(true);

            let dc = self.pc.create_data_channel_with_data_channel_dict(&label, &dc_init);

//             let on_message =
            //     Closure::wrap(Box::new(move |ev: MessageEvent| {}) as Box<dyn FnMut(MessageEvent)>);
            //
            // dc.set_onmessage(Some(on_message.as_ref().unchecked_ref()));
//
            Ok(WebrtcStream { dc })
        } else {
            Err(Error::ErrAddrType)
        }
    }

    async fn start(&mut self) -> Result<()> {
        let offer = JsFuture::from(self.pc.create_offer()).await?;
        let offer_sdp = Reflect::get(&offer, &JsValue::from_str("sdp"))?.as_string().unwrap();

        let mut offer_obj = RtcSessionDescriptionInit::new(RtcSdpType::Offer);
        offer_obj.sdp(&offer_sdp);

        JsFuture::from(self.pc.set_local_description(&offer_obj)).await?;

        let addr = WebrtcAddr::SDP(offer_obj);
        {
            let mut re = self.inner.borrow_mut();
            re.address.push_back(addr);

            let waker = mem::replace(&mut re.waker, None);

            if let Some(waker) = waker {
                waker.wake();
            }
        }

        Ok(())
    }

    async fn set_remote_addr(&self, remote: WebrtcAddr) -> Result<()> {
        match remote {
            WebrtcAddr::SDP(s) => {
                JsFuture::from(self.pc.set_remote_description(&s)).await?;
            }
            WebrtcAddr::ICE(ice) => {},
            _ => {}
        }

        Ok(())
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

    fn poll_connect(
        self: std::pin::Pin<&Self>,
        cx: &mut Context<'_>,
        label: Self::Addr,
        port: u16,
    ) -> Poll<Result<Self::Stream>> {
        let mut fu = Box::pin(async move { self.connect(label, port).await });

        fu.poll(cx)
    }

    fn poll_start(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<()>> {
        let mut this = self;

        let mut fu = Box::pin(async move { this.start().await });

        fu.poll(cx)
    }

    fn poll_fetch_local_addr(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<Self::Addr>> {
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
        let mut fu = Box::pin(async move { self.set_remote_addr(remote).await });

        fu.poll(cx)
    }
}
