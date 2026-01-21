use super::*;

#[derive(Serialize, Deserialize, JsData, Debug)]
pub enum N_ToC_Inner {
    Counter(N_Counter_ToC),
}

#[derive(Serialize, Deserialize, JsData, Debug)]
pub struct N_ToC_Full {
    pub mailbox: N_MailboxId_C,
    pub inner: N_ToC_Inner,
}

#[derive(Serialize, Deserialize, JsData, Debug, Copy, Clone)]
pub enum N_MailboxId_C {
    OneShot,
    ReplyTo(u128),
}
