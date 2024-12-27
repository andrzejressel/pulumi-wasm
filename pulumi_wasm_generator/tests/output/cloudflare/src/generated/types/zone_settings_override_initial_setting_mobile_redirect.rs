#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ZoneSettingsOverrideInitialSettingMobileRedirect {
    #[builder(into)]
    #[serde(rename = "mobileSubdomain")]
    pub r#mobile_subdomain: Box<String>,
    #[builder(into)]
    #[serde(rename = "status")]
    pub r#status: Box<String>,
    #[builder(into)]
    #[serde(rename = "stripUri")]
    pub r#strip_uri: Box<bool>,
}
