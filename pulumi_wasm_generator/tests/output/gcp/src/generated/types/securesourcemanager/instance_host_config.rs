#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct InstanceHostConfig {
    /// (Output)
    /// API hostname.
    #[builder(into, default)]
    #[serde(rename = "api")]
    pub r#api: Box<Option<String>>,
    /// (Output)
    /// Git HTTP hostname.
    #[builder(into, default)]
    #[serde(rename = "gitHttp")]
    pub r#git_http: Box<Option<String>>,
    /// (Output)
    /// Git SSH hostname.
    #[builder(into, default)]
    #[serde(rename = "gitSsh")]
    pub r#git_ssh: Box<Option<String>>,
    /// (Output)
    /// HTML hostname.
    #[builder(into, default)]
    #[serde(rename = "html")]
    pub r#html: Box<Option<String>>,
}
