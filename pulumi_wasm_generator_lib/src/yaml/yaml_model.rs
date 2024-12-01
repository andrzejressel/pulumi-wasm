use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub(crate) struct YamlFile {
    pub(crate) resources: BTreeMap<String, YamlResource>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub(crate) struct YamlResource {
    #[serde(rename = "type")]
    pub(crate) type_: String,
    pub(crate) name: Option<String>,
    #[serde(default)]
    pub(crate) properties: BTreeMap<String, YamlExpression>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub(crate) enum YamlExpression {
    String(String),
    Number(f64),
    Boolean(bool),
    Object(BTreeMap<String, YamlExpression>),
    Array(Vec<YamlExpression>),
}

impl YamlFile {
    pub fn from_yaml(yaml: &str) -> Result<Self, serde_yaml::Error> {
        serde_yaml::from_str(yaml)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_array_yaml_deserialization() {
        use super::super::tests::example_array::*;
        let yaml_file = YamlFile::from_yaml(YAML).unwrap();
        let expected_yaml_file = get_yaml_file();
        assert_eq!(yaml_file, expected_yaml_file);
    }

    #[test]
    fn test_complex_yaml_deserialization() {
        use super::super::tests::complex_yaml::*;
        let yaml_file = YamlFile::from_yaml(YAML).unwrap();
        let expected_yaml_file = get_yaml_file();
        assert_eq!(yaml_file, expected_yaml_file);
    }

    #[test]
    fn test_access_rule_yaml_deserialization() {
        use super::super::tests::access_rule::*;
        let yaml_file = YamlFile::from_yaml(YAML).unwrap();
        let expected_yaml_file = get_yaml_file();
        assert_eq!(yaml_file, expected_yaml_file);
    }

    #[test]
    fn test_access_organization_yaml_deserialization() {
        use super::super::tests::example_access_organization::*;
        let yaml_file = YamlFile::from_yaml(YAML).unwrap();
        let expected_yaml_file = get_yaml_file();
        assert_eq!(yaml_file, expected_yaml_file);
    }

    #[test]
    fn test_example_variable() {
        use super::super::tests::example_variable::*;
        let yaml_file = YamlFile::from_yaml(YAML).unwrap();
        let expected_yaml_file = get_yaml_file();
        assert_eq!(yaml_file, expected_yaml_file);
    }

    #[test]
    fn test_without_parameters() {
        use super::super::tests::example_empty_properties::*;
        let yaml_file = YamlFile::from_yaml(YAML).unwrap();
        let expected_yaml_file = get_yaml_file();
        assert_eq!(yaml_file, expected_yaml_file);
    }

    #[test]
    fn test_without_numbers() {
        use super::super::tests::example_numbers::*;
        let yaml_file = YamlFile::from_yaml(YAML).unwrap();
        let expected_yaml_file = get_yaml_file();
        assert_eq!(yaml_file, expected_yaml_file);
    }
}
