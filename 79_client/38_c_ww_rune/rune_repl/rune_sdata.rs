use super::*;

use rune::ToValue;

#[derive(rune::Any, Debug, Default, Clone)]
pub struct WwRune_SData_Ptr {}

impl WwRune_SData_Ptr {
    #[rune::function]
    pub fn hello(&self) -> Value {
        wlog!("WwRune_SData_Ptr :: hello called");
        "Hello World".to_value().unwrap()
    }

    pub fn setup_module() -> Result<Module, rune::ContextError> {
        let mut module = Module::new();
        module.ty::<WwRune_SData_Ptr>()?;
        module.function_meta(WwRune_SData_Ptr::hello)?;
        Ok(module)
    }
}
