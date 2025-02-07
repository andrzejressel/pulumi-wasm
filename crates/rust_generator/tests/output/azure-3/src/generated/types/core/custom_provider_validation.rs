#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct CustomProviderValidation {
    /// The endpoint where the validation specification is located.
    #[builder(into)]
    #[serde(rename = "specification")]
    pub r#specification: Box<String>,
}
