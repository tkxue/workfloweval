use super::value::Value;

#[derive(Debug, Clone)]
pub struct Formula {
    pub(crate) formula: String,
}

impl Formula {
    pub fn new(formula: String) -> Self {
        Self { formula }
    }
}

#[derive(Debug, Clone)]
pub enum FormulaResult {
    Pending,
    Value(Value),
    Error(String),
}
