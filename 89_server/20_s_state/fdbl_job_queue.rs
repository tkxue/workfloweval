use super::*;

use anyhow::anyhow;
use foundationdb::directory::{Directory, DirectoryError, DirectoryLayer, DirectoryOutput};
use foundationdb::future::FdbSlice;
use foundationdb::options::{StreamingMode, TransactionOption};
use foundationdb::tuple::PackResult;
use foundationdb::{RangeOption, Transaction};
use rustler::Encoder;
use std::cell::OnceCell;
use std::io::BufRead;

pub struct JobQueue_Util {}

pub struct JobQueue_Obj<'a> {
    obj_root: Sa_Dir<'a>,
}

pub enum JobQueue_Dir {
    q_runnable,
    q_running,
    q_done,
    q_failed,
    q_timeout,

    j_init,
    j_status,

    clock,
}

impl JobQueue_Dir {
    pub fn to_str(&self) -> &str {
        match self {
            JobQueue_Dir::q_runnable => "q_runnable",
            JobQueue_Dir::q_running => "q_running",
            JobQueue_Dir::q_done => "q_done",
            JobQueue_Dir::q_failed => "q_failed",
            JobQueue_Dir::q_timeout => "q_timeout",
            JobQueue_Dir::j_init => "j_init",
            JobQueue_Dir::j_status => "j_status",
            JobQueue_Dir::clock => "clock",
        }
    }
}

impl<'a> JobQueue_Obj<'a> {
    pub async fn new(trx: &'a Transaction, path: &String) -> Result<JobQueue_Obj<'a>, DirectoryError> {
        let obj_root = Xdb_Util::get_directory(&trx, &Xdb_Util::split_string(&path)).await?;
        Ok(JobQueue_Obj { obj_root: obj_root })
    }

    pub async fn get_dir(&self, dir: JobQueue_Dir) -> Result<Sa_Dir<'a>, Sa_Err> {
        todo!()
        // self.obj_root.open_str(dir.to_str()).await
    }
}

rustler::atoms! {
    ok,
    err
}

