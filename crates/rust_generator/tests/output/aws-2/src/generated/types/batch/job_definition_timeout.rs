#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct JobDefinitionTimeout {
    /// Time duration in seconds after which AWS Batch terminates your jobs if they have not finished. The minimum value for the timeout is `60` seconds.
    #[builder(into, default)]
    #[serde(rename = "attemptDurationSeconds")]
    pub r#attempt_duration_seconds: Box<Option<i32>>,
}
