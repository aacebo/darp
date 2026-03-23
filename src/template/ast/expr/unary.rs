use crate::reflect;
use crate::template::Scope;
use crate::template::ast::{Result, Span, UnaryOp, expect_number, is_truthy};

use super::Expr;

#[derive(Debug, Clone, PartialEq)]
pub struct UnaryExpr {
    pub op: UnaryOp,
    pub operand: Box<Expr>,
    pub span: Span,
}

impl UnaryExpr {
    pub fn eval(&self, scope: &Scope) -> Result<reflect::Value> {
        let val = self.operand.eval(scope)?;
        match self.op {
            UnaryOp::Not => Ok(crate::valueof!((!is_truthy(&val)))),
            UnaryOp::Neg => {
                let n = expect_number(&val, self.span.clone())?;
                if n.is_float() {
                    Ok(crate::valueof!((-n.to_f64())))
                } else {
                    Ok(crate::valueof!((n.to_i64().wrapping_neg())))
                }
            }
        }
    }
}

impl std::fmt::Display for UnaryExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.span)
    }
}
