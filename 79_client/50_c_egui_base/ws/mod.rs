use super::*;
use web_common::{CbArc, Ws_Client_Init, Ws_Client_Once, Ws_Event, Ws_Msg_In, Ws_Msg_Out, XdomA, XdomA_IFrame_Urls};

/*
#[derive(Clone)]
pub struct Ws_Loop_Ptr {
    inner: Arc<Mutex<Ws_Loop_Inner>>,
}

pub enum Ws_State {
    Not_Connected,
    Connected { inner: Ws_Client_Once },
    Connecting { inner: Ws_Client_Once, start: instant::Instant },
    Err(Err_Stack),
}

pub struct Ws_Loop_Inner {
    pub url: String,
    pub ws_state: Ws_State,
    pub ws_cnt: u64,
    pub handler: Option<Rc<dyn Fn(&Ws_Msg_In)>>,
    pub ws_sent: u64,
    pub ws_recv: u64,
}

impl Ws_Loop_Ptr {
    pub fn send(&self /*net_data: &NetS_Root*/) {
        /*
        let mut g = self.inner.lock().unwrap();
        g.ws_sent = g.ws_sent + 1;

        match &g.ws_state {
            Ws_State::Not_Connected => {}
            Ws_State::Connecting { .. } => {}
            Ws_State::Err(_) => {}
            Ws_State::Connected { inner } => {
                let mut t = vec![0_u8; 2048];
                let mut w = My_Slice_Writer {
                    data: t.as_mut_slice(),
                    bytes_written: 0,
                };
                match L_NetData_Util::write_to_my_slice_writer(net_data, &mut w) {
                    Ok(v) => {
                        inner.send(Ws_Msg_Out::Binary(w.as_slice()));
                    }
                    Err(_) => {
                        wlog!("unexpected error encoding NetS_Root")
                    }
                }
            }
        }

         */
    }

    pub fn ping(&self) {
        let t = self.inner.lock().unwrap();
        match &t.ws_state {
            Ws_State::Not_Connected => {}
            Ws_State::Connecting { .. } => {}
            Ws_State::Err(_) => {}
            Ws_State::Connected { inner } => {
                inner.send(Ws_Msg_Out::Text(Rc::new("ping".to_string())));
            }
        }
    }

    /*
    pub fn get_summary(&self) -> Ws_Summary {
        let g = self.inner.lock().unwrap();
        let s = match g.ws_state {
            Ws_State::Not_Connected => Ws_State__Cat::Not_Connected,
            Ws_State::Connected { .. } => Ws_State__Cat::Connected,
            Ws_State::Connecting { .. } => Ws_State__Cat::Not_Connected__Connecting,
            Ws_State::Err(_) => Ws_State__Cat::Err,
        };
        Ws_Summary {
            cat: s,
            ws_cnt: g.ws_cnt,
            ws_sent: 0,
            ws_recv: 0,
        }
    }
    */

    pub fn new(wasm_version: Rc<String>) -> Ws_Loop_Ptr {
        let url = match PO::__get_singleton().is_dev {
            true => XdomA_IFrame_Urls::new_local(wasm_version).ws,
            false => XdomA_IFrame_Urls::new_public(wasm_version).ws,
        };

        Ws_Loop_Ptr {
            inner: Arc::new(Mutex::new(Ws_Loop_Inner {
                url,
                ws_state: Ws_State::Not_Connected,
                ws_cnt: 0,
                handler: None,
                ws_sent: 0,
                ws_recv: 0,
            })),
        }
    }

    pub fn set_handler(&self, h: Rc<dyn Fn(&Ws_Msg_In)>) {
        let mut inner = self.inner.lock().unwrap();
        inner.handler = Some(h)
    }

    pub fn spawn_reconnect(&self) {
        let obj = self.clone();
        XdomA::spawn_local(Box::pin(async move {
            loop {
                let need_reconnect = match &obj.inner.lock().unwrap().ws_state {
                    Ws_State::Not_Connected => true,
                    Ws_State::Connected { inner } => {
                        // https://developer.mozilla.org/en-US/docs/Web/API/WebSocket/readyState
                        inner.raw_ws.ready_state() != 1
                    }
                    Ws_State::Connecting { inner, start } => (instant::Instant::now() - *start).as_millis() > 5_000,
                    Ws_State::Err(_) => true,
                };

                if need_reconnect {
                    obj.reconnect();
                }

                XdomA::sleep_millis(2000).await
            }
        }));
    }

    pub fn reconnect(&self) {
        {
            let mut g = self.inner.lock().unwrap();
            g.ws_cnt = g.ws_cnt + 1;
        }

        let obj = self.clone();
        let client_init = Ws_Client_Init {
            url: self.inner.lock().unwrap().url.clone(),
            cb: CbARc::new(Rc::new(move |msg| {
                obj.handler(msg);
            })),
        };

        match Ws_Client_Once::new(&client_init) {
            Ok(x) => {
                self.inner.lock().unwrap().ws_state = Ws_State::Connecting {
                    inner: x,
                    start: instant::Instant::now(),
                }
            }
            Err(x) => {
                self.inner.lock().unwrap().ws_state = Ws_State::Err(x);
            }
        }
    }

    pub fn handler(&self, msg: Ws_Event) {
        match &msg {
            Ws_Event::Open => {
                // wlog!("ws connected");
                let mut g = self.inner.lock().unwrap();
                match std::mem::replace(&mut g.ws_state, Ws_State::Not_Connected) {
                    Ws_State::Connecting { inner, start } => {
                        // let mut t = vec![0_u8; 2048];
                        /*
                        let mut w = My_Slice_Writer {
                            data: t.as_mut_slice(),
                            bytes_written: 0,
                        };
                        */
                        // let _ = L_NetData_Util::write_to(&NetS_Root::Hi, &mut w).unwrap();
                        // inner.send(Ws_Msg_Out::Binary(w.as_slice()));
                        g.ws_state = Ws_State::Connected { inner };
                        if let Some(h) = g.handler.clone() {
                            (h.deref())(&Ws_Msg_In::Evt_Open);
                        }
                    }
                    x => g.ws_state = x,
                }
            }
            Ws_Event::Error(e) => {
                // wlog!("ws error: {:?}", e);
                let mut g = self.inner.lock().unwrap();
                g.ws_state = Ws_State::Not_Connected;
            }
            Ws_Event::Msg(m) => {
                let mut g = self.inner.lock().unwrap();
                g.ws_recv = g.ws_recv + 1;
                if let Some(h) = g.handler.clone() {
                    (h.deref())(&m);
                } else {
                    wlog!("no active handler")
                }
            }
            // Ws_Event::Close(e)
            Ws_Event::Close => {
                // wlog!("ws closed: {:?}", e);
                let mut g = self.inner.lock().unwrap();
                g.ws_state = Ws_State::Not_Connected;
            }
        }
    }
}

