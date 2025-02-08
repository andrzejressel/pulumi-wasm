#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct AiEndpointDeployedModelPrivateEndpoint {
    /// (Output)
    /// Output only. Http(s) path to send explain requests.
    #[builder(into, default)]
    #[serde(rename = "explainHttpUri")]
    pub r#explain_http_uri: Box<Option<String>>,
    /// (Output)
    /// Output only. Http(s) path to send health check requests.
    #[builder(into, default)]
    #[serde(rename = "healthHttpUri")]
    pub r#health_http_uri: Box<Option<String>>,
    /// (Output)
    /// Output only. Http(s) path to send prediction requests.
    #[builder(into, default)]
    #[serde(rename = "predictHttpUri")]
    pub r#predict_http_uri: Box<Option<String>>,
    /// (Output)
    /// Output only. The name of the service attachment resource. Populated if private service connect is enabled.
    #[builder(into, default)]
    #[serde(rename = "serviceAttachment")]
    pub r#service_attachment: Box<Option<String>>,
}
