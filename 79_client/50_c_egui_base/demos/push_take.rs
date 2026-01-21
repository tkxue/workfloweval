use super::*;

pub struct Push_Take<Msg> {
    msgs: Vec<Msg>,
}

impl<Msg> Push_Take<Msg> {
    pub fn new() -> Push_Take<Msg> {
        Push_Take { msgs: vec![] }
    }

    pub fn push(&mut self, t: Msg) {
        self.msgs.push(t);
    }

    pub fn take(self) -> Vec<Msg> {
        self.msgs
    }
}
