#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct WorkloadAttributesDeveloperOwner {
    /// Contact's name.
    #[builder(into, default)]
    #[serde(rename = "displayName")]
    pub r#display_name: Box<Option<String>>,
    /// Email address of the contacts.
    #[builder(into)]
    #[serde(rename = "email")]
    pub r#email: Box<String>,
}
