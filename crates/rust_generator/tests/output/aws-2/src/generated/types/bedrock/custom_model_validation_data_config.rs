#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct CustomModelValidationDataConfig {
    /// Information about the validators.
    #[builder(into, default)]
    #[serde(rename = "validators")]
    pub r#validators: Box<Option<Vec<super::super::types::bedrock::CustomModelValidationDataConfigValidator>>>,
}
