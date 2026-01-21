use super::*;

pub struct To_Js {}

impl To_Js {
    pub fn arr(v: Vec<wb::JsValue>) -> wb::JsValue {
        let out = js_sys::Array::new_with_length(v.len() as u32);
        for (i, x) in v.iter().enumerate() {
            js_sys::Array::set(&out, i as u32, x.clone())
        }
        out.into()
    }

    pub fn obj(v: Vec<(String, wb::JsValue)>) -> wb::JsValue {
        let obj = js_sys::Object::new();
        for (s, v) in v.iter() {
            js_sys::Reflect::set(&obj, &s.into(), v).un(err!(""));
        }
        obj.into()
    }
}

pub struct From_Js {}

impl From_Js {
    pub fn arr(x: wb::JsValue) -> Option<Vec<wb::JsValue>> {
        if x.is_null() || x.is_undefined() {
            return None;
        }
        let inner = js_sys::Array::from(&x);
        if inner.is_null() || inner.is_undefined() {
            return None;
        }

        let mut out = vec![];
        let n = js_sys::Array::length(&inner);

        for i in 0..n {
            out.push(js_sys::Array::get(&inner, i).clone());
        }

        Some(out)
    }
}
