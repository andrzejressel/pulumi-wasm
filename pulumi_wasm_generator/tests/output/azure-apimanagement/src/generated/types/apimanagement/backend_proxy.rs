#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct BackendProxy {
    /// The password to connect to the proxy server.
    #[builder(into, default)]
    #[serde(rename = "password")]
    pub r#password: Box<Option<String>>,
    /// The URL of the proxy server.
    #[builder(into)]
    #[serde(rename = "url")]
    pub r#url: Box<String>,
    /// The username to connect to the proxy server.
    #[builder(into)]
    #[serde(rename = "username")]
    pub r#username: Box<String>,
}