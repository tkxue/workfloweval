use super::*;

#[derive(JsData, Debug, Clone)]
pub struct Err_Frame {
    pub file: String,
    pub line: u32,
    pub msg: String,
}

pub trait S_Unwrap_T<V> {
    fn un(self, err_msg: Err_Frame) -> V;
}

impl<T> S_Unwrap_T<T> for Option<T> {
    fn un(self, err_msg: Err_Frame) -> T {
        match self {
            None => {
                Log_Err_Util::over_http(&Err_Stack::new(err_msg.clone()));
                todo!("{:?}", err_msg)
            }
            Some(v) => v,
        }
    }
}

impl<T, E> S_Unwrap_T<T> for Result<T, E> {
    fn un(self, err_msg: Err_Frame) -> T {
        match self {
            Ok(v) => v,
            Err(_) => {
                Log_Err_Util::over_http(&Err_Stack::new(err_msg.clone()));
                todo!("{:?}", err_msg)
            }
        }
    }
}

pub struct U64_Util {}

impl U64_Util {
    #[inline(always)]
    pub fn with_set_bit(x: u64, idx: usize, b: bool) -> u64 {
        (x & !(1_u64 << idx)) | ((b as u64) << idx)
    }

    #[inline(always)]
    pub fn get_bit(x: u64, idx: usize) -> bool {
        let flag = 1_u64 << idx;
        x & flag != 0
    }

    pub fn set_bit(x: &mut u64, idx: usize, b: bool) {
        *x = Self::with_set_bit(*x, idx, b);
    }
}

pub struct U32_Util {}

impl U32_Util {
    #[inline(always)]
    pub fn with_set_bit(x: u32, idx: usize, b: bool) -> u32 {
        (x & !(1_u32 << idx)) | ((b as u32) << idx)
    }

    #[inline(always)]
    pub fn get_bit(x: u32, idx: usize) -> bool {
        let flag = 1_u32 << idx;
        x & flag != 0
    }

    pub fn set_bit(x: &mut u32, idx: usize, b: bool) {
        *x = Self::with_set_bit(*x, idx, b);
    }
}

pub struct U8_Util {}

impl U8_Util {
    pub fn conv_bytes<T>(v: &[T]) -> &[u8] {
        unsafe { std::slice::from_raw_parts(v.as_ptr() as *const u8, v.len() * std::mem::size_of::<T>()) }
    }

    #[inline(always)]
    pub fn with_set_bit(x: u8, idx: usize, b: bool) -> u8 {
        (x & !(1_u8 << idx)) | ((b as u8) << idx)
    }

    #[inline(always)]
    pub fn get_bit(x: u8, idx: usize) -> bool {
        let flag = 1_u8 << idx;
        x & flag != 0
    }

    pub fn set_bit(x: &mut u8, idx: usize, b: bool) {
        *x = Self::with_set_bit(*x, idx, b);
    }
}

pub struct Log_Err_Util {}

impl Log_Err_Util {
    pub fn url_str(err_msg: &Err_Frame) -> String {
        #[allow(deprecated)]
        let b = base64::encode(err_msg.file.as_bytes());
        let url = format!("http://localhost:3020/goto-file/{}/{}", err_msg.line, b);
        url
    }

    pub fn over_http(err_msg: &Err_Stack) {
        Xdom_Logger::log_s(&err_msg);
    }
}
