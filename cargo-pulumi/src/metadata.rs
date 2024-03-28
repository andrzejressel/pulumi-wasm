pub trait GetRelatedCrate {
    fn get_related_crate(&self) -> Option<String>;
}

impl GetRelatedCrate for cargo_metadata::Package {
    fn get_related_crate(&self) -> Option<String> {
        // Check if the metadata contains an object named "pulumi"
        if let Some(pulumi) = self.metadata.get("pulumi").and_then(|v| v.as_object()) {
            // If it does, check if "pulumi" contains an object named "related_crate"
            if let Some(related_crate) = pulumi.get("related_crate").and_then(|v| v.as_str()) {
                // If it does, return the string value of "related_crate"
                return Some(related_crate.to_string());
            }
        }
        // If either "pulumi" or "related_crate" do not exist, or if "related_crate" is not a string, return None
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use cargo_metadata::Package;
    use serde_json::json;

    #[test]
    fn get_related_crate_returns_none_when_no_metadata() {
        let package = create_package(json!({}));
        assert_eq!(package.get_related_crate(), None);
    }

    #[test]
    fn get_related_crate_returns_none_when_no_pulumi_in_metadata() {
        let package = create_package(json!({"not_pulumi": {}}));
        assert_eq!(package.get_related_crate(), None);
    }

    #[test]
    fn get_related_crate_returns_none_when_no_related_crate_in_pulumi() {
        let package = create_package(json!({"pulumi": {"not_related_crate": "value"}}));
        assert_eq!(package.get_related_crate(), None);
    }

    #[test]
    fn get_related_crate_returns_none_when_related_crate_is_not_string() {
        let package = create_package(json!({"pulumi": {"related_crate": 123}}));
        assert_eq!(package.get_related_crate(), None);
    }

    #[test]
    fn get_related_crate_returns_related_crate_when_present() {
        let package = create_package(json!({"pulumi": {"related_crate": "value"}}));
        assert_eq!(package.get_related_crate(), Some("value".to_string()));
    }

    fn create_package(metadata: serde_json::Value) -> Package {
        let package_json = json!({
            "name": "",
            "version": "0.0.0",
            "id": "id",
            "license": serde_json::Value::Null,
            "license_file": serde_json::Value::Null,
            "description": serde_json::Value::Null,
            "source": serde_json::Value::Null,
            "dependencies": [],
            "targets": [],
            "features": {},
            "manifest_path": "",
            "metadata": metadata,
            "links": serde_json::Value::Null,
            "authors": [],
            "categories": [],
            "keywords": [],
            "readme": serde_json::Value::Null,
            "repository": serde_json::Value::Null,
            "homepage": serde_json::Value::Null,
            "documentation": serde_json::Value::Null,
            "edition": "2021",
            "publish": serde_json::Value::Null,
            "default_run": serde_json::Value::Null,
            "rust_version": serde_json::Value::Null
        });
        serde_json::from_value(package_json).unwrap()
    }
}
