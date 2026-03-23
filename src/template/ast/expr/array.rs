use crate::reflect;
use crate::template::Scope;
use crate::template::ast::{Result, Span};

use super::Expr;

#[derive(Debug, Clone, PartialEq)]
pub struct ArrayExpr {
    pub elements: Vec<Expr>,
    pub span: Span,
}

impl ArrayExpr {
    pub fn eval(&self, scope: &Scope) -> Result<reflect::Value> {
        let values: Vec<reflect::Value> = self
            .elements
            .iter()
            .map(|e| e.eval(scope))
            .collect::<Result<_>>()?;

        Ok(crate::valueof!((values)))
    }
}

impl std::fmt::Display for ArrayExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.span)
    }
}
