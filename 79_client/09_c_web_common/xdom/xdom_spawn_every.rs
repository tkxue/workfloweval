use super::*;

#[derive(Clone, Debug)]
pub struct Xdom_Time_Event {
    _cnt: u64,
    _offset_millis: u64,
}

pub struct Xdom_Time_Event_Gen {
    pub t: instant::Instant,
    pub cnt: Cell<u64>,
}

impl Xdom_Time_Event_Gen {
    pub fn new() -> Xdom_Time_Event_Gen {
        Xdom_Time_Event_Gen {
            t: instant::Instant::now(),
            cnt: Cell::new(0),
        }
    }

    pub fn take(&self) -> Xdom_Time_Event {
        let t = (instant::Instant::now() - self.t).as_millis() as u64;
        let cnt = self.cnt.get();
        self.cnt.set(cnt + 1);
        Xdom_Time_Event {
            _cnt: cnt,
            _offset_millis: t,
        }
    }
}

#[derive(Clone)]
pub struct Xdom_Spawn_Every {
    pub tick_millis: u64,
    pub keep_running: Rc<dyn Fn(Xdom_Time_Event) -> bool>,
    pub normal_tick: Rc<dyn Fn(Xdom_Time_Event)>,
    pub miseed_tick: Rc<dyn Fn(Xdom_Time_Event)>,
}

impl Xdom_Spawn_Every {
    pub fn spawn(&self) {
        let obj = self.clone();
        XdomA::spawn_local(Box::pin(async move {
            let t_start = instant::Instant::now();
            let mut cnt: u64 = 1;
            while (obj.keep_running)(Xdom_Time_Event {
                _cnt: cnt,
                _offset_millis: 0,
            }) {
                let cur_time = (instant::Instant::now() - t_start).as_millis() as u64;
                if cur_time < cnt * obj.tick_millis {
                    (obj.normal_tick)(Xdom_Time_Event {
                        _cnt: cnt,
                        _offset_millis: cur_time,
                    });
                    let cur_time = (instant::Instant::now() - t_start).as_millis() as u64;
                    let wait_millis = (cnt * obj.tick_millis).saturating_sub(cur_time);
                    XdomA::sleep_millis(wait_millis as usize).await;
                } else {
                    (obj.miseed_tick)(Xdom_Time_Event {
                        _cnt: cnt,
                        _offset_millis: cur_time,
                    })
                }
                cnt = cnt + 1;
            }
        }));
    }
}
