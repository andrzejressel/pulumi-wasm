#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ApiContact {
    /// The email address of the contact person/organization.
    #[builder(into, default)]
    #[serde(rename = "email")]
    pub r#email: Box<Option<String>>,
    /// The name of the contact person/organization.
    #[builder(into, default)]
    #[serde(rename = "name")]
    pub r#name: Box<Option<String>>,
    /// Absolute URL of the contact information.
    #[builder(into, default)]
    #[serde(rename = "url")]
    pub r#url: Box<Option<String>>,
}