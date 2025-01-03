#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ConnectionAuthConfigUserPassword {
    /// Password for Authentication.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "password")]
    pub r#password: Box<Option<super::super::types::integrationconnectors::ConnectionAuthConfigUserPasswordPassword>>,
    /// Username for Authentication.
    #[builder(into)]
    #[serde(rename = "username")]
    pub r#username: Box<String>,
}
