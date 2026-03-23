use super::{Context, Rule, ValidError, Validator};

#[repr(transparent)]
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[serde(transparent)]
pub struct Required(bool);

impl Required {
    pub const KEY: &str = "required";
    pub const PHASE: super::Phase = super::Phase::Presence;

    pub fn new(is_required: bool) -> Self {
        Self(is_required)
    }
}

impl From<Required> for Rule {
    fn from(value: Required) -> Self {
        Self::Required(value)
    }
}

impl Validator for Required {
    fn validate(&self, ctx: &Context) -> Result<crate::Value, ValidError> {
        if self.0 && ctx.value.is_null() {
            return Err(ctx.error("required"));
        }

        Ok(ctx.value.clone())
    }
}
