use super::*;

use rune::runtime::{Protocol, RuntimeContext, VmError, VmExecution, VmResult};
use rune::support::Context;
use rune::{
    Any, ContextError, Diagnostics, Module, Source, Sources, Value, Vm,
};
use std::collections::HashMap;
use std::sync::Arc;
use web_common::XdomA;

mod rune_repl_env;
mod rune_sdata;
pub use rune_repl_env::*;
pub use rune_sdata::*;

pub struct WwRune_Repl {
    context: rune::Context,
    runtime: Arc<RuntimeContext>,
    env: WwRune_ReplEnv_Ptr,
    sdata: WwRune_SData_Ptr,
}

impl WwRune_Repl {
    pub fn eval_snippet(&mut self, script: &str) -> rune::support::Result<()> {
        let mut diagnostics = Diagnostics::new();

        let mut sources = Sources::new();
        sources.insert(Source::new("snippet", script)?)?;

        let unit = rune::prepare(&mut sources)
            .with_context(&self.context)
            .with_diagnostics(&mut diagnostics)
            .build();

        let unit = Arc::new(unit.context(format!("{:#?}", diagnostics))?);

        let runtime = self.runtime.clone();
        let env = self.env.clone();
        let sdata = self.sdata.clone();

        XdomA::spawn_local(Box::pin(async move {
            let mut vm = Vm::new(runtime, unit);

            let t = match vm.execute(["main"], (sdata.clone(), env.clone())) {
                Ok(mut v) => v.async_complete().await,
                Err(err) => VmResult::Err(err),
            };

            let out = match t {
                VmResult::Err(e) => Err(format!("{:?}", e)),
                VmResult::Ok(v) => Ok(vm.with(|| {
                    format!("{:#?}", v)
                        .lines()
                        .map(|x| x.to_string())
                        .collect::<Vec<_>>()
                })),
            };

            let msg = Cmsg_Inner::H_gfx(Cmsg_HGfx::Repl_Rune(
                Cmsg_Repl_Rune::Output(out),
            ));
            G_CmsgQ::send_oneshot(msg);
        }));

        Ok(())
    }

    pub fn new() -> rune::support::Result<WwRune_Repl> {
        let mut context = rune::Context::with_default_modules()?;
        context.install(WwRune_ReplEnv_Ptr::setup_module()?)?;
        context.install(WwRune_SData_Ptr::setup_module()?)?;
        let runtime = Arc::new(context.runtime()?);
        let env = WwRune_ReplEnv_Ptr::default();
        let sdata = WwRune_SData_Ptr::default();
        Ok(WwRune_Repl {
            context,
            runtime,
            env,
            sdata,
        })
    }
}
