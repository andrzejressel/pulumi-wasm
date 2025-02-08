#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct JobAppEngineHttpTargetAppEngineRouting {
    /// App instance.
    /// By default, the job is sent to an instance which is available when the job is attempted.
    #[builder(into, default)]
    #[serde(rename = "instance")]
    pub r#instance: Box<Option<String>>,
    /// App service.
    /// By default, the job is sent to the service which is the default service when the job is attempted.
    #[builder(into, default)]
    #[serde(rename = "service")]
    pub r#service: Box<Option<String>>,
    /// App version.
    /// By default, the job is sent to the version which is the default version when the job is attempted.
    #[builder(into, default)]
    #[serde(rename = "version")]
    pub r#version: Box<Option<String>>,
}
