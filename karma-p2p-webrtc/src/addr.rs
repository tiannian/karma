use webrtc::{
    ice_transport::{ice_candidate::RTCIceCandidateInit, ice_server::RTCIceServer},
    peer_connection::sdp::session_description::RTCSessionDescription,
};

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum WebrtcAddr {
    #[serde(skip)]
    Bootstrap(Vec<RTCIceServer>),

    #[serde(skip)]
    Label(String),

    SDP(RTCSessionDescription),
    ICE(RTCIceCandidateInit),
}
