use std::{task::{Context, Poll}, pin::Pin};

use futures_lite::Future;

use crate::P2pSocket;

pub trait P2pSocketExt {}

pub struct Bind<T: P2pSocket> {
    arg: T::Addr,
}

impl<T: P2pSocket> Future for Bind<T> {
    type Output = Result<T, T::Error>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let Self { arg } = *self;

        T::poll_bind(cx, arg)
    }
}

