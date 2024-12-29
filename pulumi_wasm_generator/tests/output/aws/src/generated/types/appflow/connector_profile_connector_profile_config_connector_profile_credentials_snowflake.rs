#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ConnectorProfileConnectorProfileConfigConnectorProfileCredentialsSnowflake {
    #[builder(into)]
    #[serde(rename = "password")]
    pub r#password: Box<String>,
    #[builder(into)]
    #[serde(rename = "username")]
    pub r#username: Box<String>,
}
