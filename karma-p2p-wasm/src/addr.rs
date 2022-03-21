use serde::{Deserialize, Serialize};
use web_sys::{RtcIceCandidate, RtcSessionDescriptionInit};

#[derive(Serialize, Deserialize, Debug, Clone)]
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
