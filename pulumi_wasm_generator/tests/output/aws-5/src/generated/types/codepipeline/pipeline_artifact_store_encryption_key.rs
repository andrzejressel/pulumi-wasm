#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct PipelineArtifactStoreEncryptionKey {
    /// The KMS key ARN or ID
    #[builder(into)]
    #[serde(rename = "id")]
    pub r#id: Box<String>,
    /// The type of key; currently only `KMS` is supported
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: Box<String>,
}
