use super::*;

use foundationdb::Transaction;
use foundationdb::directory::DirectoryError;
use s_state::{InvIndex_DocId, My_Context_T, Sa_Dir, Sb_Value};
use std::sync::Arc;

pub struct SApp_Counter<'a> {
    root_dir: Arc<Sa_Dir<'a>>,
    count: Sb_Value<'a, i128>,
}

impl<'a> SApp_Counter<'a> {
    pub async fn new(trx: &'a Transaction, path: &str) -> Result<SApp_Counter<'a>, Sa_Err> {
        let msg = format!("SApp_Counter::new, path = {path}");
        let root_dir = Arc::new(Sa_Dir::new(trx, path).await.ctx(&msg)?);
        Ok(SApp_Counter {
            count: Sb_Value::new(&root_dir, "count"),
            root_dir,
        })
    }

    pub async fn handle(path: &str, aux: &N_ToS_Aux, msg_in: &N_Counter_ToS) -> Result<N_Counter_ToC, Sa_Err> {
        let msg = format!("SApp_Counter::handle, path = {path}, msg_in = {msg_in:?}");
        let trx: Transaction = _G_S_Ffi.get().unwrap().db.create_trx().ctx("create_trx").ctx(&msg)?;
        let out = SApp_Counter::new(&trx, path)
            .await
            .ctx(&msg)?
            .handle_inner(msg_in)
            .await
            .ctx(&msg)?;
        trx.commit().await.map_err(|e| *e).ctx("commit").ctx(&msg)?;
        Ok(out)
    }

    pub async fn handle_inner(&self, msg_in: &N_Counter_ToS) -> Result<N_Counter_ToC, Sa_Err> {
        let msg = format!("SApp_Counter::handle_inner, msg_in = {msg_in:?}");
        let t = self.count.get().await.ctx(&msg)?;
        match msg_in {
            N_Counter_ToS::Inc => {
                self.count.set(t + 1).ctx(&msg)?;
            }
            N_Counter_ToS::Dec => {
                self.count.set(t - 1).ctx(&msg)?;
            }
            N_Counter_ToS::Get => {}
            N_Counter_ToS::Set(n) => {
                self.count.set(*n).ctx(&msg)?;
            }
        }
        let t = self.count.get().await.ctx(&msg)?;
        Ok(N_Counter_ToC::Value(t))
    }
}
