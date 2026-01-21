use super::*;

use e_api::*;
use foundationdb::directory::{Directory, DirectoryError, DirectoryLayer, DirectoryOutput, DirectorySubspace};
use foundationdb::future::{FdbSlice, FdbValues};
use foundationdb::options::StreamingMode;
use foundationdb::tuple::{PackResult, Subspace, TuplePack, TupleUnpack};
use foundationdb::{FdbError, RangeOption, Transaction};
use serde::{Deserialize, Serialize};

pub struct Sa_Dir<'a> {
    pub trx: &'a Transaction,
    pub path: String,
    pub dir: DirectoryOutput,
}

impl<'a> Sa_Dir<'a> {
    pub async fn new(trx: &'a Transaction, path: &str) -> Result<Sa_Dir<'a>, Sa_Err> {
        let parts: Vec<String> = Xdb_Util::split_string(&path);
        let root: DirectoryLayer = DirectoryLayer::default();

        if parts.len() == 0 {
            Ok(Sa_Dir {
                path: path.to_string(),
                trx: trx,
                dir: DirectoryOutput::DirectorySubspace(DirectorySubspace::new(&[], vec![], &root, vec![])),
            })
        } else {
            Ok(Sa_Dir {
                path: path.to_string(),
                trx: trx,
                dir: root
                    .create_or_open(&trx, &parts, None, None)
                    .await
                    .ctx(&format!("Sa_Dir::new {}", path))?,
            })
        }
    }

    /*
    pub async fn open(&self, p: &Xdb_Path) -> Result<Sa_Dir<'a>, DirectoryError> {
        Ok(Sa_Dir {
            dir: self.dir.open(self.trx, &p.data, None).await?,
            trx: self.trx,
        })
    }

    pub async fn open_str(&self, s: &str) -> Result<Sa_Dir<'a>, DirectoryError> {
        let xdb_path = Xdb_Path {
            data: Xdb_Util::split_string(s),
        };
        Ok(self.open(&xdb_path).await?)
    }

     */

    pub fn set<K: TuplePack>(&self, k: &K, v: &[u8]) -> Result<(), Sa_Err> {
        self.trx
            .set(&self.dir.pack(k).ctx(&format!("Sa_Dir::set, path = {}", self.path))?, v);
        Ok(())
    }

    pub async fn get<K: TuplePack>(&self, k: &K) -> Result<Option<FdbSlice>, Sa_Err> {
        let k = &self.dir.pack(k).ctx(&format!("could not dir.pack"))?;
        let x = self
            .trx
            .get(k, false)
            .await
            .ctx(&format!("could not read: {:?}", k.as_slice()))?;
        Ok(x)
    }

    pub fn clear<K: TuplePack>(&self, k: &K) -> Result<(), Sa_Err> {
        /*
        self.trx.clear(&self.dir.pack(k)?);
        Ok(())

         */
        todo!()
    }

    pub fn subspace<T: TuplePack>(&self, t: &T) -> Result<Subspace, Sa_Err> {
        // self.dir.subspace(t)
        todo!()
    }

    pub fn unpack<'de, T: TupleUnpack<'de>>(&self, t: &'de [u8]) -> Result<PackResult<T>, Sa_Err> {
        // self.dir.unpack(t)
        todo!()
    }

    pub async fn get_all_in_subspace<T: TuplePack>(&self, t: &T) -> Result<FdbValues, Sa_Err> {
        let err_ctx = format!("get_all_in_subspace, path = {}", &self.path);
        let subspace = self.dir.subspace(&t).ctx(&err_ctx)?;
        let mut opt = RangeOption::from(subspace.range());
        opt.mode = StreamingMode::WantAll;
        let data = self.trx.get_range(&opt, 0, false).await.ctx(&err_ctx)?;
        Ok(data)
    }
}

pub trait Sa_Struct_T<'a> {
    fn new_at(trx: &'a Transaction, path: &[String]);

    fn loc_at(trx: &'a Transaction, path: &[String]) -> Self
    where
        Self: Sized;
}

pub struct Sa_Loc {
    path: String,
    key: String,
    msg: String,
}

pub trait My_Context_T<T> {
    fn ctx(self, msg: &str) -> Result<T, Sa_Err>;
}

impl<T> My_Context_T<T> for Result<T, Sa_Err> {
    fn ctx(self, msg: &str) -> Self {
        self.map_err(|x| {
            let mut obj = x;
            obj.ctxs.push(msg.to_string());
            obj
        })
    }
}

impl<T> My_Context_T<T> for Result<T, DirectoryError> {
    fn ctx(self, msg: &str) -> Result<T, Sa_Err> {
        self.map_err(|x| Sa_Err {
            ctxs: vec![msg.to_string()],
            err: Sa_ErrType::Fdb(format!("{:?}", x)),
        })
    }
}

impl<T> My_Context_T<T> for Result<T, FdbError> {
    fn ctx(self, msg: &str) -> Result<T, Sa_Err> {
        self.map_err(|x| Sa_Err {
            ctxs: vec![msg.to_string()],
            err: Sa_ErrType::Fdb(format!("{:?}", DirectoryError::FdbError(x))),
        })
    }
}
