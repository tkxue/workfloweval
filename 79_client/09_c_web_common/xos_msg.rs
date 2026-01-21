use super::*;
use e_api::byteorder::WriteBytesExt;
use e_api::byteorder::{LittleEndian, ReadBytesExt};

pub struct Xos_Msg {
    pub id: Id_Proc,
    pub rust_part: Xos_Jab,
    pub transfers: VecDeque<wb::JsValue>,
}

/*
pub struct Xos_T_Msg<T> {
    pub inner: Xos_Msg,
    pub t: PhantomData<T>,
}
 */

impl T_JsData_ for Xos_Msg {
    fn write_to_js(&self, writer: T_JsData_Write, transfers: &mut VecDeque<wb::JsValue>) -> Result<(), L_JsData_Err>
    where
        Self: Sized,
    {
        // writer.write_u32::<LittleEndian>(self.tag as u32).unwrap();
        writer.write_u32::<LittleEndian>(self.transfers.len() as u32).unwrap();

        transfers.push_back(self.rust_part.clone().inner.into());

        for x in self.transfers.iter() {
            transfers.push_back(x.clone());
        }

        Ok(())
    }

    fn read_from_js(reader: T_JsData_Read, transfers: &mut VecDeque<wb::JsValue>) -> Result<Self, L_JsData_Err>
    where
        Self: Sized,
    {
        // let tag = reader.read_u32::<LittleEndian>().unwrap(); // un(err!("could not read tag"));
        let n_abs = reader.read_u32::<LittleEndian>().unwrap(); // un(err!("could not read abs.len"));
        let n_transfers = reader.read_u32::<LittleEndian>().unwrap(); // un(err!("could not read transfers.len()"));

        let rust_part = Xos_Jab::new_jsv(&transfers.pop_front().unwrap());

        let mut o_transfers = VecDeque::new();
        for _i in 0..(n_transfers as usize) {
            o_transfers.push_back(transfers.pop_front().unwrap())
        }
        Ok(Xos_Msg {
            // tag,
            id: Id_Proc::H_Gfx,
            rust_part,
            transfers: o_transfers,
        })
    }

    /*
    fn write_to_buf(&self, writer: JsData_Write) -> Result<(), JsData_Err> {
        todo!("")
    }

    fn read_from_buf(reader: JsData_Read) -> Result<Self, JsData_Err>
    where
        Self: Sized,
    {
        todo!("")
    }

     */
}

#[derive(Debug)]
pub enum Err_Xos_Msg {
    Missing__Rust_Part,
    Missing__Transfers,
    Missing__Non_Transfers,
    JsArray__Null_Undefined,
    JsArray_Inner__Null_Undefined,
}

impl Display for Err_Xos_Msg {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Xos_Msg {
    pub fn to_typed<T: T_JsData_>(&self) -> Result<T, L_JsData_Err> {
        T::from_xos_raw_msg(self)
    }

    pub fn get_transfers(&self) -> js_sys::Array {
        let transfers = js_sys::Array::new_with_length(self.transfers.len() as u32);
        for (i, x) in self.transfers.iter().enumerate() {
            js_sys::Array::set(&transfers, i as u32, x.clone())
        }
        transfers
    }

    pub fn to_js(&self) -> wb::JsValue {
        let out = js_sys::Object::new();

        let transfers = js_sys::Array::new_with_length(self.transfers.len() as u32);
        for (i, x) in self.transfers.iter().enumerate() {
            js_sys::Array::set(&transfers, i as u32, x.clone())
        }

        js_sys::Reflect::set(&out, &"rust_part".into(), &self.rust_part.inner.clone().into()).unwrap();
        js_sys::Reflect::set(&out, &"transfers".into(), &transfers.into()).unwrap();

        out.into()
    }

