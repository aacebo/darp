use crate::value::ToValue;

use super::{
    Context, Equals, Max, Min, NumberSchema, Options, Phase, Required, RuleSet, Schema, ToSchema,
    ValidError, Validator,
};

pub fn int() -> IntSchema {
    IntSchema::default()
}

#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct IntSchema(pub(crate) RuleSet);

impl IntSchema {
    pub fn equals(mut self, value: crate::value::Int) -> Self {
        self.0 = self.0.add(Equals::from(value.to_value()).into());
        self
    }

    pub fn options(mut self, options: &[crate::value::Int]) -> Self {
        self.0 = self
            .0
            .add(Options::from(options.iter().map(|v| v.to_value()).collect::<Vec<_>>()).into());
        self
    }

    pub fn required(mut self) -> Self {
        self.0 = self.0.add(Required::new(true).into());
        self
    }

    pub fn min(mut self, min: isize) -> Self {
        self.0 = self
            .0
            .add(Min::from(crate::value::Number::from_isize(min)).into());
        self
    }

    pub fn max(mut self, max: isize) -> Self {
        self.0 = self
            .0
            .add(Max::from(crate::value::Number::from_isize(max)).into());
        self
    }
}

impl ToSchema for IntSchema {
    fn to_schema(&self) -> Schema {
        Schema::Int(self.clone())
    }
}

impl From<IntSchema> for Schema {
    fn from(value: IntSchema) -> Self {
        Self::Int(value)
    }
}

impl From<NumberSchema> for IntSchema {
    fn from(value: NumberSchema) -> Self {
        Self(value.0)
    }
}

impl Validator for IntSchema {
    fn validate(&self, ctx: &Context) -> Result<crate::Value, ValidError> {
        let value = self.0.validate_phase(ctx, Phase::Presence)?;

        if !value.is_null() && !value.is_int() {
            return Err(ctx.error("expected integer"));
        }

        let mut next = ctx.clone();
        next.value = value;
        self.0.validate_phase(&next, Phase::Constraint)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validate_int() {
        let schema = int();
        assert!(schema.validate(&42i32.to_value().into()).is_ok());
    }

    #[test]
    fn validate_rejects_float() {
        let schema = int();
        let err = schema.validate(&3.14f64.to_value().into()).unwrap_err();
        assert_eq!(err.message.as_deref(), Some("expected integer"));
    }

    #[test]
    fn validate_rejects_string() {
        let schema = int();
        let err = schema.validate(&"hello".to_value().into()).unwrap_err();
        assert_eq!(err.message.as_deref(), Some("expected integer"));
    }

    #[test]
    fn validate_null_passes_without_required() {
        let schema = int();
        assert!(schema.validate(&crate::valueof!(null).into()).is_ok());
    }

    #[test]
    fn validate_required_rejects_null() {
        let schema = int().required();
        let err = schema.validate(&crate::valueof!(null).into()).unwrap_err();
        assert_eq!(err.errors[0].message.as_deref(), Some("required"));
    }

    #[test]
    fn validate_equals() {
        let schema = int().equals(crate::value::Int::from_i32(42));
        assert!(schema.validate(&42i32.to_value().into()).is_ok());
        assert!(schema.validate(&43i32.to_value().into()).is_err());
    }

    #[test]
    fn validate_options() {
        let schema = int().options(&[
            crate::value::Int::from_i32(1),
            crate::value::Int::from_i32(2),
            crate::value::Int::from_i32(3),
        ]);
        assert!(schema.validate(&2i32.to_value().into()).is_ok());
        assert!(schema.validate(&4i32.to_value().into()).is_err());
    }

    #[test]
    fn validate_required_and_equals() {
        let schema = int().required().equals(crate::value::Int::from_i32(10));
        assert!(schema.validate(&10i32.to_value().into()).is_ok());
        assert!(schema.validate(&11i32.to_value().into()).is_err());
        assert!(schema.validate(&crate::valueof!(null).into()).is_err());
    }

    #[test]
    fn validate_min() {
        let schema = int().min(5);
        assert!(schema.validate(&3i32.to_value().into()).is_err());
        assert!(schema.validate(&5i32.to_value().into()).is_ok());
        assert!(schema.validate(&10i32.to_value().into()).is_ok());
    }

    #[test]
    fn validate_max() {
        let schema = int().max(10);
        assert!(schema.validate(&5i32.to_value().into()).is_ok());
        assert!(schema.validate(&10i32.to_value().into()).is_ok());
        assert!(schema.validate(&15i32.to_value().into()).is_err());
    }

    #[test]
    fn validate_min_and_max() {
        let schema = int().min(1).max(10);
        assert!(schema.validate(&0i32.to_value().into()).is_err());
        assert!(schema.validate(&1i32.to_value().into()).is_ok());
        assert!(schema.validate(&5i32.to_value().into()).is_ok());
        assert!(schema.validate(&10i32.to_value().into()).is_ok());
        assert!(schema.validate(&11i32.to_value().into()).is_err());
    }

    #[test]
    fn validate_min_negative() {
        let schema = int().min(-5);
        assert!(schema.validate(&(-10i32).to_value().into()).is_err());
        assert!(schema.validate(&(-5i32).to_value().into()).is_ok());
        assert!(schema.validate(&0i32.to_value().into()).is_ok());
    }

    #[test]
    fn from_number_schema() {
        let schema: IntSchema = NumberSchema::default().into();
        assert!(schema.validate(&42i32.to_value().into()).is_ok());
    }
}
