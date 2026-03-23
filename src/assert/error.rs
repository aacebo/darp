use crate::path;
#[derive(Debug, Clone, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct ValidError {
    pub name: String,            // (the schema or rule name) "min", "string"
    pub path: path::Path,        // "test[1].name"
    pub message: Option<String>, // "length must be at least 1"
    pub errors: Vec<ValidError>,
}

impl ValidError {
    pub fn new(path: path::Path) -> ValidErrorBuilder {
        ValidErrorBuilder::new(path)
    }

    pub fn is_empty(&self) -> bool {
        self.errors.is_empty()
    }
}

impl std::fmt::Display for ValidError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Error[{}] @ {}", &self.name, &self.path)?;

        if let Some(message) = &self.message {
            writeln!(f, "  {}", &message)?;
        }

        for err in &self.errors {
            write!(f, "{}", err)?;
        }

        Ok(())
    }
}

impl std::error::Error for ValidError {}

#[derive(Debug, Clone, PartialEq)]
pub struct ValidErrorBuilder {
    name: String,
    path: path::Path,
    message: Option<String>,
    errors: Vec<ValidError>,
}

impl ValidErrorBuilder {
    pub fn new(path: path::Path) -> Self {
        Self {
            name: "unknown".to_string(),
            path,
            message: None,
            errors: vec![],
        }
    }

    pub fn name(mut self, name: &str) -> Self {
        self.name = name.to_string();
        self
    }

    pub fn message(mut self, message: &str) -> Self {
        self.message = Some(message.to_string());
        self
    }

    pub fn build(self) -> ValidError {
        ValidError {
            name: self.name,
            path: self.path,
            message: self.message,
            errors: self.errors,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::assert::*;
    use crate::path;

    mod display {
        use super::*;

        #[test]
        fn with_rule_and_message() {
            let err = ValidError::new(path::Path::parse("a/b").unwrap())
                .name("required")
                .message("field is required")
                .build();
            let output = err.to_string();
            assert!(output.contains("Error[required]"));
            assert!(output.contains("@ a/b"));
            assert!(output.contains("  field is required"));
        }

        #[test]
        fn without_rule() {
            let err = ValidError::new(path::Path::parse("a").unwrap())
                .message("something went wrong")
                .build();
            let output = err.to_string();
            assert!(output.contains("Error[unknown] @ a"));
            assert!(output.contains("  something went wrong"));
        }

        #[test]
        fn without_message() {
            let err = ValidError::new(path::Path::parse("x").unwrap())
                .name("equals")
                .build();
            let output = err.to_string();
            assert!(output.contains("Error[equals]"));
            assert!(output.contains("@ x"));
            assert_eq!(output.lines().count(), 1);
        }

        #[test]
        fn nested_errors() {
            let child1 = ValidError::new(path::Path::parse("a").unwrap())
                .name("required")
                .message("required")
                .build();
            let child2 = ValidError::new(path::Path::parse("a").unwrap())
                .name("equals")
                .message("not equal")
                .build();
            let mut parent = ValidError::new(path::Path::parse("a").unwrap()).build();
            parent.errors.push(child1);
            parent.errors.push(child2);

            let output = parent.to_string();
            assert!(output.contains("Error[required]"));
            assert!(output.contains("  required"));
            assert!(output.contains("Error[equals]"));
            assert!(output.contains("  not equal"));
        }
    }

    mod serde {
        use super::*;

        #[test]
        fn serialize() {
            let err = ValidError::new(path::Path::parse("a/b").unwrap())
                .name("required")
                .message("field is required")
                .build();
            let json = serde_json::to_string(&err).unwrap();
            let v: serde_json::Value = serde_json::from_str(&json).unwrap();
            assert_eq!(v["name"], "required");
            assert_eq!(v["path"], "a/b");
            assert_eq!(v["message"], "field is required");
        }

        #[test]
        fn roundtrip() {
            let err = ValidError::new(path::Path::parse("x/0").unwrap())
                .name("equals")
                .message("values differ")
                .build();
            let json = serde_json::to_string(&err).unwrap();
            let restored: ValidError = serde_json::from_str(&json).unwrap();
            assert_eq!(err, restored);
        }
    }
}
