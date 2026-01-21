use super::*;

pub struct HVec<K: T_BigEnum, V> {
    inner: Vec<V>,
    _t: PhantomData<K>,
}

impl<K: T_BigEnum, V: Clone> Clone for HVec<K, V> {
    fn clone(&self) -> Self {
        HVec {
            inner: self.inner.clone(),
            _t: Default::default(),
        }
    }
}

impl<K: T_BigEnum, V> HVec<K, V> {
    pub fn get(&self, k: K) -> &V {
        &self.inner[k.to_u16().inner as usize]
    }

    pub fn get_mut(&mut self, k: K) -> &mut V {
        &mut self.inner[k.to_u16().inner as usize]
    }

    pub fn set(&mut self, k: K, v: V) {
        self.inner[k.to_u16_().inner as usize] = v;
    }

    pub fn new(x: Vec<V>) -> Option<HVec<K, V>> {
        if x.len() == K::NUM_ARMS as usize {
            Some(HVec {
                inner: x,
                _t: Default::default(),
            })
        } else {
            None
        }
    }

    pub fn into_arc(self) -> HArc<K, V> {
        HArc {
            inner: Arc::from(self.inner),
            _t: Default::default(),
        }
    }
}

pub struct HArc<K: T_BigEnum, V> {
    inner: Arc<[V]>,
    _t: PhantomData<K>,
}

impl<K: T_BigEnum, V> HArc<K, V> {
    pub fn get(&self, k: K) -> &V {
        &self.inner[k.to_u16_().inner as usize]
    }
}

impl<K: T_BigEnum, V> Clone for HArc<K, V> {
    fn clone(&self) -> Self {
        HArc {
            inner: self.inner.clone(),
            _t: Default::default(),
        }
    }
}

impl<K: T_BigEnum, V: T_JsData_> T_JsData_ for HVec<K, V> {
    fn write_to_js(&self, writer: T_JsData_Write, transfers: &mut VecDeque<wb::JsValue>) -> Result<(), L_JsData_Err> {
        self.inner.write_to_js(writer, transfers)
    }

    fn read_from_js(reader: T_JsData_Read, transfers: &mut VecDeque<wb::JsValue>) -> Result<Self, L_JsData_Err>
    where
        Self: Sized,
    {
        let t = <Vec<V> as T_JsData_>::read_from_js(reader, transfers)?;
        if t.len() == (K::NUM_ARMS as usize) {
            Ok(HVec::new(t).unwrap())
        } else {
            Err(L_JsData_Err::HVec_Wrong_Size)
        }
    }
}
