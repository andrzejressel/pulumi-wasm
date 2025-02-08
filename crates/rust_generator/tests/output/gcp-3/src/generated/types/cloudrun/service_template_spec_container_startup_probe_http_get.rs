#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ServiceTemplateSpecContainerStartupProbeHttpGet {
    /// Custom headers to set in the request. HTTP allows repeated headers.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "httpHeaders")]
    pub r#http_headers: Box<Option<Vec<super::super::types::cloudrun::ServiceTemplateSpecContainerStartupProbeHttpGetHttpHeader>>>,
    /// Path to access on the HTTP server. If set, it should not be empty string.
    #[builder(into, default)]
    #[serde(rename = "path")]
    pub r#path: Box<Option<String>>,
    /// Port number to access on the container. Number must be in the range 1 to 65535.
    /// If not specified, defaults to the same value as container.ports[0].containerPort.
    #[builder(into, default)]
    #[serde(rename = "port")]
    pub r#port: Box<Option<i32>>,
}