/*
use super::*;
use e_util::{FlatFS, Tgz_Util};
use cmsg::{NetC_Root, Server_Camera};

mod net_inner;
mod ws_loop;
pub use net_inner::*;
pub use ws_loop::*;

pub struct Fetch_Manager {
    _state: HVec<Id_Res_Fetch, Id_Res_Fetch_State>,
}

pub struct IAc_Ww_Net {
    frq_net: Frq_Net,
    // ======================
    manager_state: Manager_Stat,
    open_jobs: VecDeque<Ww_Job>,
    geo_graph: Option<Xos_Jab>,
    geo_boundary: Option<Xos_Fetch_File>,
    geo_mtile: Option<Xos_Fetch_File>,
    window_size: Xdom_Window_Size,
    pub mdb_debug: Debug_Status,
    pub ws_loop: Ws_Loop_Ptr,
}

mm_lazy_singleton! {
    IAc_Ww_Net ;
    Arc_Ac<Ac_Ww_Net> ;
    __set_singleton ;
    __get_singleton ;
}

impl<'a> Impl_Actor_T<Ac_Ww_Net> for IAc_Ww_Net {
    fn handle_local(&mut self, _ra: &Arc_Ac<Ac_Ww_Net>, t: <Ac_Ww_Net as Actor_Config_T>::Local_In) {
        match t {
            Ac_Ww_Net__Local_In::Send_Work_Queue_Manager => PO::call_t(&RAc_Ww_Desktop::Client_Dbg_Info_Update(
                Client_Dbg_Info_Update::Manager_State(Rov::Ref(&self.manager_state)),
            )),
            Ac_Ww_Net__Local_In::Res_Fetch_Set_State { k, v } => {
                self.set_state(k, v);
            }
            Ac_Ww_Net__Local_In::Queue_Job(ww_job) => {
                self.open_jobs.push_back(ww_job);
                self.process_job();
            }
            Ac_Ww_Net__Local_In::Push_Frq_Net => {
                /*
                let ws_cnt = self.ws_loop.get_ws_cnt();
                self.frq_net.ws_cnt.update_input(&|x| {
                    *x = ws_cnt;
                    true
                });

                self.frq_net.frp_base.push_updates(false);
                // unrelated part
                // self.last_ws_status.set(self.ws_loop.get_status());
                if self.mdb_debug.need_update.replace(false) {
                    PO::call_t(&RAc_Ww_Desktop::Desktop_Data__Debug_Status_Res(
                        self.mdb_debug.get_current(),
                    ));
                }
                */
            }
            Ac_Ww_Net__Local_In::Ws_Msg_In(x) => match x {
                Ws_Msg_In::Text(s) => {
                    wlog!("ws_msg_in text: {:?}", s.as_str());
                }
                Ws_Msg_In::Binary(x) => {
                    match L_NetData_Util::read_from_bytes::<cmsg::NetC_Root<'static>>(x.as_slice()) {
                        Ok(netc_root) => match netc_root {
                            NetC_Root::Data_Mob__Block_16 { inner } => {
                                PO::call_t(&RAc_Ww_Map_Slow::Data_Mob__Block_16 { inner });
                            }
                            NetC_Root::Set_Server_Camera {
                                server_camera: server_camera,
                            } => PO::call_t(&RAc_Ww_Desktop::Set__Server_Camera { server_camera }),
                        },
                        Err(_) => {
                            wlog!("err netc_root");
                        }
                    }
                }
                Ws_Msg_In::Unknown(_) => {
                    wlog!("ws_msg_in unknown");
                }
                Ws_Msg_In::Blob(_) => {
                    wlog!("ws_msg_in blob");
                }
                Ws_Msg_In::Evt_Open => {
                    wlog!("ws_msg_in evt_open");
                }
            },
            Ac_Ww_Net__Local_In::Ws_Heartbeat => {
                self.ws_loop.ping();
                let summary = self.ws_loop.get_summary();
                PO::call_t(&RAc_Ww_Desktop::Ws_Summary { inner: summary });
            }
            Ac_Ww_Net__Local_In::Ws_Bin_To_Ex(_) => {
                panic!("")
            }
        }
    }

    fn handle_remote(&mut self, _ra: &Arc_Ac<Ac_Ww_Net>, t: <Ac_Ww_Net as Actor_Config_T>::Remote_In) {
        match t {
            RAc_Ww_Net::Reload_Logic_Wasm => PO::reload_logic(),
            RAc_Ww_Net::Spack { .. } => {
                wlog!("spack");
            }
            RAc_Ww_Net::Worker_Ready(x) => {
                self.manager_state.free_worker_list.insert(x);
                self.process_job();
            }
            RAc_Ww_Net::Queue_Job(ww_job) => {
                self.open_jobs.push_back(ww_job);
                self.process_job();
            }
            RAc_Ww_Net::Res_Fetch_Set_State { k, v } => {
                self.set_state(k, v);
            }
            RAc_Ww_Net::Debug_Status__Update_Status { status_group, data } => {
                self.mdb_debug.update_status(status_group, data)
            }
            RAc_Ww_Net::Debug_Status__Set_Selected { data } => {
                self.mdb_debug.set_selected(&data);
            }
            RAc_Ww_Net::Debug_Status__Get_Status => {}
            RAc_Ww_Net::To_Server { inner } => {
                // self.ws_loop.send(&inner);
            }
            RAc_Ww_Net::Mode { inner } => {
                self.frq_net.s_mode.update_input(&|v| {
                    if *v == inner {
                        false
                    } else {
                        *v = inner;
                        true
                    }
                });
            }
            RAc_Ww_Net::Xzp_Earth(x) => {
                self.frq_net.earth_xgu_zoompan.update_input(&|v| {
                    if *v == x {
                        false
                    } else {
                        *v = x;
                        true
                    }
                });
            }
            /*
            RAc_Ww_Net::Xzp_Town(x) => {
                self.frq_net.town_xgu_zoompan.update_input(&|v| {
                    if *v == x {
                        false
                    } else {
                        *v = x;
                        true
                    }
                });
            }

             */
            RAc_Ww_Net::ReInit_Desktop => {
                PO::call_t(&RAc_Ww_Desktop::Resize(self.window_size));
            }

            RAc_Ww_Net::ReInit_Map_Fast => {
                PO::log(eetf::Term::ByteList(eetf::ByteList {
                    bytes: format!("handling ReInit_Map_Fast").as_bytes().to_vec(),
                }));

                if let Some(x) = &self.geo_graph {
                    PO::call_t(&RAc_Ww_Map_Fast::Geo_Graph { inner: x.clone() });
                }

                if let Some(x) = &self.geo_graph {
                    PO::call_t(&RAc_Ww_Map_Fast::Geo_Graph { inner: x.clone() });
                }
            }
            RAc_Ww_Net::ReInit_Map_Slow => {
                PO::log(eetf::Term::ByteList(eetf::ByteList {
                    bytes: format!("handling ReInit_Map_Slow").as_bytes().to_vec(),
                }));

                if let Some(x) = &self.geo_graph {
                    PO::log(eetf::Term::ByteList(eetf::ByteList {
                        bytes: format!("ReInit_Map_Slow Geo_Graph: sending").as_bytes().to_vec(),
                    }));
                    PO::call_t(&RAc_Ww_Map_Slow::Geo_Graph { inner: x.clone() });
                } else {
                    PO::log(eetf::Term::ByteList(eetf::ByteList {
                        bytes: format!("ReInit_Map_Slow Geo_Graph: None").as_bytes().to_vec(),
                    }));
                }

                if let Some(x) = &self.geo_mtile {
                    PO::log(eetf::Term::ByteList(eetf::ByteList {
                        bytes: format!("ReInit_Map_Slow Geo_MTile: sending").as_bytes().to_vec(),
                    }));

                    PO::call_t(&RAc_Ww_Map_Slow::Geo_MTile { ff: x.clone() });
                } else {
                    PO::log(eetf::Term::ByteList(eetf::ByteList {
                        bytes: format!("ReInit_Map_Slow Geo_MTile: None").as_bytes().to_vec(),
                    }));
                }

                if let Some(x) = &self.geo_boundary {
                    PO::log(eetf::Term::ByteList(eetf::ByteList {
                        bytes: format!("ReInit_Map_Slow Geo_Boundary: sending").as_bytes().to_vec(),
                    }));

                    PO::call_t(&RAc_Ww_Map_Slow::Geo_Boundary { ff: x.clone() });
                } else {
                    PO::log(eetf::Term::ByteList(eetf::ByteList {
                        bytes: format!("ReInit_Map_Slow Geo_Boundary: None").as_bytes().to_vec(),
                    }));
                }
            }
            RAc_Ww_Net::Geo_Graph { inner } => {
                self.geo_graph = Some(inner);
            }
            RAc_Ww_Net::Geo_Boundary { ff } => {
                self.geo_boundary = Some(ff);
            }
            RAc_Ww_Net::Geo_MTile { ff } => {
                self.geo_mtile = Some(ff);
            }
            RAc_Ww_Net::Resize(x) => {
                self.window_size = x;
            }
            RAc_Ww_Net::Xzp_Room(_) => {}
        }
    }

    fn id_gactor() -> Option<Id_GActor> {
        Some(Id_GActor::Ww_Net)
    }
}

