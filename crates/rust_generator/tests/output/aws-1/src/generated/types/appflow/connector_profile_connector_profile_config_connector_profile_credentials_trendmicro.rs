#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ConnectorProfileConnectorProfileConfigConnectorProfileCredentialsTrendmicro {
    #[builder(into)]
    #[serde(rename = "apiSecretKey")]
    pub r#api_secret_key: Box<String>,
}
