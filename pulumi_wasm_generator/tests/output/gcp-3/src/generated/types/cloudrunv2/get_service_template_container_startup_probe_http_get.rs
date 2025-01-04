#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetServiceTemplateContainerStartupProbeHttpGet {
    /// Custom headers to set in the request. HTTP allows repeated headers.
    #[builder(into)]
    #[serde(rename = "httpHeaders")]
    pub r#http_headers: Box<Vec<super::super::types::cloudrunv2::GetServiceTemplateContainerStartupProbeHttpGetHttpHeader>>,
    /// Path to access on the HTTP server. Defaults to '/'.
    #[builder(into)]
    #[serde(rename = "path")]
    pub r#path: Box<String>,
    /// Port number to access on the container. Must be in the range 1 to 65535.
    /// If not specified, defaults to the same value as container.ports[0].containerPort.
    #[builder(into)]
    #[serde(rename = "port")]
    pub r#port: Box<i32>,
}
