use super::{Context, Rule, ValidError, Validator};

#[repr(transparent)]
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[serde(transparent)]
pub struct Equals(crate::Value);

impl Equals {
    pub const KEY: &str = "equals";
    pub const PHASE: super::Phase = super::Phase::Constraint;

    pub fn new(value: crate::Value) -> Self {
        Self(value)
    }
}

impl From<crate::Value> for Equals {
    fn from(value: crate::Value) -> Self {
        Self(value)
    }
}

impl From<Equals> for Rule {
    fn from(value: Equals) -> Self {
        Self::Equals(value)
    }
}

impl Validator for Equals {
    fn validate(&self, ctx: &Context) -> Result<crate::Value, ValidError> {
        if ctx.value.is_null() {
            return Ok(ctx.value.clone());
        }

        if ctx.value != self.0 {
            return Err(ctx.error(&format!("{} is not equal to {}", &ctx.value, &self.0)));
        }

        Ok(ctx.value.clone())
    }
}
