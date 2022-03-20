use std::{
    pin::Pin,
    task::{Context, Poll},
};

use futures_lite::Future;

use crate::P2pSocket;

pub struct StartFuture<'a, T: P2pSocket> {
    pub socket: &'a T,
}

impl<'a, T> Future for StartFuture<'a, T>
where
    T: P2pSocket + Unpin,
    T::Addr: Unpin,
{
    type Output = Result<(), T::Error>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self;

        let socket = &*this.socket;

        Pin::new(socket).poll_start(cx)
    }
}
