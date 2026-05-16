use dpg_model::MetadataModel;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Violation {
    pub code: &'static str,
    pub message: String,
}

#[must_use]
pub fn validate(model: &MetadataModel) -> Vec<Violation> {
    let mut violations = Vec::new();

    if model.dataset.name.trim().is_empty() {
        violations.push(Violation {
            code: "dataset.name.empty",
            message: "Dataset name must not be empty.".to_string(),
        });
    }

    if model.dataset.fields.is_empty() {
        violations.push(Violation {
            code: "dataset.fields.empty",
            message: "Dataset must contain at least one field.".to_string(),
        });
    }

    for field in &model.dataset.fields {
        if field.name.trim().is_empty() {
            violations.push(Violation {
                code: "field.name.empty",
                message: "Field name must not be empty.".to_string(),
            });
        }
    }

    violations
}

#[cfg(test)]
mod tests {
    use super::validate;
    use dpg_model::{Dataset, Field, MetadataModel};

    #[test]
    fn rejects_empty_dataset_name() {
        let model = MetadataModel::new("v1", Dataset::new("", vec![Field::new("id", "string", false)]));
        let violations = validate(&model);
        assert!(!violations.is_empty());
    }

    #[test]
    fn accepts_well_formed_model() {
        let model = MetadataModel::new(
            "v1",
            Dataset::new("sales_orders", vec![Field::new("order_id", "string", false)]),
        );

        let violations = validate(&model);
        assert!(violations.is_empty());
    }
}
