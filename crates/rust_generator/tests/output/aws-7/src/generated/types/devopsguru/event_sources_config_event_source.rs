#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct EventSourcesConfigEventSource {
    /// Stores whether DevOps Guru is configured to consume recommendations which are generated from AWS CodeGuru Profiler. See `amazon_code_guru_profiler` below.
    #[builder(into, default)]
    #[serde(rename = "amazonCodeGuruProfilers")]
    pub r#amazon_code_guru_profilers: Box<Option<Vec<super::super::types::devopsguru::EventSourcesConfigEventSourceAmazonCodeGuruProfiler>>>,
}