impl JobQueue_Util {
    async fn get_clock_value<'a>(fdbo__clock: &Sa_Dir<'a>) -> Result<u128, Sa_Err> {
        /*
        let x = fdbo__clock.get(&("value",)).await?;
        match x {
            None => Ok(0),
            Some(t) => {
                let v = match TryInto::<[u8; 16]>::try_into(t.as_ref()) {
                    Err(_) => 0_u128,
                    Ok(arr) => u128::from_le_bytes(arr),
                };
                Ok(v)
            }
        }

         */
        todo!()
    }

    // returns cur value of clock
    async fn inc_if_equal<'a>(path: String, v: u128) -> Result<u128, Sa_Err> {
        todo!()
        /*
        let trx: Transaction = _G_S_Ffi.get().unwrap().db.create_trx()?;
        let fdbo_root = JobQueue_Obj::new(&trx, &path).await?;
        let fdbo__clock = fdbo_root.get_dir(JobQueue_Dir::clock).await?;

        let cur_value = Self::get_clock_value(&fdbo__clock).await?;
        if cur_value == v {
            let bytes: [u8; 16] = (v + 1).to_le_bytes();
            fdbo__clock.set(&("value, "), &bytes)?;

            Ok(v + 1)
        } else {
            Ok(cur_value)
        }
        */
    }

    async fn get_clock<'a>(path: String) -> Result<u128, DirectoryError> {
        /*
        let trx: Transaction = _G_S_Ffi.get().unwrap().db.create_trx()?;
        let fdbo_root = JobQueue_Obj::new(&trx, &path).await?;
        let fdbo__clock = fdbo_root.get_dir(JobQueue_Dir::clock).await?;
        let out = Self::get_clock_value(&fdbo__clock).await?;
        trx.commit().await.map_err(|e| *e)?;
        Ok(out)

         */
        todo!()
    }

    /*
    async fn doc_add<'a>(
        path: String,
        doc_id: Vec<u8>,
        terms: Vec<Vec<u8>>,
    )
    -> Result<(), DirectoryError>
    {
        let trx: Transaction = G_S_Ffi.get().unwrap().db.create_trx()?;
        let fdbo_root = Fdb__Job_Queue::new(&trx, &path).await?;
        let fdbo__doc_to_term = fdbo_root.init__doc_to_term().await?;
        let fdbo__term_to_doc = fdbo_root.init__term_to_doc().await?;

        for term in terms.iter() {
            fdbo__doc_to_term.set(&(&doc_id, &term), &[]);
            fdbo__term_to_doc.set(&(&term, &doc_id), &[]);
        }

        trx.commit().await.map_err(|e| *e)?;
        Ok(())
    }

    async fn doc_del<'a>(path: String, doc_id: Vec<u8>) -> Result<(), DirectoryError> {
        let trx: Transaction = G_S_Ffi.get().unwrap().db.create_trx()?;
        let fdbo_root = Fdb__Job_Queue::new(&trx, &path).await?;
        let fdbo__doc_to_term = fdbo_root.init__doc_to_term().await?;
        let fdbo__term_to_doc = fdbo_root.init__term_to_doc().await?;

        let data = fdbo__doc_to_term.get_all_in_subspace(&doc_id).await?;
        for fdb_value in data {
            let (doc_id, term) =
                fdbo__doc_to_term.unpack::<(Vec<u8>, Vec<u8>)>(fdb_value.key())??;
            fdbo__doc_to_term.clear(&(&doc_id, &term));
            fdbo__term_to_doc.clear(&(&term, &doc_id));
        }

        trx.commit().await.map_err(|e| *e)?;
        Ok(())
    }

    async fn term_lookup<'a>(path: String, term: Vec<u8>) -> Result<Vec<Vec<u8>>, DirectoryError> {
        let trx: Transaction = G_S_Ffi.get().unwrap().db.create_trx()?;
        let fdbo_root = Fdb__Job_Queue::new(&trx, &path).await?;
        let fdbo__doc_to_term = fdbo_root.init__doc_to_term().await?;
        let fdbo__term_to_doc = fdbo_root.init__term_to_doc().await?;

        let data = fdbo__term_to_doc.get_all_in_subspace(&term).await?;

        let out = data
            .into_iter()
            .map(|x| {
                let (_term, doc_id) = fdbo__term_to_doc.unpack::<(Vec<u8>, Vec<u8>)>(x.key())??;
                Ok(doc_id)
            })
            .collect::<Result<Vec<_>, DirectoryError>>()?;

        trx.commit().await.map_err(|e| *e)?;

        Ok(out)
    }
    */
}

/*


q_runnable
q_running
q_done
q_failed
q_timeout

j_init
j_status

clock
 */

/*
#[rustler::nif]
pub fn inv_index__doc_add<'a>(
    env: rustler::Env<'a>,
    pid: rustler::LocalPid,
    path: String,
    doc_id: rustler::Binary<'a>,
    terms: Vec<rustler::Binary<'a>>,
) {
    let doc_id = doc_id.as_slice().to_vec();
    let terms = terms
        .iter()
        .map(|x| x.as_slice().to_vec())
        .collect::<Vec<_>>();
    let sffi_global = G_S_Ffi.get().unwrap();
    sffi_global.run_fdbt(pid, Fdb__Job_Queue__Util::doc_add(path, doc_id, terms));
}

#[rustler::nif]
pub fn inv_index__doc_del<'a>(
    env: rustler::Env<'a>,
    pid: rustler::LocalPid,
    path: String,
    doc_id: rustler::Binary<'a>,
) {
    let doc_id = doc_id.as_slice().to_vec();
    let sffi_global = G_S_Ffi.get().unwrap();
    sffi_global.run_fdbt(pid, Fdb__Job_Queue__Util::doc_del(path, doc_id));
}
*/

#[rustler::nif]
pub fn job_queue__get_clock<'a>(env: rustler::Env<'a>, pid: rustler::LocalPid, path: String) {
    let sffi_global = _G_S_Ffi.get().unwrap();
    G_S_Ffi::run_fdbt(pid, JobQueue_Util::get_clock(path));
}

#[rustler::nif]
pub fn job_queue__inc_if_equal<'a>(env: rustler::Env<'a>, pid: rustler::LocalPid, path: String, v: u128) {
    /*
    let sffi_global = _G_S_Ffi.get().unwrap();
    G_S_Ffi::run_fdbt(pid, JobQueue_Util::inc_if_equal(path, v));

     */
    todo!()
}