    pub fn from_js(v: wb::JsValue) -> Result<Xos_Msg, Err_Xos_Msg> {
        let rust_part = js_sys::Reflect::get(&v, &"rust_part".into()).map_err(|_x| Err_Xos_Msg::Missing__Rust_Part)?;
        let transfers = js_sys::Reflect::get(&v, &"transfers".into()).map_err(|_x| Err_Xos_Msg::Missing__Transfers)?;
        let transfers = Xos_Js_Array::new_jsv(&transfers)?.to_vec();
        let non_transfers = js_sys::Reflect::get(&v, &"non_transfers".into()).map_err(|_x| Err_Xos_Msg::Missing__Non_Transfers)?;
        // let tag = js_sys::Reflect::get(&v, &"tag".into()).map_err?;
        let non_transfers = Xos_Js_Array::new_jsv(&non_transfers)?.to_vec();

        Ok(Xos_Msg {
            // tag: tag.as_f64().unwrap() as u32,
            id: Id_Proc::H_Gfx,
            rust_part: Xos_Jab::new_jsv(&rust_part),
            transfers: transfers.into(),
        })
    }
}

impl std::fmt::Debug for Xos_Msg {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[Xos_Raw_Msg")
    }
}

pub trait JsData_Dyn {
    fn from_xos_raw_msg(xos_raw_msg: &Xos_Msg) -> Result<Self, L_JsData_Err>
    where
        Self: Sized;
}

impl<T: T_JsData_> JsData_Dyn for T {
    fn from_xos_raw_msg(xos_raw_msg: &Xos_Msg) -> Result<Self, L_JsData_Err>
    where
        Self: Sized,
    {
        let v = xos_raw_msg.rust_part.to_vec();
        let mut v_vals = xos_raw_msg.transfers.clone();

        let ans = {
            let mut buf = BufReader::new(v.as_slice());
            Self::read_from_js(&mut buf, &mut v_vals)?
        };

        Ok(ans)
    }
}

pub fn conv_xos_raw_msg(obj: &dyn T_JsData_) -> Result<Xos_Msg, L_JsData_Err> {
    let mut v = vec![];
    let mut v_vals = VecDeque::new();
    {
        let mut buf = BufWriter::new(&mut v);
        obj.write_to_js(&mut buf, &mut v_vals)?;
    }

    let array_buffer = Xos_Jab::new_slice(v.as_slice());

    Ok(Xos_Msg {
        // tag,
        id: Id_Proc::H_Gfx,
        rust_part: array_buffer,
        transfers: v_vals,
    })
}

#[derive(Clone)]
pub struct Xos_Js_Array {
    pub inner: js_sys::Array,
}

impl Xos_Js_Array {
    pub fn new_jsv(x: &wb::JsValue) -> Result<Xos_Js_Array, Err_Xos_Msg> {
        if x.is_null() || x.is_undefined() {
            Err(Err_Xos_Msg::JsArray__Null_Undefined)?
        }

        let inner = js_sys::Array::from(x);

        if x.is_null() || x.is_undefined() {
            Err(Err_Xos_Msg::JsArray_Inner__Null_Undefined)?
        }

        Ok(Xos_Js_Array { inner })
    }

    pub fn to_vec(&self) -> Vec<wb::JsValue> {
        let mut out = vec![];
        let n = self.length();
        for i in 0..n {
            out.push(self.get(i));
        }
        out
    }

    pub fn new_len(n: usize) -> Xos_Js_Array {
        Xos_Js_Array {
            inner: js_sys::Array::new_with_length(n as u32),
        }
    }

    pub fn set(&self, idx: usize, v: wb::JsValue) {
        self.inner.set(idx as u32, v);
    }

    pub fn length(&self) -> usize {
        self.inner.length() as usize
    }

    pub fn get(&self, i: usize) -> wb::JsValue {
        self.inner.get(i as u32)
    }

    pub fn to_jsvalue(&self) -> &wb::JsValue {
        self.inner.deref()
    }

    pub fn into_jsvalue(self) -> wb::JsValue {
        self.inner.into()
    }
}
