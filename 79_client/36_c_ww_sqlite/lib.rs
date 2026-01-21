#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_imports)]

pub use c_app_msg::*;
use e_api::*;
use rusqlite::{Connection, Result};
use wasm_bindgen::prelude::wasm_bindgen;
use web_common::{Id_Proc, Msg_Code, Web_Root, XdomA, my_ffi};

pub struct Rust_Sqlite_Main {
    // pub(crate) _po_cbs: Arc<PO>,
}

#[wasm_bindgen]
pub struct Rust_Sqlite_Ffi {}

#[wasm_bindgen]
impl Rust_Sqlite_Ffi {
    pub fn rust_sqlite_ffi__create(
        name: String,
        msg_code: wb::JsValue,
        args: wb::JsValue,
    ) -> Rust_Sqlite_Ffi {
        console_error_panic_hook::set_once();
        wasm_init();
        Rust_Sqlite_Main::main();
        Rust_Sqlite_Ffi {}
    }
}

impl Rust_Sqlite_Main {}

impl Rust_Sqlite_Main {
    fn handle_input(conn: &Connection, input: &str) -> Result<Vec<String>> {
        let mut out = vec![];

        let mut stmt = conn.prepare(input)?;
        let column_count = stmt.column_count();
        if column_count == 0 {
            // It's an UPDATE, INSERT, DELETE, or DDL
            let changes = stmt.execute([])?;
            out.push(format!("Rows affected: {}", changes));
        } else {
            // It's a SELECT or similar
            let column_names: Vec<String> = stmt
                .column_names()
                .into_iter()
                .map(|s| s.to_string())
                .collect();

            out.push(format!("{}", column_names.join(" | ")));

            let mut rows = stmt.query([])?;
            while let Some(row) = rows.next()? {
                let mut res = Vec::new();
                for i in 0..column_count {
                    let val: rusqlite::types::Value = row.get(i)?;
                    res.push(format!("{:?}", val));
                }

                out.push(format!("{}", res.join(" | ")));
            }
        }
        Ok(out)
    }

    pub fn main() {
        XdomA::spawn_local(Box::pin(async {
            wlog!("Rust_Sqlite_Main::main");
            let conn = Connection::open_in_memory().unwrap();
            loop {
                G_CmsgQ::wait_on().await;
                for msg in G_CmsgQ::take_all() {
                    match msg.inner {
                        Cmsg_Inner::Ww_sqlite(cmsg_ww_sqlite) => {
                            match cmsg_ww_sqlite {
                                Cmsg_WwSqlite::ReplEval { cmd } => {
                                    let out =
                                        Self::handle_input(&conn, cmd.as_str())
                                            .map_err(|err| {
                                                format!("{:?}", err)
                                            });
                                    G_CmsgQ::send_oneshot(Cmsg_Inner::H_gfx(
                                        Cmsg_HGfx::Repl_Sqlite(
                                            Cmsg_Repl_Sqlite::Output(out),
                                        ),
                                    ));
                                }
                            }
                        }
                        y => {
                            wlog!("Rust_Sqlite_Main: don't recognize: {:?}", y);
                        }
                    }
                }
            }
            /*
             */
        }));
    }
}
