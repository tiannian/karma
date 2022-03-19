use crate::{futures::{BindFuture, ConnectFuture}, P2pSocket};

pub trait P2pSocketExt: P2pSocket {
    fn bind(bootstrap: Self::Addr) -> BindFuture<Self> {
        BindFuture {
            arg: Some(bootstrap),
        }
    }

    fn connect(&self, label: Self::Addr) -> ConnectFuture<'_, Self> {
        ConnectFuture {
            socket: self,
            label: Some(label)
        }
    }
}