impl IAc_Ww_Net {
    pub fn set_state(&mut self, k: Id_Res_Fetch, v: Id_Res_Fetch_State) {
        self.manager_state.fetch_jobs[k] = v;
        PO::call_t(&RAc_Ww_Desktop::Res_Fetch_Set_State { k, v });
    }

    pub fn process_job(&mut self) {
        if self.open_jobs.len() > 0 && self.manager_state.free_worker_list.len() > 0 {
            let job = self.open_jobs.pop_front().unwrap();
            let worker = self.manager_state.free_worker_list.iter().next().unwrap().clone();
            self.manager_state.free_worker_list.remove(&worker);
            match worker {
                Id_Proc::Ww0 => PO::call_t(&RAc_Ww0::Base_Cmd(Ww_Base_Cmd::Queue_Job(job))),
                Id_Proc::Ww1 => PO::call_t(&RAc_Ww1::Base_Cmd(Ww_Base_Cmd::Queue_Job(job))),
                Id_Proc::Ww2 => PO::call_t(&RAc_Ww2::Base_Cmd(Ww_Base_Cmd::Queue_Job(job))),
                Id_Proc::Ww3 => PO::call_t(&RAc_Ww3::Base_Cmd(Ww_Base_Cmd::Queue_Job(job))),
                Id_Proc::Ww4 => PO::call_t(&RAc_Ww4::Base_Cmd(Ww_Base_Cmd::Queue_Job(job))),
                Id_Proc::Ww_Editor => PO::call_t(&RAc_Ww_Editor::Base_Cmd(Ww_Base_Cmd::Queue_Job(job))),
                // Id_Proc::Ww_Map_Fast => Po_Cbs::call_t(&RAc_Ww_Map_Fast::Base_Cmd(Ww_Base_Cmd::Queue_Job(job))),
                // Id_Proc::Ww_Nushell => Po_Cbs::call_t(&RAc_Ww_Nu::Base_Cmd(Ww_Base_Cmd::Queue_Job(job))),
                _ => {
                    wlog!("error: {:?} got in worker queue", worker);
                }
            }
        }
    }

