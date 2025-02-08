#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct JobLatestCreatedExecution {
    /// (Output)
    /// Completion timestamp of the execution.
    /// A timestamp in RFC3339 UTC "Zulu" format, with nanosecond resolution and up to nine fractional digits. Examples: "2014-10-02T15:01:23Z" and "2014-10-02T15:01:23.045123456Z".
    #[builder(into, default)]
    #[serde(rename = "completionTime")]
    pub r#completion_time: Box<Option<String>>,
    /// (Output)
    /// Creation timestamp of the execution.
    /// A timestamp in RFC3339 UTC "Zulu" format, with nanosecond resolution and up to nine fractional digits. Examples: "2014-10-02T15:01:23Z" and "2014-10-02T15:01:23.045123456Z".
    #[builder(into, default)]
    #[serde(rename = "createTime")]
    pub r#create_time: Box<Option<String>>,
    /// Name of the Job.
    #[builder(into, default)]
    #[serde(rename = "name")]
    pub r#name: Box<Option<String>>,
}
