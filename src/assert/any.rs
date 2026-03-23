use super::{Context, Equals, Options, Required, RuleSet, Schema, ToSchema, ValidError, Validator};
use crate::reflect;

pub fn any() -> AnySchema {
    AnySchema::default()
}

#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct AnySchema(pub(crate) RuleSet);

impl AnySchema {
    pub fn equals<T: reflect::ToValue>(mut self, value: T) -> Self {
        self.0 = self.0.add(Equals::from(value.to_value()).into());
        self
    }

    pub fn options(mut self, options: &[&dyn reflect::ToValue]) -> Self {
        self.0 = self
            .0
            .add(Options::from(options.iter().map(|v| v.to_value()).collect::<Vec<_>>()).into());
        self
    }

    pub fn required(mut self) -> Self {
        self.0 = self.0.add(Required::new(true).into());
        self
    }
}

impl ToSchema for AnySchema {
    fn to_schema(&self) -> Schema {
        Schema::Any(self.clone())
    }
}

impl From<AnySchema> for Schema {
    fn from(value: AnySchema) -> Self {
        Self::Any(value)
    }
}

impl Validator for AnySchema {
    fn validate(&self, ctx: &Context) -> Result<crate::Value, ValidError> {
        self.0.validate(ctx)
    }
}

#[cfg(test)]
mod tests {
    use reflect::ToValue;

    use super::*;

    #[test]
    fn validate_any_value() {
        let schema = any();
        let result = schema.validate(&true.to_value().into());
        assert!(result.is_ok());
    }

    #[test]
    fn validate_null() {
        let schema = any();
        let result = schema.validate(&crate::valueof!(null).into());
        assert!(result.is_ok());
    }

    #[test]
    fn validate_required_rejects_null() {
        let schema = any().required();
        let result = schema.validate(&crate::valueof!(null).into());
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().errors[0].message.as_deref(),
            Some("required")
        );
    }

    #[test]
    fn validate_required_accepts_value() {
        let schema = any().required();
        let result = schema.validate(&42i32.to_value().into());
        assert!(result.is_ok());
    }

    #[test]
    fn validate_equals_match() {
        let schema = any().equals("hello");
        let result = schema.validate(&"hello".to_value().into());
        assert!(result.is_ok());
    }

    #[test]
    fn validate_equals_mismatch() {
        let schema = any().equals("hello");
        let result = schema.validate(&"world".to_value().into());
        assert!(result.is_err());
    }

    #[test]
    fn validate_options_match() {
        let schema = any().options(&[&1i32, &"test", &true]);
        let result = schema.validate(&"test".to_value().into());
        assert!(result.is_ok());
    }

    #[test]
    fn validate_options_mismatch() {
        let schema = any().options(&[&1i32, &2i32]);
        let result = schema.validate(&3i32.to_value().into());
        assert!(result.is_err());
    }

    #[test]
    fn validate_required_and_options() {
        let schema = any().required().options(&[&true, &false]);

        assert!(schema.validate(&true.to_value().into()).is_ok());
        assert!(schema.validate(&false.to_value().into()).is_ok());
        assert!(schema.validate(&crate::valueof!(null).into()).is_err());
        assert!(schema.validate(&42i32.to_value().into()).is_err());
    }

    #[test]
    fn validate_collects_multiple_errors() {
        let schema = any().required().equals(true);
        let err = schema.validate(&crate::valueof!(null).into()).unwrap_err();
        assert_eq!(err.errors.len(), 1);

        let err = schema.validate(&crate::valueof!(false).into()).unwrap_err();
        assert_eq!(err.errors.len(), 1);
    }
}
