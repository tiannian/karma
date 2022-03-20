use std::{
    mem,
    pin::Pin,
    task::{Context, Poll},
};

use futures_lite::Future;

use crate::P2pSocket;

pub struct ConnectFuture<'a, T: P2pSocket> {
    pub socket: &'a T,
    pub label: Option<T::Addr>,
    pub port: u16,
}

impl<'a, T> Future for ConnectFuture<'a, T>
where
    T: P2pSocket + Unpin,
    T::Addr: Unpin,
{
    type Output = Result<T::Stream, T::Error>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut this = self;

        let opt_label = mem::replace(&mut this.label, None);
        let socket = &*this.socket;

        Pin::new(socket).poll_connect(cx, opt_label.unwrap(), this.port)
    }
}
