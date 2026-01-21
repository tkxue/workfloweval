use super::*;

#[macro_export]
macro_rules! err {
    ($($t:expr),*) => (
        {
            {

            let err_msg = format!( $($t),* );
            Err_Frame {
                file: file!().to_string(),
                line: line!(),
                msg: err_msg,
            }
            }
        } ) }

#[macro_export]
macro_rules! err_jsv {
    ( ) => {{
        {
            |e: wb::JsValue| Err_Frame {
                file: file!().to_string(),
                line: line!(),
                msg: format!("{:?}", e),
            }
        }
    }};
}

#[macro_export]
macro_rules! wlog {
    ($($tt:expr),*) => {
        Xdom_Logger::log_s(
            &Err_Stack::new(
            err!($($tt),*)
            )
        );
    };
}

#[macro_export]
macro_rules! elog {
    ($($tt:expr),*) => {
        Xdom_Logger::log_s(
            &Err_Stack::new(
            err!($($tt),*)
            )
        );
    };
}

#[cfg(not(target_arch = "wasm32"))]
#[macro_export]
macro_rules! damn_it {
    ($($t:expr),*) => (
        todo!( $($t),* )

         ) }

#[cfg(target_arch = "wasm32")]
#[macro_export]
macro_rules! damn_it {
    ($($t:expr),*) => (
        {
            let err_msg = err!( $($t),* );
            Log_Err_Util::over_http(&
            &Err_Stack::new(

            err_msg.clone()
            )

            );
            todo!("{}", err_msg.msg);
        } ) }

#[macro_export]
macro_rules! my_eq {
    ($lhs:expr, $rhs:expr, $aux:expr) => {
        let lhs = $lhs;
        let rhs = $rhs;
        if !(lhs == rhs) {
            let aux = $aux;
            damn_it!("eq failed {:?} != {:?}\naux: {}", lhs, rhs, aux);
        }
    };
}

#[macro_export]
macro_rules! my_lt {
    ($lhs:expr, $rhs:expr, $aux:expr) => {
        let lhs = $lhs;
        let rhs = $rhs;
        if !(lhs < rhs) {
            let aux = $aux;
            damn_it!("lt failed {:?} < {:?}\naux: {}", lhs, rhs, aux);
        }
    };
}

#[macro_export]
macro_rules! my_le {
    ($lhs:expr, $rhs:expr, $aux:expr) => {
        let lhs = $lhs;
        let rhs = $rhs;
        if !(lhs <= rhs) {
            let aux = $aux;
            damn_it!("le failed {:?} <= {:?}\naux: {}", lhs, rhs, aux);
        }
    };
}

#[macro_export]
macro_rules! un_res {
    ($lhs:expr ) => {
        match $lhs {
            Ok(x) => x,
            Err(e) => damn_it!("{:?}", e),
        }
    };
}

#[macro_export]
macro_rules! un_opt {
    ($lhs:expr ) => {
        match $lhs {
            Some(x) => x,
            None => damn_it!("un_opt!(None)"),
        }
    };
}

#[macro_export]
macro_rules! console_log {
    ($($t:expr),*) => (
        {
            let err_msg = format!( $($t),* );
            Log_Err_Util::over_http(file!(), line!(), &err_msg);
        } ) }
