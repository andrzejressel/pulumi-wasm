#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct EventConnectionAuthParametersBasic {
    /// A password for the authorization. Created and stored in AWS Secrets Manager.
    #[builder(into)]
    #[serde(rename = "password")]
    pub r#password: Box<String>,
    /// A username for the authorization.
    #[builder(into)]
    #[serde(rename = "username")]
    pub r#username: Box<String>,
}