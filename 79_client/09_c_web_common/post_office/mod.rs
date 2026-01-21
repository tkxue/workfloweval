use super::*;

mod msg_0;
mod msg_code;
pub use msg_0::*;
pub use msg_code::*;

/*
pub struct PO {
    /*
    _incoming_ports: Mutex<Incoming_Ports>,
    receiver: Arc<Keep_Process<(Id_Proc, XdomA_Message_Event)>>,
    pub senders: Arc<Msg_Senders>,
    pub actors: ArcState<HashMap<Id_GActor, CbArc<Xos_Msg>>>,
    pub id_proc: Id_Proc,
    pub is_dev: bool,

     */
    pub web_root: Arc<Web_Root>,
}

impl PO {
    /*
    pub fn log(msg: eetf::Term) {
        let po = PO::__get_singleton();

        if !po.is_dev {
            return;
        }

        let x = Term::Map(eetf::Map {
            map: vec![
                (
                    Term::Atom(eetf::Atom {
                        name: "timestamp".to_string(),
                    }),
                    Term::Float(eetf::Float {
                        value: XdomA::now_f64_milli() / 1000.,
                    }),
                ),
                (
                    Term::Atom(eetf::Atom {
                        name: "proc_name".to_string(),
                    }),
                    Term::Atom(eetf::Atom {
                        name: format!("{:?}", PO::__get_singleton().id_proc),
                    }),
                ),
                (
                    Term::Atom(eetf::Atom {
                        name: "proc_inner_cnt".to_string(),
                    }),
                    Term::Float(eetf::Float {
                        value: XdomA::browser_log_msg_cnt__get_and_inc() as f64,
                    }),
                ),
                (
                    Term::Atom(eetf::Atom {
                        name: "msg".to_string(),
                    }),
                    msg,
                ),
            ]
                .into_iter()
                .collect::<HashMap<_, _>>(),
        });

        let url = format!("http://{}/e/esi_browser_log", po.web_root.host);

        // wlog!("posting to: {}", url);

        let mut buf = Vec::new();
        x.encode(&mut buf).unwrap();
        let t = XdomA::post_request(url.as_str(), buf.as_slice());
    }
    */

    pub fn reload_logic() {
        damn_it!("")
        // PO::__get_singleton().reload_logic_inner()
    }

    pub fn reload_logic_inner(&self) {
        damn_it!("")
        /*
        self.senders.reset_hotreload();
        let mut g = self._incoming_ports.lock().unwrap();
        for id_proc in Id_Proc::iter() {
            if id_proc.is_hot_reload() {
                let (a, b) = XdomA_Message_Port::new_message_channel();
                g.set(id_proc, a);
                let msg = Msg_0::Rust_Msg(Msg_0_Rust::Send_One_Message_Port {
                    from_proc_id: self.id_proc,
                    dst: id_proc,
                    port: b,
                });
                XdomA::parent_post_message_with_transfer(&msg.to_js(), &msg.transfers()).un(err!(""));
            }
        }
        */
    }

    pub fn call(&self, t: Msg_Po) {
        // self.senders.send(t);
        todo!()
    }

    pub fn call_t(t: &impl To_Msg_Po_T) {
        damn_it!("")
        // PO::__get_singleton().senders.send(t.to_msg_po())
    }

    pub fn register<T: Actor_Config_T<'static> + 'static>(&self, label: &'static str, actor: Arc_Ac<T>) {
        todo!()
        /*
        let actors = self.actors.clone();
        let capp_id = T::Remote_In::id_gactor();
        let cbrc = CbArc::new(Rc::new(move |x: Xos_Msg| {
            let t = x.to_typed::<T::Remote_In>();
            match t {
                Ok(v) => {
                    actor.queue_remote(v);
                    actor.tick_once(label);
                }
                Err(e) => {
                    wlog!("error decoding mst to {:?}\n{:?}", T::Remote_In::id_gactor(), e)
                }
            }
        }));

        actors.apply(&|x| {
            x.insert(capp_id, cbrc.clone());
        })
        */
    }

