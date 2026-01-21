#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_imports)]

pub use c_app_msg::*;
use e_api::*;
use rustpython::vm::{self, PyResult, VirtualMachine};
use wasm_bindgen::prelude::wasm_bindgen;
use web_common::{Id_Proc, Msg_Code, Web_Root, XdomA, my_ffi};

pub struct Rust_Python_Main {
    // pub(crate) _po_cbs: Arc<PO>,
}

#[wasm_bindgen]
pub struct Rust_Python_Ffi {}

#[wasm_bindgen]
impl Rust_Python_Ffi {
    pub fn rust_python_ffi__create(
        name: String,
        msg_code: wb::JsValue,
        args: wb::JsValue,
    ) -> Rust_Python_Ffi {
        console_error_panic_hook::set_once();
        wasm_init();
        Rust_Python_Main::main(name, msg_code, args);

        Rust_Python_Ffi {}
    }
}

// #[wasm_bindgen]
impl Rust_Python_Main {}

impl Rust_Python_Main {
    pub fn eval_string(
        vm: &VirtualMachine,
        scope: vm::scope::Scope,
        cmd: &str,
    ) -> Result<vm::PyObjectRef, String> {
        let code_obj = match vm.compile(
            cmd,
            vm::compiler::Mode::Eval,
            "<repl>".to_owned(),
        ) {
            Ok(x) => x,
            Err(_) => match vm.compile(
                cmd,
                vm::compiler::Mode::Exec,
                "<repl>".to_owned(),
            ) {
                Ok(x) => x,
                Err(x) => Err(format!("{:?}", x))?,
            },
        };
        let result = vm
            .run_code_obj(code_obj, scope.clone())
            .map_err(|err| format!("{:?}", err))?;
        Ok(result)
    }

    pub fn main(
        name: String,
        msg_code: wb::JsValue,
        _args: wb::JsValue,
    ) -> Rust_Python_Main {
        XdomA::spawn_local(Box::pin(async {
            let settings = rustpython::vm::Settings::default();
            let mut interp = vm::Interpreter::with_init(
                settings,
                |vm: &mut VirtualMachine| {},
            );

            let scope = interp.enter(|vm| vm.new_scope_with_builtins());

            loop {
                G_CmsgQ::wait_on().await;
                for msg in G_CmsgQ::take_all() {
                    match msg.inner {
                        Cmsg_Inner::Ww_python(cmsg_ww_python) => {
                            match cmsg_ww_python {
                                Cmsg_WwPython::ReplEval { cmd } => {
                                    let res = interp.enter(|vm| {
                                        Self::eval_string(
                                            vm,
                                            scope.clone(),
                                            &cmd,
                                        )
                                        .map(|x| {
                                            format!("{:#?}", x)
                                                .lines()
                                                .map(|s| s.to_string())
                                                .collect::<Vec<_>>()
                                        })
                                    });
                                    G_CmsgQ::send_oneshot(Cmsg_Inner::H_gfx(
                                        Cmsg_HGfx::Repl_Python(
                                            Cmsg_Repl_Python::Output(res),
                                        ),
                                    ));
                                }
                            }
                        }

                        x => {
                            wlog!("ww_python unrecognized: {:?}", x)
                        }
                    }
                }
            }
        }));

        Rust_Python_Main {}
    }
}
