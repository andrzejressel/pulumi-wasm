#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct PipeEnrichmentParameters {
    /// Contains the HTTP parameters to use when the target is a API Gateway REST endpoint or EventBridge ApiDestination. If you specify an API Gateway REST API or EventBridge ApiDestination as a target, you can use this parameter to specify headers, path parameters, and query string keys/values as part of your target invoking request. If you're using ApiDestinations, the corresponding Connection can also have these values configured. In case of any conflicting keys, values from the Connection take precedence. Detailed below.
    #[builder(into, default)]
    #[serde(rename = "httpParameters")]
    pub r#http_parameters: Box<Option<super::super::types::pipes::PipeEnrichmentParametersHttpParameters>>,
    /// Valid JSON text passed to the target. In this case, nothing from the event itself is passed to the target. Maximum length of 8192 characters.
    #[builder(into, default)]
    #[serde(rename = "inputTemplate")]
    pub r#input_template: Box<Option<String>>,
}
