#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct SlotSiteConfigCors {
    /// A list of origins which should be able to make cross-origin calls. `*` can be used to allow all calls.
    #[builder(into)]
    #[serde(rename = "allowedOrigins")]
    pub r#allowed_origins: Box<Vec<String>>,
    /// Are credentials supported?
    #[builder(into, default)]
    #[serde(rename = "supportCredentials")]
    pub r#support_credentials: Box<Option<bool>>,
}