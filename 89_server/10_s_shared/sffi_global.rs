use super::*;

use foundationdb::api::NetworkAutoStop;
use foundationdb::directory::DirectoryError;
use rustler::Encoder;
use std::sync::OnceLock;

rustler::atoms! {
    ok,
    err
}

pub struct S_Ffi {
    pub fdb_network: NetworkAutoStop,
    pub db: foundationdb::Database,
    pub tokio_rt: tokio::runtime::Runtime,
}

pub static _G_S_Ffi: OnceLock<S_Ffi> = OnceLock::new();

pub struct G_S_Ffi {}

impl G_S_Ffi {
    pub fn run_fdbt<T: Encoder + 'static>(pid: rustler::LocalPid, af: impl Future<Output = Result<T, DirectoryError>> + 'static + Send) {
        let sffi_global = _G_S_Ffi.get().unwrap();
        sffi_global.tokio_rt.spawn(async move {
            let mut env = rustler::OwnedEnv::new();

            let _ = match af.await {
                Ok(v) => env.send_and_clear(&pid, |env| (RustlerAtoms::get().ok, v).encode(env)),
                Err(v) => env.send_and_clear(&pid, |env| (RustlerAtoms::get().err, format!("{:?}", v)).encode(env)),
            };
        });
    }

    pub fn __init_once() {
        if _G_S_Ffi.get().is_none() {
            print!("SFfi_Global::init/set\n\r");
            let fdb_network = unsafe { foundationdb::boot() };
            let db = foundationdb::Database::new(Some("/r/fdb/fdb.cluster")).unwrap();

            let tokio_rt = tokio::runtime::Builder::new_multi_thread()
                .enable_all()
                .build()
                .expect("Failed to create Tokio runtime");

            let g = S_Ffi { fdb_network, db, tokio_rt };
            let _ = _G_S_Ffi.set(g);
        }
    }
}
