use super::*;

use foundationdb::directory::DirectoryError;
use foundationdb::future::FdbSlice;
use std::marker::PhantomData;
use std::sync::Arc;

pub trait Sb_Value_T: Default {
    fn to_bytes(&self) -> Vec<u8>;

    fn from_bytes(data: &[u8]) -> Result<Self, Sa_Err>
    where
        Self: Sized;
}

impl Sb_Value_T for i128 {
    fn to_bytes(&self) -> Vec<u8> {
        let x = self.to_be_bytes().to_vec();
        x
    }

    fn from_bytes(data: &[u8]) -> Result<Self, Sa_Err>
    where
        Self: Sized,
    {
        let array: [u8; 16] = data
            .try_into()
            .map_err(|_| Sa_Err {
                ctxs: vec![],
                err: Sa_ErrType::DeserializeFail,
            })
            .ctx("i128::from_bytes")?;
        Ok(i128::from_be_bytes(array))
    }
}

pub struct Sb_Value<'a, T: Sb_Value_T> {
    dir: Arc<Sa_Dir<'a>>,
    name: String,
    _t: PhantomData<T>,
}

impl<'a, T: Sb_Value_T> Sb_Value<'a, T> {
    pub fn new(dir: &Arc<Sa_Dir<'a>>, name: &str) -> Sb_Value<'a, T> {
        Sb_Value {
            dir: dir.clone(),
            name: name.to_string(),
            _t: Default::default(),
        }
    }

    pub async fn get(&self) -> Result<T, Sa_Err> {
        let t = self.dir.get(&self.name).await?;
        match t {
            None => Ok(T::default()),
            Some(e) => match T::from_bytes(e.as_ref()) {
                Ok(v) => Ok(v),
                Err(_) => Err(Sa_Err {
                    ctxs: vec![format!("Sb_Value cant deserialize: {:?}", self.name)],
                    err: Sa_ErrType::DeserializeFail,
                }),
            },
        }
    }

    pub fn set(&self, t: T) -> Result<(), Sa_Err> {
        self.dir.set(&self.name, &t.to_bytes())
    }
}

pub struct Sc_Map<'a, K: Sb_Value_T, V: Sb_Value_T> {
    t: &'a PhantomData<(K, V)>,
}

pub struct Sc_MultiMap<'a, K: Sb_Value_T, V: Sb_Value_T> {
    t: &'a PhantomData<(K, V)>,
}
