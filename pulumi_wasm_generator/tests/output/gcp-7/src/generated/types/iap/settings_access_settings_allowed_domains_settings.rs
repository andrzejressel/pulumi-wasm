#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct SettingsAccessSettingsAllowedDomainsSettings {
    /// List of trusted domains.
    #[builder(into, default)]
    #[serde(rename = "domains")]
    pub r#domains: Box<Option<Vec<String>>>,
    /// Configuration for customers to opt in for the feature.
    #[builder(into, default)]
    #[serde(rename = "enable")]
    pub r#enable: Box<Option<bool>>,
}