    /*
    pub fn flat_fs_to_shader_src(fs: &FlatFS) -> Result<Xg_Shader_Src, String> {
        Ok(Xg_Shader_Src {
            xg_10_blit_rect: fs.get("earth_10__rect.c")?.clone(),
            xg_11_blit_text: fs.get("xh_10_blit_text.c")?.clone(),
            xg_20_blit_face: fs.get("xh_20_blit_face.c")?.clone(),
            xg_58_dbg_sprite: fs.get("asset_58__sprite.c")?.clone(),
            xg_51_blit_square: fs.get("xh_51_blit_square.c")?.clone(),
            xg_59_blit_mob: fs.get("earth_59__mob.c")?.clone(),
            xg_61_blit_building: fs.get("earth_61__icon.c")?.clone(),
            xg_62_blit_mtile: fs.get("town_62__tile.c")?.clone(),
            xg_63_rt_terrain: fs.get("earth_63__rt_terrain.c")?.clone(),
            xg_term_icon: fs.get("term_62__icon.c")?.clone(),
            xg_term_sidf: fs.get("term_62__sdf.c")?.clone(),
            xg_term_tile: fs.get("term_62__tile.c")?.clone(),
            xg_util: fs.get("xg_util.c")?.clone(),
        })
    }
    */

    /*
    pub async fn monitor_shaders(cbrc: Arc<Po_Cbs>) {
        let mut old_last_modified = "".to_string();
        loop {
            XdomA::sleep_millis(1000).await;
            let url = cbrc.web_root.make_priv_url("shaders.tgz");
            let file = XdomA::fetch_file_loop(&url, 1000, Rc::new(XdomA::default_fetch_handler)).await;
            match file.get_last_modified() {
                None => {
                    wlog!("get_file_last_modified returned none")
                }
                Some(x) => {
                    if old_last_modified != x {
                        old_last_modified = x.to_string();
                        wlog!("reload shaders: {}", old_last_modified);
                        match Tgz_Util::untar_gz(file.data.to_vec().as_slice()) {
                            Ok(fs) => {
                                let src = Self::flat_fs_to_shader_src(&fs);
                                match src {
                                    Ok(x) => Po_Cbs::call_t(&RAc_Xgpu_Shader::Shader_Src(Arc::new(x))),
                                    Err(e) => {
                                        wlog!("trying to load shaders.tgz: error: {}", e)
                                    }
                                }
                            }
                            Err(e) => {
                                e.dump();
                            }
                        }
                    }
                }
            }
        }
    }
    */

