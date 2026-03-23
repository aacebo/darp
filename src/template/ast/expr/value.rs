use crate::reflect;
use crate::template::Scope;
use crate::template::ast::{Result, Span};

#[derive(Debug, Clone, PartialEq)]
pub struct ValueExpr {
    pub value: reflect::Value,
    pub span: Span,
}

impl ValueExpr {
    pub fn eval(&self, _scope: &Scope) -> Result<reflect::Value> {
        Ok(self.value.clone())
    }
}

impl std::fmt::Display for ValueExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.span)
    }
}
