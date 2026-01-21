use super::*;

#[derive(Serialize, Deserialize, JsData, Debug)]
pub enum N_MailboxId_S {
    OneShot,
    NeedReply(u128),
}

impl N_MailboxId_S {
    pub fn to_client(&self) -> N_MailboxId_C {
        match self {
            N_MailboxId_S::OneShot => N_MailboxId_C::OneShot,
            N_MailboxId_S::NeedReply(x) => N_MailboxId_C::ReplyTo(*x),
        }
    }
}

#[derive(Serialize, Deserialize, JsData, Debug)]
pub struct N_ToS_Aux {}

#[derive(Serialize, Deserialize, JsData, Debug)]
pub struct N_ToS_Full {
    pub aux: N_ToS_Aux,
    pub mailbox: N_MailboxId_S,
    pub inner: N_ToS_Inner,
}

#[derive(Serialize, Deserialize, JsData, Debug)]
pub enum N_ToS_Inner {
    Counter(N_Counter_ToS),
}


