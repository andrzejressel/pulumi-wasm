//! Options for tuning the Kubernetes client used by a Provider.

#[derive(serde::Deserialize, serde::Serialize, bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
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
    pub r#rec_test: Box<Option<crate::types::KubeClientSettings>>,
}
