use std::collections::BTreeMap;

use crate::Schema;

use super::{Context, Rule, ValidError, Validator};

#[repr(transparent)]
#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct Fields(BTreeMap<String, Schema>);

impl Fields {
    pub const KEY: &str = "fields";
    pub const PHASE: super::Phase = super::Phase::Constraint;

    pub fn new(fields: BTreeMap<String, Schema>) -> Self {
        Self(fields)
    }

    pub fn get(&self, name: &str) -> Option<&Schema> {
        self.0.get(name)
    }

    pub fn set(&mut self, name: &str, schema: Schema) -> &mut Self {
        self.0.insert(name.to_string(), schema);
        self
    }

    pub fn field<T: crate::ToSchema>(mut self, name: &str, value: T) -> Self {
        self.0.insert(name.to_string(), value.to_schema());
        self
    }
}

impl From<BTreeMap<String, Schema>> for Fields {
    fn from(value: BTreeMap<String, Schema>) -> Self {
        Self(value)
    }
}

impl From<Fields> for Rule {
    fn from(value: Fields) -> Self {
        Self::Fields(value)
    }
}

impl Validator for Fields {
    fn validate(&self, ctx: &Context) -> Result<crate::Value, ValidError> {
        if !ctx.value.is_null() && ctx.value.is_struct() {
            let input = ctx.value.as_struct();
            let mut error = ValidError::new(ctx.path.clone()).build();

            // Check for unexpected fields in the input
            for (ident, _) in input.items() {
                if !self.0.contains_key(&ident.to_string()) {
                    let path = ctx
                        .path
                        .child(crate::path::Ident::parse(&ident.to_string()));
                    error.errors.push(
                        ValidError::new(path)
                            .message(&format!("unexpected field '{}'", &ident))
                            .build(),
                    );
                }
            }

            // Validator all schema-defined fields
            for (name, schema) in &self.0 {
                let mut next = ctx.clone();
                next.path = ctx.path.child(crate::path::Ident::parse(name));
                next.value = input
                    .field(crate::path::Ident::key(name))
                    .map(|v| v.to_value())
                    .unwrap_or(crate::valueof!(null));

                if let Err(err) = schema.validate(&next) {
                    error.errors.push(err);
                }
            }

            if !error.is_empty() {
                return Err(error);
            }
        }

        Ok(ctx.value.clone())
    }
}
