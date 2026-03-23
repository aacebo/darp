use super::{Context, Rule, ValidError, Validator};
use crate::reflect;

#[repr(transparent)]
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[serde(transparent)]
pub struct Max(reflect::Number);

impl Max {
    pub const KEY: &str = "max";
    pub const PHASE: super::Phase = super::Phase::Constraint;

    pub fn new(value: reflect::Number) -> Self {
        Self(value)
    }
}

impl From<reflect::Number> for Max {
    fn from(value: reflect::Number) -> Self {
        Self(value)
    }
}

impl From<Max> for Rule {
    fn from(value: Max) -> Self {
        Self::Max(value)
    }
}

impl Validator for Max {
    fn validate(&self, ctx: &Context) -> Result<crate::Value, ValidError> {
        if (ctx.value.is_array() || ctx.value.is_string()) && ctx.value.len() > self.0.to_usize() {
            return Err(ctx.error(&format!(
                "length must be at most {}, got {}",
                self.0.to_usize(),
                ctx.value.len()
            )));
        }

        if ctx.value.is_number() && ctx.value.as_number() > &self.0 {
            return Err(ctx.error(&format!(
                "value must be at most {}, got {}",
                &self.0,
                ctx.value.as_number()
            )));
        }

        Ok(ctx.value.clone())
    }
}
