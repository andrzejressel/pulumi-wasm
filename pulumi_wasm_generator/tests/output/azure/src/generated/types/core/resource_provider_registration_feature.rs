#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ResourceProviderRegistrationFeature {
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// Should this feature be Registered or Unregistered?
    #[builder(into)]
    #[serde(rename = "registered")]
    pub r#registered: Box<bool>,
}