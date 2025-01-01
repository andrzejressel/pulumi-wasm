#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct LinuxFunctionAppSiteConfigCors {
    /// Specifies a list of origins that should be allowed to make cross-origin calls.
    #[builder(into, default)]
    #[serde(rename = "allowedOrigins")]
    pub r#allowed_origins: Box<Option<Vec<String>>>,
    /// Are credentials allowed in CORS requests? Defaults to `false`.
    #[builder(into, default)]
    #[serde(rename = "supportCredentials")]
    pub r#support_credentials: Box<Option<bool>>,
}
