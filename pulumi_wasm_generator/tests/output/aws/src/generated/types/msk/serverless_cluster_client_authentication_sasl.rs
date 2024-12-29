#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ServerlessClusterClientAuthenticationSasl {
    /// Details for client authentication using IAM. See below.
    #[builder(into)]
    #[serde(rename = "iam")]
    pub r#iam: Box<super::super::types::msk::ServerlessClusterClientAuthenticationSaslIam>,
}
