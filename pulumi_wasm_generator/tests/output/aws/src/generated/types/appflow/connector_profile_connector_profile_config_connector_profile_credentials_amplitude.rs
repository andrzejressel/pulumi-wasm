#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ConnectorProfileConnectorProfileConfigConnectorProfileCredentialsAmplitude {
    #[builder(into)]
    #[serde(rename = "apiKey")]
    pub r#api_key: Box<String>,
    /// The Secret Access Key portion of the credentials.
    #[builder(into)]
    #[serde(rename = "secretKey")]
    pub r#secret_key: Box<String>,
}