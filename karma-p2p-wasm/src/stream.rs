use std::{
    pin::Pin,
    task::{Context, Poll, Waker}, rc::Rc, cell::RefCell, collections::VecDeque, mem,
};

use futures_lite::{AsyncRead, AsyncWrite};
use js_sys::Uint8Array;
use karma_p2p::P2pStream;
use wasm_bindgen::{prelude::Closure, JsCast};
use web_sys::{RtcDataChannel, MessageEvent};

pub struct ReadFutureInner {
    pub waker: Option<Waker>,
    pub data: VecDeque<Vec<u8>>,
}

impl Default for ReadFutureInner {
    fn default() -> Self {
        Self {
            waker: None,
            data: VecDeque::new()
        }
    }
}

pub struct WebrtcStream {
    pub(crate) dc: RtcDataChannel,
    pub(crate) inner: Rc<RefCell<ReadFutureInner>>,
}

impl WebrtcStream {
    pub fn new(dc: RtcDataChannel) -> Self {
        Self {
            dc,
            inner: Rc::new(RefCell::new(Default::default()))
        }
    }
}

impl WebrtcStream {
    pub fn init(&self) {

        let inner = self.inner.clone();

        let on_message =
            Closure::wrap(Box::new(move |ev: MessageEvent| {
                let data = Uint8Array::new(&ev.data());

                let data_vec = data.to_vec();

                let mut re = inner.borrow_mut();

                re.data.push_back(data_vec);

                let waker = mem::replace(&mut re.waker, None);

                if let Some(waker) = waker {
                    waker.wake();
                }

            }) as Box<dyn FnMut(MessageEvent)>);

        self.dc.set_onmessage(Some(on_message.as_ref().unchecked_ref()));
    }

    pub fn write(&self, buf: &[u8]) -> std::io::Result<usize> {
        let size = buf.len();
        if let Err(e) = self.dc.send_with_u8_array(buf) {
            let value: String = e.into_serde()?;
            return Err(std::io::Error::new(std::io::ErrorKind::Other, value));
        }
        Ok(size)
    }

    pub fn close(&self) -> std::io::Result<()> {
        self.dc.close();
        Ok(())
    }
}

impl AsyncRead for WebrtcStream {
    fn poll_read(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut [u8],
    ) -> Poll<std::io::Result<usize>> {
        let mut re = self.inner.borrow_mut();

        let _ = mem::replace(&mut re.waker, Some(cx.waker().clone()));

        if let Some(data) = re.data.pop_front() {
            let size = data.len();
            buf.copy_from_slice(&data);
            Poll::Ready(Ok(size))
        } else {
            Poll::Pending
        }
    }
}

impl AsyncWrite for WebrtcStream {
    fn poll_write(
        self: Pin<&mut Self>,
        _cx: &mut Context<'_>,
        buf: &[u8],
    ) -> Poll<std::io::Result<usize>> {
        // TODO: process error of send
        Poll::Ready(self.write(buf))
    }

    fn poll_close(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<std::io::Result<()>> {
        Poll::Ready(self.close())
    }

    fn poll_flush(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<std::io::Result<()>> {
        Poll::Ready(Ok(()))
    }
}

impl P2pStream for WebrtcStream {}
