use super::*;
use anyhow::anyhow;
use foundationdb::directory::{Directory, DirectoryError, DirectoryLayer, DirectoryOutput};
use foundationdb::options::{StreamingMode, TransactionOption};
use foundationdb::tuple::PackResult;
use foundationdb::{RangeOption, Transaction};
use rustler::Encoder;
use std::cell::OnceCell;
use std::io::BufRead;

pub struct KVShell_Util {}

pub struct KVShell_Obj<'a> {
    obj_root: Sa_Dir<'a>,
}

impl<'a> KVShell_Obj<'a> {
    pub async fn new(trx: &'a Transaction, path: &String) -> Result<KVShell_Obj<'a>, DirectoryError> {
        let obj_root = Xdb_Util::get_directory(&trx, &Xdb_Util::split_string(&path)).await?;
        Ok(KVShell_Obj { obj_root: obj_root })
    }

    /*
    pub async fn get_dir(&self, dir: KVShell_Dir) -> Result<Xdb_Dir<'a>, DirectoryError> {
        self.obj_root.open_str(dir.to_str()).await
    }
     */
}

rustler::atoms! {
    ok,
    err
}

impl KVShell_Util {
    async fn ls<'a>(path: String) -> Result<Vec<String>, DirectoryError> {
        let trx: Transaction = _G_S_Ffi.get().unwrap().db.create_trx()?;
        let parts = &Xdb_Util::split_string(&path);
        let root = DirectoryLayer::default();
        let data = root.list(&trx, &parts).await?;
        trx.commit().await.map_err(|e| *e)?;
        Ok(data)
    }

    async fn rm<'a>(path: String) -> Result<(), DirectoryError> {
        let trx: Transaction = _G_S_Ffi.get().unwrap().db.create_trx()?;
        let parts = &Xdb_Util::split_string(&path);
        let root = DirectoryLayer::default();
        let data = root.list(&trx, &parts).await?;
        let res = if data.len() > 0 {
            Err(DirectoryError::Other("Refusing to deleted non empty dir.".to_string()))
        } else {
            root.remove(&trx, &parts).await?;
            Ok(())
        };
        trx.commit().await.map_err(|e| *e)?;
        res
    }

    /*
    async fn doc_add<'a>(
        path: String,
        doc_id: Vec<u8>,
        terms: Vec<Vec<u8>>,
    ) ->
     Result<(), DirectoryError>
     {
        let trx: Transaction = G_S_Ffi.get().unwrap().db.create_trx()?;
        let fdbo_root = KVShell_Obj::new(&trx, &path).await?;
        let fdbo__doc_to_term = fdbo_root.get_dir(KVShell_Dir::doc_to_term).await?;
        let fdbo__term_to_doc = fdbo_root.get_dir(KVShell_Dir::term_to_doc).await?;

        for term in terms.iter() {
            fdbo__doc_to_term.set(&(&doc_id, &term), &[]);
            fdbo__term_to_doc.set(&(&term, &doc_id), &[]);
        }

        trx.commit().await.map_err(|e| *e)?;
        Ok(())
    }

    async fn doc_del<'a>(path: String, doc_id: Vec<u8>) -> Result<(), DirectoryError> {
        let trx: Transaction = G_S_Ffi.get().unwrap().db.create_trx()?;
        let fdbo_root = KVShell_Obj::new(&trx, &path).await?;
        let fdbo__doc_to_term = fdbo_root.get_dir(KVShell_Dir::doc_to_term).await?;
        let fdbo__term_to_doc = fdbo_root.get_dir(KVShell_Dir::term_to_doc).await?;

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
        let fdbo_root = KVShell_Obj::new(&trx, &path).await?;
        let fdbo__term_to_doc = fdbo_root.get_dir(KVShell_Dir::term_to_doc).await?;

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

#[rustler::nif]
pub fn kv_dir__ls<'a>(env: rustler::Env<'a>, pid: rustler::LocalPid, path: String) {
    let sffi_global = _G_S_Ffi.get().unwrap();
    G_S_Ffi::run_fdbt(pid, KVShell_Util::ls(path));
}

#[rustler::nif]
pub fn kv_dir__rm<'a>(env: rustler::Env<'a>, pid: rustler::LocalPid, path: String) {
    let sffi_global = _G_S_Ffi.get().unwrap();
    G_S_Ffi::run_fdbt(pid, KVShell_Util::rm(path));
}

/*
pub enum KVShell_Dir {
    term_to_doc,
    doc_to_term,
}

impl KVShell_Dir {
    pub fn to_str(&self) -> &str {
        match self {
            KVShell_Dir::term_to_doc => "term_to_doc",
            KVShell_Dir::doc_to_term => "doc_to_term",
        }
    }
}
 */

/*
#[rustler::nif]
pub fn dir_util__doc_add<'a>(
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
    sffi_global.run_fdbt(pid, KVShell_Util::doc_add(path, doc_id, terms));
}

#[rustler::nif]
pub fn dir_util__doc_del<'a>(
    env: rustler::Env<'a>,
    pid: rustler::LocalPid,
    path: String,
    doc_id: rustler::Binary<'a>,
) {
    let doc_id = doc_id.as_slice().to_vec();
    let sffi_global = G_S_Ffi.get().unwrap();
    sffi_global.run_fdbt(pid, KVShell_Util::doc_del(path, doc_id));
}


*/