    pub fn spawn_update_dbg_info(out: Arc_Ac<Ac_Ww_Net>) {
        XdomA::spawn_local(Box::pin({
            async move {
                loop {
                    XdomA::sleep_millis(1000).await;
                    out.queue_local(Ac_Ww_Net__Local_In::Send_Work_Queue_Manager);
                    out.tick_once("spawn_update_dbg_info");
                }
            }
        }));
    }
}

impl Actor_New_T<Ac_Ww_Net> for crate::IAc_Ww_Net {
    fn new_iac(_args: Actor_New_Args<Ac_Ww_Net>) -> Arc_Ac<Ac_Ww_Net> {
        let ws_loop = Ws_Loop_Ptr::new(PO::__get_singleton().web_root.wasm_version.clone());
        ws_loop.spawn_reconnect();

        let out = Arc_Ac::new0(
            "IAc_Ww_Net".to_string(),
            crate::IAc_Ww_Net {
                manager_state: Manager_Stat::new(),
                open_jobs: VecDeque::new(),
                geo_graph: None,
                geo_boundary: None,
                geo_mtile: None,
                window_size: Xdom_Window_Size { width: 1, height: 1 },
                mdb_debug: Debug_Status::new_empty(),
                frq_net: Frq_Net::new(),
                ws_loop: ws_loop.clone(),
            },
        );

        ws_loop.set_handler(Rc::new({
            let out = out.clone();
            move |x| {
                out.queue_local(Ac_Ww_Net__Local_In::Ws_Msg_In(x.clone()));
                out.tick_loop("");
            }
        }));

        let po_cbs = PO::__get_singleton();
        po_cbs.register("IAc_Ww_Net :: register", out.clone());

        Self::spawn_update_dbg_info(out.clone());

        for id_res_fetch in Id_Res_Fetch::iter() {
            XdomA::spawn_local(Box::pin(async move {
                let po_cbs = PO::__get_singleton();

                let manager = IAc_Ww_Net::__get_singleton();
                manager.queue_local(Ac_Ww_Net__Local_In::Res_Fetch_Set_State {
                    k: id_res_fetch,
                    v: Id_Res_Fetch_State::Fetching,
                });
                let url = po_cbs.web_root.make_pub_url(id_res_fetch.to_url_part());
                let file = XdomA::fetch_file_loop(&url, 1000, Rc::new(XdomA::default_fetch_handler)).await;
                manager.queue_local(Ac_Ww_Net__Local_In::Res_Fetch_Set_State {
                    k: id_res_fetch,
                    v: Id_Res_Fetch_State::Inflating,
                });
                manager.queue_local(Ac_Ww_Net__Local_In::Queue_Job(Ww_Job::Fetch_Process {
                    id: id_res_fetch,
                    file,
                }));
                manager.tick_once("IAc_Ww_Net :: tick_once :: fetch");
            }))
        }

        IAc_Ww_Net::__set_singleton(Arc::new({
            let out = out.clone();
            move || out.clone()
        }));

        XdomA::spawn_local(Box::pin({
            let manager = out.clone();
            async move {
                loop {
                    manager.queue_local(Ac_Ww_Net__Local_In::Push_Frq_Net);
                    manager.tick_once("IAc_Ww_Net :: push_frq_net");
                    XdomA::sleep_millis(100).await;
                }
            }
        }));

        XdomA::spawn_local(Box::pin({
            let manager = out.clone();
            async move {
                loop {
                    manager.queue_local(Ac_Ww_Net__Local_In::Ws_Heartbeat);
                    manager.tick_once("IAc_Ww_Net :: ws_heartbeat");
                    XdomA::sleep_millis(1000).await;
                }
            }
        }));

        out
    }
}

my_init!( Ac_Ww_Net::__set_fn__new_ac(IAc_Ww_Net::__make_fn_new()); );


 */


 */
