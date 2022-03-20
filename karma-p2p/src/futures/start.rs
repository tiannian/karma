use std::{
    pin::Pin,
    task::{Context, Poll},
};

use futures_lite::Future;

use crate::P2pSocket;

pub struct StartFuture<'a, T: P2pSocket> {
    pub socket: &'a mut T,
}

impl<'a, T> Future for StartFuture<'a, T>
where
    T: P2pSocket + Unpin,
    T::Addr: Unpin,
{
    type Output = Result<(), T::Error>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut this = self;

        let socket = &mut *this.socket;

        Pin::new(socket).poll_start(cx)
    }
}
