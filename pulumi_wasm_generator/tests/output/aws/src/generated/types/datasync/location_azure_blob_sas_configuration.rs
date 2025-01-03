#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct LocationAzureBlobSasConfiguration {
    /// A SAS token that provides permissions to access your Azure Blob Storage.
    #[builder(into)]
    #[serde(rename = "token")]
    pub r#token: Box<String>,
}
