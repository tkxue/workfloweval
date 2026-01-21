use super::*;
use async_oneshot::Receiver;
use js_sys::Array;
use serde::{Deserializer, Serialize, Serializer};
use web_common::Id_Proc;

#[wasm_bindgen]
pub struct G_CmsgQ {
    id_proc: Id_Proc,
    outgoing: js_sys::Function,
    incoming: MsgQueue<Cmsg_Full>,
    async_cond_var: AsyncCondVar,
    next_free_id: Cell<u128>,
    handle_reply: ArcState<
        HashMap<Cmsg_MailboxId, Box<dyn FnOnce(Cmsg_MailboxId, Cmsg_Inner)>>,
    >,
}

thread_local! {
  static _G_CmsgQ: OnceCell<G_CmsgQ> = OnceCell::new();
}

#[wasm_bindgen]
impl G_CmsgQ {
    pub fn hello() -> String {
        "Hello World".to_string()
    }

    pub fn init(name: &str, send: &js_sys::Function) {
        let send = send.clone();

        let id_proc = match name {
            "h_index" => Id_Proc::H_Index,
            "h_gfx" => Id_Proc::H_Gfx,
            "h_vid" => Id_Proc::H_Vid,
            "ww_net" => Id_Proc::Ww_Net,
            "ww_rune" => Id_Proc::Ww_Rune,
            "ww_python" => Id_Proc::Ww_Python,
            "ww_sqlite" => Id_Proc::Ww_Sqlite,
            "ww_sheet" => Id_Proc::Ww_Sheet,
            _ => damn_it!("unknown name: {}", name),
        };

        let _ = _G_CmsgQ.with(|cb| {
            if cb.get().is_none() {
                cb.set(G_CmsgQ {
                    id_proc,
                    incoming: MsgQueue::new(),
                    outgoing: send,
                    async_cond_var: AsyncCondVar::new(),
                    next_free_id: Cell::new(0),
                    handle_reply: ArcState::new(HashMap::new()),
                })
                .map_err(|_| format!("G_Cmsg0::init .set error on {}", name))
                .unwrap();
            }
        });
    }

    pub fn push_msg(x: &JsValue) {
        let _ = _G_CmsgQ.with(|cb| {
            let obj = cb.get().unwrap();
            let typed = Cmsg_Full::from_js_value(x);
            match typed {
                Ok(v) => {
                    if let Cmsg_Type::ReplyTo(mailbox_id) = v.msg_type {
                        let t =
                            obj.handle_reply.update(|x| x.remove(&mailbox_id));
                        match t {
                            None => {
                                wlog!(
                                    "replying to empty mailbox: {:?} {:?}",
                                    mailbox_id,
                                    v.inner
                                );
                            }
                            Some(x) => (x)(mailbox_id, v.inner),
                        }
                    } else {
                        obj.incoming.extend(vec![v]);
                        obj.async_cond_var.notify();
                    }
                }
                Err(e) => {
                    wlog!("parsing error: {:?}", e);
                    web_sys::console::log_1(x)
                }
            }
        });
    }
}

impl G_CmsgQ {
    pub fn id_proc() -> Id_Proc {
        _G_CmsgQ.with(|cb| cb.get().unwrap().id_proc)
    }

    pub async fn wait_on() {
        let obj = _G_CmsgQ.with(|cb| cb.get().unwrap().async_cond_var.clone());
        obj.wait_on().await;
    }

    pub fn take_all() -> Vec<Cmsg_Full> {
        _G_CmsgQ.with(|cb| cb.get().unwrap().incoming.take_all())
    }

    pub fn send_jsvalue(t: JsValue) {
        _G_CmsgQ.with(|cb| {
            let js_func: &Function = &cb.get().unwrap().outgoing; // cb.get().unwrap().outgoing.as_ref().unchecked_ref();
            js_func.call1(&JsValue::NULL, &t).unwrap();
        })
    }

    pub fn send_cmsg_full(t: Cmsg_Full) {
        if t.inner.id_proc() == G_CmsgQ::id_proc() {
            _G_CmsgQ.with(|cb| {
                let obj = cb.get().unwrap();
                obj.incoming.extend(vec![t]);
                obj.async_cond_var.notify();
            })
        } else {
            G_CmsgQ::send_jsvalue(t.to_js_value())
        }
    }

    pub fn take_mailbox() -> Cmsg_MailboxId {
        _G_CmsgQ.with(|cb| {
            let obj = cb.get().unwrap();
            let idx = obj.next_free_id.get();
            obj.next_free_id.set(idx + 1);
            Cmsg_MailboxId {
                id_proc: obj.id_proc,
                idx,
            }
        })
    }

    pub fn send_oneshot(t: Cmsg_Inner) {
        Self::send_cmsg_full(Cmsg_Full {
            inner: t,
            msg_type: Cmsg_Type::Oneshot,
        });
    }

    pub fn send_reply(mailbox_id: Cmsg_MailboxId, t: Cmsg_Inner) {
        Self::send_cmsg_full(Cmsg_Full {
            inner: t,
            msg_type: Cmsg_Type::ReplyTo(mailbox_id),
        });
    }

    pub fn send_need_reply<T: 'static, F: FnOnce(Cmsg_Inner) -> T + 'static>(
        t: Cmsg_Inner,
        f: F,
    ) -> Receiver<Result<(Cmsg_MailboxId, T), ()>> {
        let mailbox_id = Self::take_mailbox();
        Self::send_cmsg_full(Cmsg_Full {
            inner: t,
            msg_type: Cmsg_Type::NeedReply(mailbox_id),
        });
        let (mut sender, receiver) = async_oneshot::oneshot();

        _G_CmsgQ.with(move |cb| {
            let obj = cb.get().unwrap();
            obj.handle_reply.update(move |x| {
                x.insert(
                    mailbox_id,
                    Box::new(
                        move |mailbox_id: Cmsg_MailboxId, inner: Cmsg_Inner| {
                            let inner = f(inner);
                            let _ = sender.send(Ok((mailbox_id, inner)));
                        },
                    ),
                );
            })
        });

        receiver
    }
}
