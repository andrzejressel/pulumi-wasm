#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct CxWebhookServiceDirectory {
    /// The name of Service Directory service.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "genericWebService")]
    pub r#generic_web_service: Box<super::super::types::diagflow::CxWebhookServiceDirectoryGenericWebService>,
    /// The name of Service Directory service.
    #[builder(into)]
    #[serde(rename = "service")]
    pub r#service: Box<String>,
}
