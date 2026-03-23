use super::{Context, Rule, ValidError, Validator};

#[repr(transparent)]
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[serde(transparent)]
pub struct Options(Vec<crate::Value>);

impl Options {
    pub const KEY: &str = "options";
    pub const PHASE: super::Phase = super::Phase::Constraint;

    pub fn new(options: Vec<crate::Value>) -> Self {
        Self(options)
    }
}

impl From<Vec<crate::Value>> for Options {
    fn from(value: Vec<crate::Value>) -> Self {
        Self(value)
    }
}

impl From<&[crate::Value]> for Options {
    fn from(value: &[crate::Value]) -> Self {
        Self(value.to_vec())
    }
}

impl From<Options> for Rule {
    fn from(value: Options) -> Self {
        Self::Options(value)
    }
}

impl Validator for Options {
    fn validate(&self, ctx: &Context) -> Result<crate::Value, ValidError> {
        if ctx.value.is_null() {
            return Ok(ctx.value.clone());
        }

        for option in &self.0 {
            if ctx.value == *option {
                return Ok(ctx.value.clone());
            }
        }

        let options = self
            .0
            .iter()
            .map(|v| v.to_string())
            .collect::<Vec<_>>()
            .join(", ");

        Err(ctx.error(&format!("must be one of [{}]", options)))
    }
}
