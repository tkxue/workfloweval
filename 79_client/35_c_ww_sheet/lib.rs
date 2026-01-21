#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_imports)]

pub use c_app_msg::*;
use e_api::*;
use ironcalc_base::{types::CellType, Model};
// use formualizer_workbook::{LiteralValue, Workbook};
use wasm_bindgen::prelude::wasm_bindgen;
use web_common::{my_ffi, Id_Proc, Msg_Code, Web_Root, XdomA};

pub struct Rust_Sheet_Main {
    // pub(crate) _po_cbs: Arc<PO>,
}

#[wasm_bindgen]
pub struct Rust_Sheet_Ffi {}

#[wasm_bindgen]
impl Rust_Sheet_Ffi {
    pub fn rust_sheet_ffi__create(
        name: String,
        msg_code: wb::JsValue,
        args: wb::JsValue,
    ) -> Rust_Sheet_Ffi {
        console_error_panic_hook::set_once();
        wasm_init();
        let msg_code = Msg_Code::new(msg_code);
        wlog!("sheet_ffi: {:?}", Rust_Sheet_Main::main(&msg_code));
        Rust_Sheet_Ffi {}
    }
}

impl Rust_Sheet_Main {}

impl Rust_Sheet_Main {
    async fn load_file(
        url: String,
    ) -> Result<Vec<Cmsg_Sheet__Cell_Data>, String> {
        let response = reqwest::get(url)
            .await
            .map_err(|err| format!("err: {:?}", err))?
            .bytes()
            .await
            .map_err(|err| format!("err: {:?}", err))?;

        let workbook = xlsx::import::load_from_xlsx_bytes(
            &*response,
            "",
            "en",
            "America/New_York",
        )
        .map_err(|err| format!("workbook error: {:?}", err))?;

        let model = Model::from_workbook(workbook, "en")
            .map_err(|err| format!("model error: {:?}", err))?;

        let idxs = model.get_all_cells();

        let contents = idxs
            .iter()
            .filter_map(|idx| {
                if idx.index == 0 {
                    Some(Cmsg_Sheet__Cell_Data {
                        index: idx.index,
                        row: idx.row,
                        column: idx.column,
                        data: model.get_formatted_cell_value(
                            idx.index, idx.row, idx.column,
                        ),
                    })
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();

        Ok(contents)
    }

    fn main(msg_code: &Msg_Code) -> Result<(), Box<dyn std::error::Error>> {
        let file_lst =
            include_str!("/r/data/xls/lst").lines().collect::<Vec<_>>();

        let prefix = format!(
            "{}//{}/",
            msg_code.window_location_protocol, msg_code.window_location_host
        );

        XdomA::spawn_local(Box::pin(async move {
            loop {
                G_CmsgQ::wait_on().await;
                for msg in G_CmsgQ::take_all() {
                    match msg.inner {
                        Cmsg_Inner::Ww_sheet(cmsg_ww_sheet) => {
                            match cmsg_ww_sheet {
                                Cmsg_WwSheet::OpenFile { cmd } => {
                                    let url = format!(
                                        "{}/pub/data/xls/{}",
                                        prefix, cmd
                                    );
                                    let res = Self::load_file(url).await;
                                    G_CmsgQ::send_oneshot(Cmsg_Inner::H_gfx(
                                        Cmsg_HGfx::Sheet(Cmsg_Sheet::Loaded(
                                            res,
                                        )),
                                    ));
                                }
                            }
                        }
                        y => {
                            wlog!("Rust_Sheet_Main: don't recognize: {:?}", y);
                        }
                    }
                }
            }
        }));

        Ok(())
    }
}
