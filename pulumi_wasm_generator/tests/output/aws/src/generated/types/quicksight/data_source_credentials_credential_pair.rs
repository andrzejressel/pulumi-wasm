#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct DataSourceCredentialsCredentialPair {
    /// Password, maximum length of 1024 characters.
    #[builder(into)]
    #[serde(rename = "password")]
    pub r#password: Box<String>,
    /// User name, maximum length of 64 characters.
    #[builder(into)]
    #[serde(rename = "username")]
    pub r#username: Box<String>,
}
