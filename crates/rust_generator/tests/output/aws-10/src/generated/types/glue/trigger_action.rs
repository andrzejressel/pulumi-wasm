#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct TriggerAction {
    /// Arguments to be passed to the job. You can specify arguments here that your own job-execution script consumes, as well as arguments that AWS Glue itself consumes.
    #[builder(into, default)]
    #[serde(rename = "arguments")]
    pub r#arguments: Box<Option<std::collections::HashMap<String, String>>>,
    /// The name of the crawler to be executed. Conflicts with `job_name`.
    #[builder(into, default)]
    #[serde(rename = "crawlerName")]
    pub r#crawler_name: Box<Option<String>>,
    /// The name of a job to be executed. Conflicts with `crawler_name`.
    #[builder(into, default)]
    #[serde(rename = "jobName")]
    pub r#job_name: Box<Option<String>>,
    /// Specifies configuration properties of a job run notification. See Notification Property details below.
    #[builder(into, default)]
    #[serde(rename = "notificationProperty")]
    pub r#notification_property: Box<Option<super::super::types::glue::TriggerActionNotificationProperty>>,
    /// The name of the Security Configuration structure to be used with this action.
    #[builder(into, default)]
    #[serde(rename = "securityConfiguration")]
    pub r#security_configuration: Box<Option<String>>,
    /// The job run timeout in minutes. It overrides the timeout value of the job.
    #[builder(into, default)]
    #[serde(rename = "timeout")]
    pub r#timeout: Box<Option<i32>>,
}
