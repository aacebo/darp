use super::{
    Context, Items, Max, Min, Phase, Required, RuleSet, Schema, ToSchema, ValidError, Validator,
};
use crate::reflect;

pub fn array() -> ArraySchema {
    ArraySchema::default()
}

#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct ArraySchema(pub(crate) RuleSet);

impl ArraySchema {
    pub fn required(mut self) -> Self {
        self.0 = self.0.add(Required::new(true).into());
        self
    }

    pub fn min(mut self, min: usize) -> Self {
        self.0 = self
            .0
            .add(Min::from(reflect::Number::from_usize(min)).into());
        self
    }

    pub fn max(mut self, max: usize) -> Self {
        self.0 = self
            .0
            .add(Max::from(reflect::Number::from_usize(max)).into());
        self
    }

    pub fn items<T: ToSchema>(mut self, items: T) -> Self {
        self.0 = self.0.add(Items::from(items.to_schema()).into());
        self
    }
}

impl ToSchema for ArraySchema {
    fn to_schema(&self) -> Schema {
        Schema::Array(self.clone())
    }
}

impl From<ArraySchema> for Schema {
    fn from(value: ArraySchema) -> Self {
        Self::Array(value)
    }
}

impl Validator for ArraySchema {
    fn validate(&self, ctx: &Context) -> Result<crate::Value, ValidError> {
        let value = self.0.validate_phase(ctx, Phase::Presence)?;

        if !value.is_null() && !value.is_array() {
            return Err(ctx.error("expected array"));
        }

        let mut next = ctx.clone();
        next.value = value;
        self.0.validate_phase(&next, Phase::Constraint)
    }
}
