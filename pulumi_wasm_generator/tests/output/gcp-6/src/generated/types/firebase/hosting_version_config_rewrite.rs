#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct HostingVersionConfigRewrite {
    /// The function to proxy requests to. Must match the exported function name exactly.
    #[builder(into, default)]
    #[serde(rename = "function")]
    pub r#function: Box<Option<String>>,
    /// The user-supplied glob to match against the request URL path.
    #[builder(into, default)]
    #[serde(rename = "glob")]
    pub r#glob: Box<Option<String>>,
    /// The URL path to rewrite the request to.
    #[builder(into, default)]
    #[serde(rename = "path")]
    pub r#path: Box<Option<String>>,
    /// The user-supplied RE2 regular expression to match against the request URL path.
    #[builder(into, default)]
    #[serde(rename = "regex")]
    pub r#regex: Box<Option<String>>,
    /// The request will be forwarded to Cloud Run.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "run")]
    pub r#run: Box<Option<super::super::types::firebase::HostingVersionConfigRewriteRun>>,
}
