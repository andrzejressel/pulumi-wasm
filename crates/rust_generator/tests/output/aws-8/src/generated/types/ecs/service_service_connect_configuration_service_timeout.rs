#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ServiceServiceConnectConfigurationServiceTimeout {
    /// Amount of time in seconds a connection will stay active while idle. A value of 0 can be set to disable idleTimeout.
    #[builder(into, default)]
    #[serde(rename = "idleTimeoutSeconds")]
    pub r#idle_timeout_seconds: Box<Option<i32>>,
    /// Amount of time in seconds for the upstream to respond with a complete response per request. A value of 0 can be set to disable perRequestTimeout. Can only be set when appProtocol isn't TCP.
    #[builder(into, default)]
    #[serde(rename = "perRequestTimeoutSeconds")]
    pub r#per_request_timeout_seconds: Box<Option<i32>>,
}
