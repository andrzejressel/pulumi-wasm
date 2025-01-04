#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetClusterGateway {
    /// Is the Ambari Portal enabled?
    #[builder(into)]
    #[serde(rename = "enabled")]
    pub r#enabled: Box<bool>,
    /// The password used for the Ambari Portal.
    #[builder(into)]
    #[serde(rename = "password")]
    pub r#password: Box<String>,
    /// The username used for the Ambari Portal.
    #[builder(into)]
    #[serde(rename = "username")]
    pub r#username: Box<String>,
}
