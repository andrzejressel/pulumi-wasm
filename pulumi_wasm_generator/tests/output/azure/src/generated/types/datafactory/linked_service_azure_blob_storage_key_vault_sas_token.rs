#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct LinkedServiceAzureBlobStorageKeyVaultSasToken {
    /// Specifies the name of an existing Key Vault Data Factory Linked Service.
    #[builder(into)]
    #[serde(rename = "linkedServiceName")]
    pub r#linked_service_name: Box<String>,
    /// Specifies the secret name in Azure Key Vault that stores the SAS token.
    #[builder(into)]
    #[serde(rename = "secretName")]
    pub r#secret_name: Box<String>,
}