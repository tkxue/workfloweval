use super::*;

use anyhow::anyhow;
use foundationdb::directory::{Directory, DirectoryError, DirectoryLayer, DirectoryOutput};
use foundationdb::options::{StreamingMode, TransactionOption};
use foundationdb::tuple::PackResult;
use foundationdb::{RangeOption, Transaction};
use rustler::Encoder;
use std::cell::OnceCell;
use std::io::BufRead;

pub struct Fdbl_BijU128_Util {}

pub struct Fdbl_BijU128_Obj<'a> {
    obj_root: Sa_Dir<'a>,
}

pub enum Fdbl_BijU128_Dir {
    bytes_to_u128,
    u128_to_bytes,
    cnt,
}

impl Fdbl_BijU128_Dir {
    pub fn to_str(&self) -> &str {
        match self {
            Fdbl_BijU128_Dir::bytes_to_u128 => "bytes_to_u128",
            Fdbl_BijU128_Dir::u128_to_bytes => "u128_to_bytes",
            Fdbl_BijU128_Dir::cnt => "cnt",
        }
    }
}

impl<'a> Fdbl_BijU128_Obj<'a> {
    pub async fn new(trx: &'a Transaction, path: &String) -> Result<Fdbl_BijU128_Obj<'a>, DirectoryError> {
        let obj_root = Xdb_Util::get_directory(&trx, &Xdb_Util::split_string(&path)).await?;
        Ok(Fdbl_BijU128_Obj { obj_root: obj_root })
    }

    pub async fn get_dir(&self, dir: Fdbl_BijU128_Dir) -> Result<Sa_Dir<'a>, DirectoryError> {
        todo!()
        // self.obj_root.open_str(dir.to_str()).await
    }
}

rustler::atoms! {
    ok,
    err
}

impl Fdbl_BijU128_Util {
    async fn doc_add<'a>(path: String, doc_id: Vec<u8>, terms: Vec<Vec<u8>>) -> Result<(), DirectoryError> {
        let trx: Transaction = _G_S_Ffi.get().unwrap().db.create_trx()?;
        let fdbo_root = Fdbl_BijU128_Obj::new(&trx, &path).await?;
        let fdbo__doc_to_term = fdbo_root.get_dir(Fdbl_BijU128_Dir::u128_to_bytes).await?;
        let fdbo__term_to_doc = fdbo_root.get_dir(Fdbl_BijU128_Dir::bytes_to_u128).await?;

        for term in terms.iter() {
            fdbo__doc_to_term.set(&(&doc_id, &term), &[]);
            fdbo__term_to_doc.set(&(&term, &doc_id), &[]);
        }

        trx.commit().await.map_err(|e| *e)?;
        Ok(())
    }

    async fn doc_del<'a>(path: String, doc_id: Vec<u8>) -> Result<(), Sa_Err> {
        /*
        let trx: Transaction = _G_S_Ffi.get().unwrap().db.create_trx()?;
        let fdbo_root = Fdbl_BijU128_Obj::new(&trx, &path).await?;
        let fdbo__doc_to_term = fdbo_root.get_dir(Fdbl_BijU128_Dir::u128_to_bytes).await?;
        let fdbo__term_to_doc = fdbo_root.get_dir(Fdbl_BijU128_Dir::bytes_to_u128).await?;

        let data = fdbo__doc_to_term.get_all_in_subspace(&doc_id).await?;
        for fdb_value in data {
            let (doc_id, term) = fdbo__doc_to_term.unpack::<(Vec<u8>, Vec<u8>)>(fdb_value.key())??;
            fdbo__doc_to_term.clear(&(&doc_id, &term));
            fdbo__term_to_doc.clear(&(&term, &doc_id));
        }

        trx.commit().await.map_err(|e| *e)?;
        Ok(())

         */
        todo!()
    }

    async fn term_lookup<'a>(path: String, term: Vec<u8>) -> Result<Vec<Vec<u8>>, Sa_Err> {
        /*
        let trx: Transaction = _G_S_Ffi.get().unwrap().db.create_trx()?;
        let fdbo_root = Fdbl_BijU128_Obj::new(&trx, &path).await?;
        let fdbo__term_to_doc = fdbo_root.get_dir(Fdbl_BijU128_Dir::bytes_to_u128).await?;

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

         */
        todo!()
    }
}

#[rustler::nif]
pub fn inv_index__doc_add<'a>(env: rustler::Env<'a>, pid: rustler::LocalPid, path: String, doc_id: rustler::Binary<'a>, terms: Vec<rustler::Binary<'a>>) {
    let doc_id = doc_id.as_slice().to_vec();
    let terms = terms.iter().map(|x| x.as_slice().to_vec()).collect::<Vec<_>>();
    let sffi_global = _G_S_Ffi.get().unwrap();
    G_S_Ffi::run_fdbt(pid, Fdbl_BijU128_Util::doc_add(path, doc_id, terms));
}

#[rustler::nif]
pub fn inv_index__doc_del<'a>(env: rustler::Env<'a>, pid: rustler::LocalPid, path: String, doc_id: rustler::Binary<'a>) {
    /*
    let doc_id = doc_id.as_slice().to_vec();
    let sffi_global = _G_S_Ffi.get().unwrap();
    G_S_Ffi::run_fdbt(pid, Fdbl_BijU128_Util::doc_del(path, doc_id));

     */
    todo!()
}

#[rustler::nif]
pub fn inv_index__term_lookup<'a>(env: rustler::Env<'a>, pid: rustler::LocalPid, path: String, term: rustler::Binary<'a>) {
    /*
    let term = term.as_slice().to_vec();
    let sffi_global = _G_S_Ffi.get().unwrap();
    G_S_Ffi::run_fdbt(pid, Fdbl_BijU128_Util::term_lookup(path, term));

     */
    todo!()
}
