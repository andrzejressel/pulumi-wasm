#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct IntegrationRuntimeSsisExpressCustomSetupCommandKeyKeyVaultPassword {
    #[builder(into)]
    #[serde(rename = "linkedServiceName")]
    pub r#linked_service_name: Box<String>,
    /// A map of parameters to associate with the Key Vault Data Factory Linked Service.
    #[builder(into, default)]
    #[serde(rename = "parameters")]
    pub r#parameters: Box<Option<std::collections::HashMap<String, String>>>,
    /// Specifies the secret name in Azure Key Vault.
    #[builder(into)]
    #[serde(rename = "secretName")]
    pub r#secret_name: Box<String>,
    /// Specifies the secret version in Azure Key Vault.
    #[builder(into, default)]
    #[serde(rename = "secretVersion")]
    pub r#secret_version: Box<Option<String>>,
}
