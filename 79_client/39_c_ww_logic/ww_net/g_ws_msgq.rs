use e_api::js_sys::Uint8Array;

use super::*;

#[wasm_bindgen]
pub struct G_WsMsgQ {
    outgoing: js_sys::Function,
    next_free_id: Cell<u128>,
    handle_reply: ArcState<HashMap<u128, Box<dyn FnOnce(Result<N_ToC_Full, Sa_Err>)>>>,
    incoming: MsgQueue<Result<N_ToC_Full, Sa_Err>>,
    async_cond_var: AsyncCondVar,
}

thread_local! {
  static _G_WsMsgQ: OnceCell<G_WsMsgQ> = OnceCell::new();
}

#[wasm_bindgen]
impl G_WsMsgQ {
    pub fn init(send: &js_sys::Function) {
        let send = send.clone();
        let _ = _G_WsMsgQ.with(|cb| {
            if cb.get().is_none() {
                cb.set(G_WsMsgQ {
                    outgoing: send,
                    next_free_id: Cell::new(0),
                    handle_reply: ArcState::new(HashMap::new()),
                    incoming: MsgQueue::new(),
                    async_cond_var: AsyncCondVar::new(),
                })
                .map_err(|_| ())
                .unwrap();
            }
        });
    }

    pub fn push_msg(t: JsValue) {
        let msg = {
            let data = js_sys::Reflect::get(&t, &JsValue::from_str("data")).unwrap();
            let t2: Vec<u8> = Uint8Array::new(&data).to_vec();
            L_JsData_Util::read_obj::<Result<N_ToC_Full, Sa_Err>>(t2.as_ref()).unwrap()
        };
        match msg {
            Err(msg) => {}
            Ok(ref msg_full) => match msg_full.mailbox {
                N_MailboxId_C::OneShot => _G_WsMsgQ.with(|obj| {
                    let obj = obj.get().unwrap();
                    obj.incoming.extend(vec![msg]);
                    obj.async_cond_var.notify();
                }),
                N_MailboxId_C::ReplyTo(idx) => {
                    let handler = _G_WsMsgQ.with(|obj| obj.get().unwrap().handle_reply.update(|obj| obj.remove(&idx)));
                    match handler {
                        Some(handler) => (handler)(msg),
                        None => wlog!("mailbox non existent: {:?}", idx),
                    }
                }
            },
        }
    }
}

impl G_WsMsgQ {
    pub async fn wait_on() {
        let obj = _G_WsMsgQ.with(|cb| cb.get().unwrap().async_cond_var.clone());
        obj.wait_on().await;
    }

    pub fn take_all() -> Vec<Result<N_ToC_Full, Sa_Err>> {
        _G_WsMsgQ.with(|obj| obj.get().unwrap().incoming.take_all())
    }

    pub fn take_id() -> u128 {
        let t = _G_WsMsgQ.with(|cb| {
            let obj = cb.get().unwrap();
            let t = obj.next_free_id.get();
            obj.next_free_id.set(t + 1);
            t
        });
        t
    }

    pub fn send_oneshot(aux: N_ToS_Aux, inner: N_ToS_Inner) {
        Self::send_full(N_ToS_Full {
            aux,
            mailbox: N_MailboxId_S::OneShot,
            inner,
        })
    }

    fn send_full(t: N_ToS_Full) {
        let t = L_JsData_Util::obj_to_bytes(&t);
        let t = js_sys::Uint8Array::from(t.as_slice());
        let t: JsValue = t.into();
        Self::send_raw(t);
    }

    pub fn send_raw(t: JsValue) {
        _G_WsMsgQ.with(|cb| {
            let js_func: &js_sys::Function = &cb.get().unwrap().outgoing;
            js_func.call1(&JsValue::NULL, &t).unwrap();
        })
    }

    pub fn send_need_reply(
        aux: N_ToS_Aux,
        inner: N_ToS_Inner,
    ) -> async_oneshot::Receiver<Result<Result<N_ToC_Full, Sa_Err>, ()>> {
        let mailbox_id = Self::take_id();
        Self::send_full(N_ToS_Full {
            aux,
            mailbox: N_MailboxId_S::NeedReply(mailbox_id),
            inner,
        });

        let (mut sender, receiver) = async_oneshot::oneshot();

        _G_WsMsgQ.with(move |cb| {
            let obj = cb.get().unwrap();
            obj.handle_reply.update(move |x| {
                x.insert(
                    mailbox_id,
                    Box::new(move |inner| {
                        let _ = sender.send(Ok(inner));
                    }),
                );
            })
        });

        receiver
    }
}
