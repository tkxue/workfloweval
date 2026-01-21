use super::*;
use std::collections::VecDeque;

pub struct Cstate_LogViewer {
    pub entrys: CircularVec<C_LogEntry>,
}

impl Cstate_LogViewer {
    pub fn new() -> Cstate_LogViewer {
        Cstate_LogViewer {
            entrys: CircularVec::new(10_000),
        }
    }

    pub fn process(&mut self, msg: Cmsg_LogViewer) {
        match msg {
            Cmsg_LogViewer::LogViewer(x) => {
                self.entrys.push(x);
            }
        }
    }
}
