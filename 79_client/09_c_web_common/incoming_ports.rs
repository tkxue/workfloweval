use super::*;

/*
pub struct Incoming_Ports {
    f: Arc<dyn Fn(Id_Proc, XdomA_Message_Event) + Send + Sync>,
    _incoming_ports: Per_Proc<XdomA_Message_Port_Cb>,
}

impl Incoming_Ports {
    pub fn new(r: Arc<dyn Fn(Id_Proc, XdomA_Message_Event) + Send + Sync>) -> Incoming_Ports {
        Incoming_Ports {
            f: r.clone(),
            _incoming_ports: Per_Proc { inner: HashMap::new() },
        }
    }

    pub fn set(&mut self, k: Id_Proc, v: XdomA_Message_Port) {
        self._incoming_ports.inner.insert(
            k,
            v.into_cb({
                let r = self.f.clone();
                Arc::new(move |e| (r)(k, e))
            }),
        );
    }
}

// ==================

#[derive(Clone, JsData)]
pub struct Per_Proc<T> {
    pub inner: HashMap<Id_Proc, T>,
}

impl<V: 'static> Per_Proc<V> {
    pub fn get(&self, t: Id_Proc) -> &V {
        match self.inner.get(&t) {
            None => {
                damn_it!("Unable to get: {:?}, available keys: {:?}", t, self.inner.keys())
            }
            Some(x) => x,
        }
    }
}


 */
