use serde::{Deserialize, Serialize};
use web_sys::{RtcSessionDescriptionInit, RtcIceCandidate};

#[derive(Serialize, Deserialize, Debug)]
pub enum CredentialType {
    #[serde(rename = "password")]
    Password,
    #[serde(rename = "oauth")]
    Oauth,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct IceServer {
    pub credential: String,
    #[serde(rename = "credentialType")]
    pub credential_type: CredentialType,
    pub urls: Vec<String>,
    pub username: String,
}

pub enum WebrtcAddr {
    Bootstrap(Vec<IceServer>),
    SDP(RtcSessionDescriptionInit),
    ICE(RtcIceCandidate),
    Label(String),
}