    fn set_global(self: &Arc<Self>) {
        /*
        let po_cbs = self.clone();
        PO::__set_singleton(Arc::new(move || po_cbs.clone()));

         */
        todo!()
    }
}

impl PO {
    pub fn set_recv_func(&self, f: Rc<dyn Fn((Id_Proc, Result<Msg_Po, Js_Parse_Error>))>) {
        /*
        let f = Rc::new(move |(proc_id, v)| (f.as_ref())((proc_id, Jsv::message_event_to_typed(v))))
            as Rc<dyn Fn((Id_Proc, XdomA_Message_Event))>;
        self.receiver.set_recv_func(f);

         */
        damn_it!("")
    }

    fn actor_process(actors: &Rc2<HashMap<Id_GActor, CbArc<Xos_Msg>>>, sender: Id_Proc, dst: Id_Proc, id_gactor: Id_GActor, msg: Xos_Msg) {
        let t = actors.get(&|x| x.get(&id_gactor).cloned());
        match t {
            None => {
                wlog!("missing id_gactor:-\nSender: {:?}\nId_Proc: {:?}\nId_GActor {:?}", sender, dst, id_gactor);
            }
            Some(f) => {
                f.call(msg);
            }
        }
    }

    pub fn register_cb(&self) {
        todo!()
        /*
        self.receiver.set_recv_func(Rc::new({
            let senders = self.senders.clone();
            let actors = self.actors.clone();
            move |(sender, y)| {
                let y = Jsv::message_event_to_typed::<Msg_Po>(y);
                match y {
                    Ok(v) => match v.inner {
                        Msg_Po_Inner::Send_Sender((id, proc)) => {
                            senders.set_sender(id, proc);
                        }
                        Msg_Po_Inner::Actor(id_gactor, msg) => {
                            Self::actor_process(&actors, sender, v.dst, id_gactor, msg);
                        }
                    },
                    Err(e) => {
                        wlog!("err: {:?} {:?}", sender, e.error);
                    }
                }
            }
        }));
        */
    }

    pub fn new(id_proc__self: Id_Proc, web_root: Arc<Web_Root>) -> Arc<PO> {
        Arc::new(PO { web_root: web_root.clone() })
        /*
        let actors = Rc2::new(HashMap::<Id_GActor, CbArc<Xos_Msg>>::new());

        let receiver = Rc::new(Keep_Process::new());

        let f = Rc::new({
            let r = receiver.clone();
            move |proc_id, x| r.process((proc_id, x /* Jsv::message_event_to_typed::<T>(x) */))
        });

        let mut incoming_ports = Incoming_Ports::new(f);

        // wlog!("Postoffice::new {:?}", id_proc__self);
        {
            for id_proc in Id_Proc::iter() {
                let (a, b) = XdomA_Message_Port::new_message_channel();
                incoming_ports.set(id_proc, a);
                let msg = Msg_0::Rust_Msg(Msg_0_Rust::Send_One_Message_Port {
                    from_proc_id: id_proc__self,
                    dst: id_proc,
                    port: b,
                });
                XdomA::parent_post_message_with_transfer(&msg.to_js(), &msg.transfers()).un(err!(""));
            }
        }

        let senders = Rc::new(Msg_Senders::new(id_proc__self));

        let po = Arc::new(PO {
            _incoming_ports: Mutex::new(incoming_ports),
            receiver,
            senders: senders,
            actors: actors.clone(),
            id_proc: id_proc__self,
            is_dev: web_root.is_dev,
        });

        po.set_global();

        po
        */
    }
}

pub static _G_PO: OnceLock<Arc<PO>> = OnceLock::new();

/*
mm_lazy_singleton! {
    PO ;
    Arc< PO > ;
    __set_singleton ;
    __get_singleton ;
}


 */



*/
