#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct LinkedServiceOdbcBasicAuthentication {
    /// The password associated with the username, which can be used to authenticate to the ODBC endpoint.
    #[builder(into)]
    #[serde(rename = "password")]
    pub r#password: Box<String>,
    /// The username which can be used to authenticate to the ODBC endpoint.
    #[builder(into)]
    #[serde(rename = "username")]
    pub r#username: Box<String>,
}