use super::*;

#[derive(Serialize, Deserialize, JsData, Debug)]
pub struct Cmsg_Full {
    pub inner: Cmsg_Inner,
    pub msg_type: Cmsg_Type,
}

impl Cmsg_Full {
    pub fn to_js_value(&self) -> JsValue {
        let dst = self.inner.id_proc();

        let (bytes_r, t_vd) = {
            let mut bytes = vec![];
            let mut bw = BufWriter::new(&mut bytes);
            let mut t_vd = VecDeque::new();
            let _ = <Self as T_JsData_>::write_to_js(&self, &mut bw, &mut t_vd);
            drop(bw);
            (bytes, t_vd)
        };

        let out = js_sys::Object::new();
        let transfers = {
            let out = js_sys::Array::new_with_length(t_vd.len() as u32);
            for (i, x) in t_vd.iter().enumerate() {
                js_sys::Array::set(&out, i as u32, x.clone())
            }
            out
        };

        let bytes: JsValue = {
            let array = Uint8Array::from(&bytes_r[..]);
            array.into()
        };

        js_sys::Reflect::set(&out, &"to".into(), &JsValue::from(dst.to_name())).unwrap();
        js_sys::Reflect::set(&out, &"rust_part".into(), &bytes.into()).unwrap();
        js_sys::Reflect::set(&out, &"transfers".into(), &transfers.into()).unwrap();

        out.into()
    }

    pub fn from_js_value(t: &JsValue) -> Result<Cmsg_Full, L_Serde_Err> {
        let t = js_sys::Reflect::get(&t, &"data".into()).map_err(|x| L_Serde_Err::Missing_Data)?;
        let rust_part = js_sys::Reflect::get(&t, &"rust_part".into()).map_err(|_x| L_Serde_Err::Missing_RustPart)?;
        let transfers = js_sys::Reflect::get(&t, &"transfers".into()).map_err(|_x| L_Serde_Err::Transfers_Missing)?;

        if transfers.is_null() || transfers.is_undefined() {
            Err(L_Serde_Err::Transfers_Is_Null)?
        }
        let inner = js_sys::Array::from(&transfers);

        let mut transfers = {
            let mut out = VecDeque::new();
            let n = inner.length();
            for i in 0..n {
                out.push_back(inner.get(i));
            }
            out
        };

        let bytes_vec: Vec<u8> = Uint8Array::new(&rust_part).to_vec();

        let mut buf_reader = BufReader::new(bytes_vec.as_slice());

        <Self as T_JsData_>::read_from_js(&mut buf_reader, &mut transfers)
    }
}
