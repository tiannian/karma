use crate::{
    futures::{BindFuture, ConnectFuture, FetchLocalAddrFuture, SetRemoteAddr, StartFuture},
    P2pSocket,
};

pub trait P2pSocketExt: P2pSocket {
    fn bind(bootstrap: Self::Addr) -> BindFuture<Self> {
        BindFuture {
            arg: Some(bootstrap),
        }
    }

    fn connect(&self, label: Self::Addr, port: u16) -> ConnectFuture<'_, Self> {
        ConnectFuture {
            socket: self,
            label: Some(label),
            port,
        }
    }

    fn start(&mut self) -> StartFuture<'_, Self> {
        StartFuture { socket: self }
    }

    fn fetch_local_addr(&mut self) -> FetchLocalAddrFuture<'_, Self> {
        FetchLocalAddrFuture { socket: self }
    }

    fn set_remote_addr(&self, remote: Self::Addr) -> SetRemoteAddr<'_, Self> {
        SetRemoteAddr {
            socket: self,
            remote: Some(remote),
        }
    }
}
