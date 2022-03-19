use std::{
    mem,
    pin::Pin,
    task::{Context, Poll},
};

use futures_lite::Future;

use crate::P2pSocket;

pub struct BindFuture<T: P2pSocket> {
    pub arg: Option<T::Addr>,
}

impl<T> Future for BindFuture<T>
where
    T: P2pSocket,
    T::Addr: Unpin,
{
    type Output = Result<T, T::Error>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut this = self;

        let opt_arg = mem::replace(&mut this.arg, None);

        T::poll_bind(cx, opt_arg.unwrap())
    }
}
