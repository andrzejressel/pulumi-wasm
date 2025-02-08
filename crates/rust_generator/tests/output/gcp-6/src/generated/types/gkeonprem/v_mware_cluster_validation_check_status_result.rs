#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct VMwareClusterValidationCheckStatusResult {
    /// (Output)
    /// The category of the validation.
    #[builder(into, default)]
    #[serde(rename = "category")]
    pub r#category: Box<Option<String>>,
    /// (Output)
    /// The description of the validation check.
    #[builder(into, default)]
    #[serde(rename = "description")]
    pub r#description: Box<Option<String>>,
    /// (Output)
    /// Detailed failure information, which might be unformatted.
    #[builder(into, default)]
    #[serde(rename = "details")]
    pub r#details: Box<Option<String>>,
    /// (Output)
    /// Options used for the validation check.
    #[builder(into, default)]
    #[serde(rename = "options")]
    pub r#options: Box<Option<String>>,
    /// (Output)
    /// Machine-readable message indicating details about last transition.
    #[builder(into, default)]
    #[serde(rename = "reason")]
    pub r#reason: Box<Option<String>>,
}
