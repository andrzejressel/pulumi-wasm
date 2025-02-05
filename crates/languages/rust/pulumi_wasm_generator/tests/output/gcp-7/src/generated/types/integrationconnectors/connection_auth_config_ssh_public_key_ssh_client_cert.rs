#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ConnectionAuthConfigSshPublicKeySshClientCert {
    /// The resource name of the secret version in the format,
    /// format as: projects/*/secrets/*/versions/*.
    #[builder(into)]
    #[serde(rename = "secretVersion")]
    pub r#secret_version: Box<String>,
}
