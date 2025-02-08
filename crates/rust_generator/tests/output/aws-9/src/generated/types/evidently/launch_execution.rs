#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct LaunchExecution {
    /// The date and time that the launch ended.
    #[builder(into, default)]
    #[serde(rename = "endedTime")]
    pub r#ended_time: Box<Option<String>>,
    /// The date and time that the launch started.
    #[builder(into, default)]
    #[serde(rename = "startedTime")]
    pub r#started_time: Box<Option<String>>,
}
