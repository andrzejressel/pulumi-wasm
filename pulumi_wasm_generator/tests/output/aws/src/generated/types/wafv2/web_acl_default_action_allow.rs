#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct WebAclDefaultActionAllow {
    /// Defines custom handling for the web request. See `custom_request_handling` below for details.
    #[builder(into, default)]
    #[serde(rename = "customRequestHandling")]
    pub r#custom_request_handling: Box<Option<super::super::types::wafv2::WebAclDefaultActionAllowCustomRequestHandling>>,
}
