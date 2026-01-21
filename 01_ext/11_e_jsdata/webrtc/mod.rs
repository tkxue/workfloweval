#[allow(unused_imports)]
use super::*;

/*
const g_webrtc_packet_size: usize = 1200;

#[derive(Debug)]
pub struct Webrtc_Packet {
    len: usize,
    pub data: Box<[u8; g_webrtc_packet_size]>,}

impl Webrtc_Packet {
    pub fn as_slice(&self) -> &[u8] {
        &self.data[0..self.len]}

    pub fn len(&self) -> usize {
        self.len}

    pub fn new(data: &[u8]) -> Webrtc_Packet {
        let mut ans = Webrtc_Packet::default();
        let len = data.len().clamp(0, g_webrtc_packet_size);
        ans.len = len;
        ans.data[0..len].copy_from_slice(data);
        ans}

    pub fn truncate(&mut self, n: usize) {
        self.len = n;}}

impl Default for Webrtc_Packet {
    fn default() -> Self {
        Webrtc_Packet {
            len: 0,
            data: Box::new([0_u8; g_webrtc_packet_size]),}}}


 */
