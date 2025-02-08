#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct KubeClientSettings {
    /// Maximum burst for throttle. Default value is 10.
    #[builder(into, default)]
    #[serde(rename = "burst")]
    pub r#burst: Box<Option<i32>>,
    /// Maximum queries per second (QPS) to the API server from this client. Default value is 5.
    #[builder(into, default)]
    #[serde(rename = "qps")]
    pub r#qps: Box<Option<f64>>,
    #[builder(into, default)]
    #[serde(rename = "recTest")]
    pub r#rec_test: Box<Option<super::types::KubeClientSettings>>,
}
