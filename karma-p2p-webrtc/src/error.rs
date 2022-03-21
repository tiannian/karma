#[derive(Debug)]
pub enum Error {
    ErrAddrType,
    ErrChannelClosed,
    WebrtcError(webrtc::Error),
}

impl From<smol::channel::RecvError> for Error {
    fn from(_: smol::channel::RecvError) -> Self {
        Error::ErrChannelClosed
    }
}

impl From<webrtc::Error> for Error {
    fn from(e: webrtc::Error) -> Self {
        Error::WebrtcError(e)
    }
}

pub type Result<T> = std::result::Result<T, Error>;
