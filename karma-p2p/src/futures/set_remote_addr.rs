use std::{
    mem,
    pin::Pin,
    task::{Context, Poll},
};

use futures_lite::Future;

use crate::P2pSocket;

pub struct SetRemoteAddr<'a, T: P2pSocket> {
    pub socket: &'a T,
    pub remote: Option<T::Addr>,
}

impl<'a, T> Future for SetRemoteAddr<'a, T>
where
    T: P2pSocket + Unpin,
    T::Addr: Unpin,
{
    type Output = Result<(), T::Error>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut this = self;

        let opt_remote = mem::replace(&mut this.remote, None);
        let socket = &*this.socket;

        Pin::new(socket).poll_set_remote_addr(cx, opt_remote.unwrap())
    }
}
