#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetWindowsFunctionAppSiteConfigCor {
    /// A list of origins that should be allowed to make cross-origin calls.
    #[builder(into)]
    #[serde(rename = "allowedOrigins")]
    pub r#allowed_origins: Box<Vec<String>>,
    /// Are credentials allows in CORS requests?.
    #[builder(into)]
    #[serde(rename = "supportCredentials")]
    pub r#support_credentials: Box<bool>,
}