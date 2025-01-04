#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct JobTemplateConfigEncryptionSecretManagerKeySource {
    /// The name of the Secret Version containing the encryption key in the following format: projects/{project}/secrets/{secret_id}/versions/{version_number}.
    #[builder(into)]
    #[serde(rename = "secretVersion")]
    pub r#secret_version: Box<String>,
}
