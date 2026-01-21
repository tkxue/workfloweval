use super::*;

/*
pub struct Ws_Client_Retry {
    state: Rr<Ws_Client_Retry_State>,
}

#[derive(Clone)]
pub enum Ws_Client_Retry_State {
    Trying {
        init: Rc<Ws_Client_Init>,
    },
    Running {
        init: Rc<Ws_Client_Init>,
        inner: Rc<Ws_Client_Once>,
    },
    Closed,
}

impl Ws_Client_Retry {
    pub fn new(init: Rc<Ws_Client_Init>) -> Ws_Client_Retry {
        let state = Rr::new(Self::create_ws(init.clone()));
        Self::spawn_reconnect(init.clone(), state.clone());
        Ws_Client_Retry { state }
    }

    pub fn spawn_reconnect(init: Rc<Ws_Client_Init>, state: Rr<Ws_Client_Retry_State>) {
        /*
        let state = state.clone();
        let init = init.clone();
        let t = async move {
            loop {
                match state.get_cloned() {
                    Ws_Client_Retry_State::Trying { init } => {
                        state.replace(Self::create_ws(init.clone()));
                    }
                    Ws_Client_Retry_State::Running { init, inner } => {
                        if !inner.is_open() {
                            state.replace(Self::create_ws(init.clone()));
                        }
                    }
                    Ws_Client_Retry_State::Closed => {
                        break;
                    }
                }
                XdomA::sleep_millis(init.retry_delay_millis as usize).await;
            }
        };
        XdomA::spawn_local(Box::pin(t));

         */
        damn_it!("")
    }

    pub fn create_ws(init: Rc<Ws_Client_Init>) -> Ws_Client_Retry_State {
        wlog!("ws connecting to: {:?}", init.url);
        match Ws_Client_Once::new(init.as_ref()) {
            Ok(x) => Ws_Client_Retry_State::Running {
                init: init.clone(),
                inner: Rc::new(x),
            },
            Err(_) => Ws_Client_Retry_State::Trying { init: init.clone() },
        }
    }

    pub fn send(&self, msg: Ws_Msg) -> Result<(), Ws_Client_Err> {
        match self.state.get_cloned() {
            Ws_Client_Retry_State::Running { inner, .. } => inner.send(msg),
            Ws_Client_Retry_State::Closed => Err(Ws_Client_Err::Send_On_Closed),
            Ws_Client_Retry_State::Trying { .. } => Err(Ws_Client_Err::Send_On_Trying),
        }
    }
}
*/
