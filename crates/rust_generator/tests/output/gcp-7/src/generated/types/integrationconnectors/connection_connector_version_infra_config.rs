#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct ConnectionConnectorVersionInfraConfig {
    /// (Output)
    /// Max QPS supported by the connector version before throttling of requests.
    #[builder(into, default)]
    #[serde(rename = "ratelimitThreshold")]
    pub r#ratelimit_threshold: Box<Option<String>>,
}
