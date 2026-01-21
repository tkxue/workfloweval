use super::*;
use crate::rune_repl::WwRune_Repl;
use web_common::XdomA;

#[derive(Clone)]
pub struct Ww_Rune {
    inner: ArcState<Ww_Rune_Inner>,
}

impl Ww_Rune {
    fn new() -> Ww_Rune {
        Ww_Rune {
            inner: ArcState::new(Ww_Rune_Inner {
                repl: WwRune_Repl::new().unwrap(),
            }),
        }
    }

    pub fn start_loop() {
        let obj = Arc::new(Self::new());
        XdomA::spawn_local(Box::pin(async move {
            wlog!("ww_rune: waiting");
            loop {
                G_CmsgQ::wait_on().await;
                let msgs = G_CmsgQ::take_all();
                for msg in msgs {
                    match msg.inner {
                        Cmsg_Inner::Ww_rune(v) => {
                            obj.handle_msg(v);
                        }
                        _ => {
                            wlog!("ww_rune: msg: wrong msg delivery")
                        }
                    }
                }
                wlog!("ww_rune: looping");
            }
        }));
    }

    pub fn handle_msg(&self, msg: Cmsg_WwRune) {
        wlog!("ww_rune: handle_msg: {:?}", msg);
        match msg {
            Cmsg_WwRune::ReplEval { cmd } => self.inner.update(|x| x.proc_msg(cmd)),
        }
    }
}

pub struct Ww_Rune_Inner {
    repl: WwRune_Repl,
}

impl Ww_Rune_Inner {
    pub fn proc_msg(&mut self, t: String) {
        match self.repl.eval_snippet(&t) {
            Ok(_) => {}
            Err(err) => {
                let msg = Cmsg_Inner::H_gfx(Cmsg_HGfx::Repl_Rune(Cmsg_Repl_Rune::Output(Err(format!("{:?}", err)))));
                G_CmsgQ::send_oneshot(msg);
            }
        }
    }
}
