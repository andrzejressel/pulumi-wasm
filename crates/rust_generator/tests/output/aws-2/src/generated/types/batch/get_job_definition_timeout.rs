#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct GetJobDefinitionTimeout {
    /// The job timeout time (in seconds) that's measured from the job attempt's startedAt timestamp.
    #[builder(into)]
    #[serde(rename = "attemptDurationSeconds")]
    pub r#attempt_duration_seconds: Box<i32>,
}
