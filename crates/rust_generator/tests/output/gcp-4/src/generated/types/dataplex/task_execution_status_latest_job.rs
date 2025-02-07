#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct TaskExecutionStatusLatestJob {
    /// (Output)
    /// The time when the job ended.
    #[builder(into, default)]
    #[serde(rename = "endTime")]
    pub r#end_time: Box<Option<String>>,
    /// (Output)
    /// Additional information about the current state.
    #[builder(into, default)]
    #[serde(rename = "message")]
    pub r#message: Box<Option<String>>,
    /// (Output)
    /// The relative resource name of the job, of the form: projects/{project_number}/locations/{locationId}/lakes/{lakeId}/tasks/{taskId}/jobs/{jobId}.
    #[builder(into, default)]
    #[serde(rename = "name")]
    pub r#name: Box<Option<String>>,
    /// (Output)
    /// The number of times the job has been retried (excluding the initial attempt).
    #[builder(into, default)]
    #[serde(rename = "retryCount")]
    pub r#retry_count: Box<Option<i32>>,
    /// (Output)
    /// The underlying service running a job.
    #[builder(into, default)]
    #[serde(rename = "service")]
    pub r#service: Box<Option<String>>,
    /// (Output)
    /// The full resource name for the job run under a particular service.
    #[builder(into, default)]
    #[serde(rename = "serviceJob")]
    pub r#service_job: Box<Option<String>>,
    /// (Output)
    /// The time when the job was started.
    #[builder(into, default)]
    #[serde(rename = "startTime")]
    pub r#start_time: Box<Option<String>>,
    /// (Output)
    /// Execution state for the job.
    #[builder(into, default)]
    #[serde(rename = "state")]
    pub r#state: Box<Option<String>>,
    /// (Output)
    /// System generated globally unique ID for the job.
    #[builder(into, default)]
    #[serde(rename = "uid")]
    pub r#uid: Box<Option<String>>,
}
