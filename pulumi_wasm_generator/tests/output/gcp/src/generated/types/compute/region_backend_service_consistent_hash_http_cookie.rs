#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct RegionBackendServiceConsistentHashHttpCookie {
    /// Name of the cookie.
    #[builder(into, default)]
    #[serde(rename = "name")]
    pub r#name: Box<Option<String>>,
    /// Path to set for the cookie.
    #[builder(into, default)]
    #[serde(rename = "path")]
    pub r#path: Box<Option<String>>,
    /// Lifetime of the cookie.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "ttl")]
    pub r#ttl: Box<Option<super::super::types::compute::RegionBackendServiceConsistentHashHttpCookieTtl>>,
}
