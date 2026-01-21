use super::*;

/*
#[derive(JsData)]
pub struct Msg_Po {
    pub dst: Id_Proc,
    pub inner: Msg_Po_Inner,
}

// pub type Per_Actor_Message_Port = (Id_Proc, XdomA_Message_Port); // Per_Proc<XdomA_Message_Port>;

#[derive(JsData)]
pub enum Msg_Po_Inner {
    Send_Sender(Per_Actor_Message_Port),
    Actor(Id_GActor, Xos_Msg),
}

pub trait To_Msg_Po_T: T_JsData_ + Sized {
    fn id_proc() -> Id_Proc;

    fn id_gactor() -> Id_GActor;

    fn to_msg_po(&self) -> Msg_Po {
        Msg_Po {
            dst: Self::id_proc(),
            inner: Msg_Po_Inner::Actor(Self::id_gactor(), conv_xos_raw_msg(self).unwrap()),
        }
    }
}

impl To_Msg_Po_T for () {
    fn id_proc() -> Id_Proc {
        todo!()
    }

    fn id_gactor() -> Id_GActor {
        todo!()
    }
}


 */
