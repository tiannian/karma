use crate::{
    futures::{
        AcceptFuture, BindFuture, ConnectFuture, FetchLocalAddrFuture, SetRemoteAddr, StartFuture,
    },
    P2pSocket,
};

pub trait P2pSocketExt: P2pSocket {
    fn bind(bootstrap: Self::Addr) -> BindFuture<Self> {
        BindFuture {
            arg: Some(bootstrap),
        }
    }

    fn connect(&self, label: Self::Addr) -> ConnectFuture<'_, Self> {
        ConnectFuture {
            socket: self,
            label: Some(label),
        }
    }

    fn start(&self) -> StartFuture<'_, Self> {
        StartFuture { socket: self }
    }

    fn accept(&self) -> AcceptFuture<'_, Self> {
        AcceptFuture { socket: self }
    }

    fn fetch_local_addr(&self) -> FetchLocalAddrFuture<'_, Self> {
        FetchLocalAddrFuture { socket: self }
    }

    fn set_remote_addr(&self, remote: Self::Addr) -> SetRemoteAddr<'_, Self> {
        SetRemoteAddr {
            socket: self,
            remote: Some(remote),
        }
    }
}
