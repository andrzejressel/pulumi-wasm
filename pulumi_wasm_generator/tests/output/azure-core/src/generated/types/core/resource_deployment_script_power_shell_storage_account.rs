#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ResourceDeploymentScriptPowerShellStorageAccount {
    /// Specifies the storage account access key.
    #[builder(into)]
    #[serde(rename = "key")]
    pub r#key: Box<String>,
    /// Specifies the storage account name.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
}