use crate::{Schema, value::ToValue};

use super::{Context, Rule, ValidError, Validator};

#[repr(transparent)]
#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct Items(Schema);

impl Items {
    pub const KEY: &str = "items";
    pub const PHASE: super::Phase = super::Phase::Constraint;

    pub fn new(items: Schema) -> Self {
        Self(items)
    }
}

impl From<Schema> for Items {
    fn from(value: Schema) -> Self {
        Self(value)
    }
}

impl From<Items> for Rule {
    fn from(value: Items) -> Self {
        Self::Items(value)
    }
}

impl Validator for Items {
    fn validate(&self, ctx: &Context) -> Result<crate::Value, ValidError> {
        if !ctx.value.is_null() && ctx.value.is_array() {
            let mut items = vec![];
            let mut error = ValidError::new(ctx.path.clone()).build();

            for (i, item) in ctx.value.as_array().items().enumerate() {
                let mut next = ctx.clone();
                next.path = ctx.path.child(i);
                next.value = item.to_value();

                match self.0.validate(&next) {
                    Ok(v) => items.push(v),
                    Err(err) => {
                        items.push(next.value);
                        error.errors.push(err);
                    }
                }
            }

            if !error.errors.is_empty() {
                return Err(error);
            }

            return Ok(items.to_value());
        }

        Ok(ctx.value.clone())
    }
}
