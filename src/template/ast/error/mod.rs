mod division_by_zero;
mod include_depth;
mod index_out_of_bounds;
mod invalid_index;
mod not_callable;
mod not_iterable;
mod type_error;
mod undefined_field;
mod undefined_pipe;
mod undefined_template;
mod undefined_variable;

pub use division_by_zero::*;
pub use include_depth::*;
pub use index_out_of_bounds::*;
pub use invalid_index::*;
pub use not_callable::*;
pub use not_iterable::*;
pub use type_error::*;
pub use undefined_field::*;
pub use undefined_pipe::*;
pub use undefined_template::*;
pub use undefined_variable::*;

use super::Span;

pub type Result<T> = std::result::Result<T, EvalError>;

#[derive(Debug, Clone, PartialEq)]
pub struct SpanError {
    pub error: Box<EvalError>,
    pub span: Span,
}

impl std::fmt::Display for SpanError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "eval error at {}..{}: {}",
            self.span.start, self.span.end, self.error
        )
    }
}

impl std::error::Error for SpanError {}

#[derive(Debug, Clone, PartialEq)]
pub enum EvalError {
    UndefinedVariable(UndefinedVariableError),
    UndefinedPipe(UndefinedPipeError),
    UndefinedField(UndefinedFieldError),
    UndefinedTemplate(UndefinedTemplateError),
    IndexOutOfBounds(IndexOutOfBoundsError),
    TypeError(TypeError),
    DivisionByZero(DivisionByZeroError),
    NotCallable(NotCallableError),
    NotIterable(NotIterableError),
    InvalidIndex(InvalidIndexError),
    IncludeDepth(IncludeDepthError),
    Span(SpanError),
}

impl EvalError {
    pub fn span(&self) -> Option<&Span> {
        match self {
            Self::Span(e) => Some(&e.span),
            _ => None,
        }
    }

    pub fn with_span(self, span: Span) -> Self {
        Self::Span(SpanError {
            error: Box::new(self),
            span,
        })
    }

    pub fn inner(&self) -> &Self {
        match self {
            Self::Span(e) => e.error.inner(),
            other => other,
        }
    }
}

impl std::fmt::Display for EvalError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UndefinedVariable(e) => write!(f, "{e}"),
            Self::UndefinedPipe(e) => write!(f, "{e}"),
            Self::UndefinedField(e) => write!(f, "{e}"),
            Self::UndefinedTemplate(e) => write!(f, "{e}"),
            Self::IndexOutOfBounds(e) => write!(f, "{e}"),
            Self::TypeError(e) => write!(f, "{e}"),
            Self::DivisionByZero(e) => write!(f, "{e}"),
            Self::NotCallable(e) => write!(f, "{e}"),
            Self::NotIterable(e) => write!(f, "{e}"),
            Self::InvalidIndex(e) => write!(f, "{e}"),
            Self::IncludeDepth(e) => write!(f, "{e}"),
            Self::Span(e) => write!(f, "{e}"),
        }
    }
}

impl std::error::Error for EvalError {}

pub fn is_truthy(val: &crate::reflect::Value) -> bool {
    match val {
        crate::reflect::Value::Null => false,
        crate::reflect::Value::Bool(b) => b.to_bool(),
        crate::reflect::Value::Number(n) => n.to_f64() != 0.0,
        crate::reflect::Value::String(s) => !s.as_str().is_empty(),
        crate::reflect::Value::Object(o) => !o.is_empty(),
    }
}

pub fn expect_number(val: &crate::reflect::Value, span: Span) -> Result<&crate::reflect::Number> {
    match val {
        crate::reflect::Value::Number(n) => Ok(n),
        other => Err(EvalError::TypeError(TypeError {
            expected: "number",
            got: value_type_name(other),
        })
        .with_span(span)),
    }
}

pub fn value_to_usize(val: &crate::reflect::Value, span: Span) -> Result<usize> {
    let n = expect_number(val, span.clone())?;
    let v = n.to_i64();
    if v >= 0 {
        Ok(v as usize)
    } else {
        Err(EvalError::InvalidIndex(InvalidIndexError).with_span(span))
    }
}

pub fn value_type_name(val: &crate::reflect::Value) -> String {
    match val {
        crate::reflect::Value::Null => "null".to_string(),
        crate::reflect::Value::Bool(_) => "bool".to_string(),
        crate::reflect::Value::Number(_) => "number".to_string(),
        crate::reflect::Value::String(_) => "string".to_string(),
        crate::reflect::Value::Object(o) => o.name().to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn truthiness() {
        assert!(!is_truthy(&crate::valueof!(null)));
        assert!(!is_truthy(&crate::valueof!(false)));
        assert!(is_truthy(&crate::valueof!(true)));
        assert!(!is_truthy(&crate::valueof!(0_i64)));
        assert!(is_truthy(&crate::valueof!(1_i64)));
        assert!(!is_truthy(&crate::valueof!(0.0_f64)));
        assert!(is_truthy(&crate::valueof!(0.1_f64)));
        assert!(!is_truthy(&crate::valueof!("")));
        assert!(is_truthy(&crate::valueof!("x")));
    }
}
