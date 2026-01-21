use super::*;
use foundationdb::directory::{Directory, DirectoryError, DirectoryLayer, DirectoryOutput, DirectorySubspace};
use foundationdb::future::{FdbSlice, FdbValues};
use foundationdb::options::StreamingMode;
use foundationdb::tuple::{PackResult, Subspace, TuplePack, TupleUnpack};
use foundationdb::{RangeOption, Transaction};
use std::ops::Deref;

pub struct Xdb_Util {}

impl Xdb_Util {
    pub fn split_string(s: &str) -> Vec<String> {
        s.split('/').map(|s| s.trim()).filter(|s| !s.is_empty()).map(|s| s.to_string()).collect()
    }

    pub async fn get_directory<'a>(trx: &'a Transaction, parts: &[String]) -> Result<Sa_Dir<'a>, DirectoryError> {
        /*
        let root: DirectoryLayer = DirectoryLayer::default();
        if parts.len() == 0 {
            Ok(Sa_Dir {
                trx: trx,
                dir: DirectoryOutput::DirectorySubspace(DirectorySubspace::new(&[], vec![], &root, vec![])),
            })
        } else {
            Ok(Sa_Dir {
                trx: trx,
                dir: root.create_or_open(&trx, parts, None, None).await?,
            })
        }

         */
        todo!()
    }
}

pub struct O_Xdb_Dir<'a> {
    inner: Option<Sa_Dir<'a>>,
}

impl<'a> O_Xdb_Dir<'a> {
    pub fn get(&self) -> Result<&Sa_Dir<'a>, DirectoryError> {
        match &self.inner {
            None => Err(DirectoryError::Other("Directory Not Loaded".to_string())),
            Some(x) => Ok(x),
        }
    }
}

pub struct Xdb_Path {
    pub data: Vec<String>,
}
