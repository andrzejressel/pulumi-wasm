#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct BareMetalAdminClusterValidationCheckStatusResult {
    /// (Output)
    /// The category of the validation.
    #[builder(into, default)]
    #[serde(rename = "category")]
    pub r#category: Box<Option<String>>,
    /// A human readable description of this Bare Metal Admin Cluster.
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
    /// A human-readable message of the check failure.
    #[builder(into, default)]
    #[serde(rename = "reason")]
    pub r#reason: Box<Option<String>>,
}
