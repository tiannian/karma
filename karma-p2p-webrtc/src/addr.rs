use webrtc::{
    ice_transport::{ice_candidate::RTCIceCandidateInit, ice_server::RTCIceServer},
    peer_connection::sdp::session_description::RTCSessionDescription,
};

#[derive(Debug)]
pub enum WebrtcAddr {
    Bootstrap(Vec<RTCIceServer>),
    SDP(RTCSessionDescription),
    ICE(RTCIceCandidateInit),
    Label(String),
}
