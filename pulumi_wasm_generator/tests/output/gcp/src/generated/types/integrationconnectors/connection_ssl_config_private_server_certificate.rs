#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ConnectionSslConfigPrivateServerCertificate {
    /// Secret version of Secret Value for Config variable.
    #[builder(into)]
    #[serde(rename = "secretVersion")]
    pub r#secret_version: Box<String>,
}
