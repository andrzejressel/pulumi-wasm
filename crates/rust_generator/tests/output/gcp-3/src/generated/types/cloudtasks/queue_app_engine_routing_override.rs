#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct QueueAppEngineRoutingOverride {
    /// (Output)
    /// The host that the task is sent to.
    #[builder(into, default)]
    #[serde(rename = "host")]
    pub r#host: Box<Option<String>>,
    /// App instance.
    /// By default, the task is sent to an instance which is available when the task is attempted.
    #[builder(into, default)]
    #[serde(rename = "instance")]
    pub r#instance: Box<Option<String>>,
    /// App service.
    /// By default, the task is sent to the service which is the default service when the task is attempted.
    #[builder(into, default)]
    #[serde(rename = "service")]
    pub r#service: Box<Option<String>>,
    /// App version.
    /// By default, the task is sent to the version which is the default version when the task is attempted.
    #[builder(into, default)]
    #[serde(rename = "version")]
    pub r#version: Box<Option<String>>,
}
