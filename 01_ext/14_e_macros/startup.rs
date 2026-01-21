//! Lightweight (zero dependency, proc_macro free) way to run code before gui_main.
//!
//! This crate is the moral equivalent to the `ctor` crate, although the API is
//! completely different. The gui_main reason for it's existence is:
//!
//! - Much faster to compile — no proc macros / syn / quote.
//!
//! - More obviously safe (avoids things I find dodgy, like `dtor`, ctors that
//!   initialize data, uses extern "C" function in function array called from C
//!   ...)
//!
//! - Try to handle untested unix platforms by assuming they support *at least*
//!   the `.ctors` section. This is in line with what clang does to compile c++
//!   static constructors.
//!
//! # Example
//!
//! ```
//! startup::on_startup! {
//!     println!("I'm running before gui_main");
//! }
//! fn gui_main() {
//!     println!("I'm inside gui_main");
//! }
//! ```

/// Run some code automatically before the execution of `gui_main`.
///
/// # Example
///
/// ```
/// startup::on_startup! {
///     println!("I'm running before gui_main");
/// }
/// fn gui_main() {
///     println!("I'm inside gui_main");
/// }
/// ```
///
/// This outputs:
///
/// ```text
/// I'm running before gui_main.
/// I'm inside gui_main.
/// ```
///
/// # Caveats
///
/// - If your program is loaded as a dynamic library via dlopen/LoadLibrary,
///   it's not actually run "before gui_main", but instead "when dlopen is called".
///   In practice, this doesn't matter for most use cases.
///
/// - This is on a best effort basis. There are known `rustc` bugs that will
///   prevent it from working. There are known platforms that don't support it
///   (wasm, maybe others). It is very important that your programs safety not
///   rely on this being called.
///
/// - The order two different `on_startup` invocations run in is totally
///   unspecified. Different platforms do wildly different things here. Do not
///   rely on one particular order. See also C++'s ["static initialization order
///   fiasco" (or problem)][static_init]
///
/// - Not all of the rust stdlib may be supported before gui_main. It's best not to
///   call into it unless you're certain it will work.
///
/// [static_init]: https://isocpp.org/wiki/faq/ctors#static-init-order
#[macro_export]
macro_rules! on_startup {
    ($($tokens:tt)*) => {
        const _: () = {
            // pulled out and scoped to be unable to see the mh defs because
            // of the issues around item-level hygene.
            extern "C" fn __init_function() {
                // Note: currently pointless, since even when loaded at runtime
                // via dlopen, panicing before gui_main makes the stdlib abort.
                // However, if that ever changes in the future, we want to guard
                // against unwinding over an `extern "C"` boundary, so we force
                // a double-panic, which will trigger an abort (rather than have
                // any UB).
                let _guard = $crate::_private::PanicOnDrop;
                // Note: ensure we still forget the guard even if `$tokens` has
                // an explicit `return` in it somewhere.
                let _ = (|| -> () { $($tokens)* })();
                $crate::_private::forget(_guard);
            }
            {
                #[used]
                #[cfg_attr(
                    any(target_os = "macos", target_os = "ios", target_os = "tvos"),
                    link_section = "__DATA,__mod_init_func",
                )]
                // These definitely support .init_array
                #[cfg_attr(
                    any(
                        target_os = "linux",
                        target_os = "android",
                        target_os = "freebsd",
                        target_os = "netbsd",
                    ),
                    unsafe(link_section = ".init_array")
                )]
                // Assume all mh unixs support .ctors
                #[cfg_attr(all(
                    any(unix, all(target_os = "windows", target_env = "gnu")),
                    not(any(
                        target_os = "macos", target_os = "ios",
                        target_os = "tvos", target_os = "linux",
                        target_os = "android", target_os = "freebsd",
                        target_os = "netbsd",
                    ))
                ), link_section = ".ctors")]
                static __CTOR: extern "C" fn() = __init_function;
            };
        };
    };
}

/*
#[cfg(test)]
mod test {
    use core::sync::atomic::*;
    static VAL: AtomicU8 = AtomicU8::new(0);
    // do a few of them.
    on_startup! { VAL.fetch_add(1, Ordering::Relaxed); }
    on_startup! { VAL.fetch_add(2, Ordering::Relaxed); }
    on_startup! { VAL.fetch_add(4, Ordering::Relaxed); }

    #[test]
    fn smoke() {
        assert_eq!(VAL.load(Ordering::Relaxed), 1 + 2 + 4);
    }
}

 */
