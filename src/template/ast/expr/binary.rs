use crate::reflect;
use crate::template::Scope;
use crate::template::ast::{
    BinaryOp, DivisionByZeroError, EvalError, Result, Span, expect_number, is_truthy,
};

use super::Expr;

#[derive(Debug, Clone, PartialEq)]
pub struct BinaryExpr {
    pub left: Box<Expr>,
    pub op: BinaryOp,
    pub right: Box<Expr>,
    pub span: Span,
}

impl BinaryExpr {
    pub fn eval(&self, scope: &Scope) -> Result<reflect::Value> {
        // Short-circuit for logical ops.
        match self.op {
            BinaryOp::And => {
                let left_val = self.left.eval(scope)?;
                if !is_truthy(&left_val) {
                    return Ok(left_val);
                }
                return self.right.eval(scope);
            }
            BinaryOp::Or => {
                let left_val = self.left.eval(scope)?;
                if is_truthy(&left_val) {
                    return Ok(left_val);
                }
                return self.right.eval(scope);
            }
            _ => {}
        }

        let left_val = self.left.eval(scope)?;
        let right_val = self.right.eval(scope)?;

        match self.op {
            BinaryOp::Eq => Ok(crate::valueof!((left_val == right_val))),
            BinaryOp::Ne => Ok(crate::valueof!((left_val != right_val))),
            BinaryOp::Lt | BinaryOp::Le | BinaryOp::Gt | BinaryOp::Ge => {
                Self::eval_comparison(&left_val, self.op, &right_val)
            }
            BinaryOp::Add if left_val.is_string() || right_val.is_string() => {
                Ok(crate::valueof!((format!("{}{}", left_val, right_val))))
            }
            BinaryOp::Add | BinaryOp::Sub | BinaryOp::Mul | BinaryOp::Div | BinaryOp::Mod => {
                let l = expect_number(&left_val, self.span.clone())?;
                let r = expect_number(&right_val, self.span.clone())?;
                Self::eval_arithmetic(l, self.op, r, self.span.clone())
            }
            BinaryOp::And | BinaryOp::Or => unreachable!(),
        }
    }

    fn eval_comparison(
        left: &reflect::Value,
        op: BinaryOp,
        right: &reflect::Value,
    ) -> Result<reflect::Value> {
        if left.is_number() && right.is_number() {
            let lf = left.as_number().to_f64();
            let rf = right.as_number().to_f64();
            let result = match op {
                BinaryOp::Lt => lf < rf,
                BinaryOp::Le => lf <= rf,
                BinaryOp::Gt => lf > rf,
                BinaryOp::Ge => lf >= rf,
                _ => unreachable!(),
            };
            Ok(crate::valueof!((result)))
        } else {
            let cmp = left.partial_cmp(right);
            let result = matches!(
                (op, cmp),
                (BinaryOp::Lt, Some(std::cmp::Ordering::Less))
                    | (
                        BinaryOp::Le,
                        Some(std::cmp::Ordering::Less | std::cmp::Ordering::Equal)
                    )
                    | (BinaryOp::Gt, Some(std::cmp::Ordering::Greater))
                    | (
                        BinaryOp::Ge,
                        Some(std::cmp::Ordering::Greater | std::cmp::Ordering::Equal)
                    )
            );
            Ok(crate::valueof!((result)))
        }
    }

    fn eval_arithmetic(
        l: &reflect::Number,
        op: BinaryOp,
        r: &reflect::Number,
        span: Span,
    ) -> Result<reflect::Value> {
        if l.is_float() || r.is_float() {
            let lf = l.to_f64();
            let rf = r.to_f64();
            return match op {
                BinaryOp::Add => Ok(crate::valueof!((lf + rf))),
                BinaryOp::Sub => Ok(crate::valueof!((lf - rf))),
                BinaryOp::Mul => Ok(crate::valueof!((lf * rf))),
                BinaryOp::Div => {
                    if rf == 0.0 {
                        return Err(EvalError::DivisionByZero(DivisionByZeroError).with_span(span));
                    }
                    Ok(crate::valueof!((lf / rf)))
                }
                BinaryOp::Mod => {
                    if rf == 0.0 {
                        return Err(EvalError::DivisionByZero(DivisionByZeroError).with_span(span));
                    }
                    Ok(crate::valueof!((lf % rf)))
                }
                _ => unreachable!(),
            };
        }

        let li = l.to_i64();
        let ri = r.to_i64();

        match op {
            BinaryOp::Add => Ok(crate::valueof!((li.wrapping_add(ri)))),
            BinaryOp::Sub => Ok(crate::valueof!((li.wrapping_sub(ri)))),
            BinaryOp::Mul => Ok(crate::valueof!((li.wrapping_mul(ri)))),
            BinaryOp::Div => {
                if ri == 0 {
                    return Err(EvalError::DivisionByZero(DivisionByZeroError).with_span(span));
                }
                Ok(crate::valueof!((li.wrapping_div(ri))))
            }
            BinaryOp::Mod => {
                if ri == 0 {
                    return Err(EvalError::DivisionByZero(DivisionByZeroError).with_span(span));
                }
                Ok(crate::valueof!((li.wrapping_rem(ri))))
            }
            _ => unreachable!(),
        }
    }
}

impl std::fmt::Display for BinaryExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.span)
    }
}
