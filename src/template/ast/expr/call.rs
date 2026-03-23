use crate::reflect;
use crate::template::Scope;
use crate::template::ast::{EvalError, NotCallableError, Result, Span};

use super::Expr;

#[derive(Debug, Clone, PartialEq)]
pub struct CallExpr {
    pub callee: Box<Expr>,
    pub args: Vec<Expr>,
    pub span: Span,
}

impl CallExpr {
    pub fn eval(&self, scope: &Scope) -> Result<reflect::Value> {
        let name = match &*self.callee {
            Expr::Ident(ident) => ident.name.as_str(),
            _ => {
                return Err(EvalError::NotCallable(NotCallableError).with_span(self.span.clone()));
            }
        };

        let func = scope
            .func(name)
            .ok_or_else(|| EvalError::NotCallable(NotCallableError).with_span(self.span.clone()))?;

        let evaluated_args: Vec<reflect::Value> = self
            .args
            .iter()
            .map(|a| a.eval(scope))
            .collect::<Result<_>>()?;

        func.invoke(&evaluated_args)
    }
}

impl std::fmt::Display for CallExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.span)
    }
}
