use std::{
    pin::Pin,
    task::{Context, Poll},
};

use futures_lite::Future;

use crate::P2pSocket;

pub struct FetchLocalAddrFuture<'a, T: P2pSocket> {
    pub socket: &'a T,
}

impl<'a, T> Future for FetchLocalAddrFuture<'a, T>
where
    T: P2pSocket + Unpin,
    T::Addr: Unpin,
{
    type Output = Result<T::Addr, T::Error>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self;

        let socket = &*this.socket;

        Pin::new(socket).poll_fetch_local_addr(cx)
    }
}
