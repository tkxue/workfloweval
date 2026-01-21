use super::*;

#[derive(rune::Any, Debug, Default, Clone)]
pub struct WwRune_ReplEnv_Ptr {
    variables: ArcState<HashMap<String, Value>>,
}

impl WwRune_ReplEnv_Ptr {
    pub fn new() -> Self {
        Self::default()
    }

    #[rune::function]
    pub fn set(&self, key: String, value: Value) {
        self.variables.update(move |x| x.insert(key, value));
    }

    #[rune::function]
    pub fn get(&self, key: String) -> Option<Value> {
        self.variables.read(move |x| x.get(&key).cloned())
    }

    pub fn setup_module() -> Result<Module, rune::ContextError> {
        let mut module = Module::new();
        module.ty::<WwRune_ReplEnv_Ptr>()?;
        module.function_meta(WwRune_ReplEnv_Ptr::set)?;
        module.function_meta(WwRune_ReplEnv_Ptr::get)?;
        Ok(module)
    }
}
