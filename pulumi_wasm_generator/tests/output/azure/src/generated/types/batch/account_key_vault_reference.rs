#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct AccountKeyVaultReference {
    /// The Azure identifier of the Azure KeyVault to use.
    #[builder(into)]
    #[serde(rename = "id")]
    pub r#id: Box<String>,
    /// The HTTPS URL of the Azure KeyVault to use.
    #[builder(into)]
    #[serde(rename = "url")]
    pub r#url: Box<String>,
}