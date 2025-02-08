#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct RepositoryWorkflowConfigRecentScheduledExecutionRecord {
    /// (Output)
    /// The error status encountered upon this attempt to create the workflow invocation, if the attempt was unsuccessful.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "errorStatuses")]
    pub r#error_statuses: Box<Option<Vec<super::super::types::dataform::RepositoryWorkflowConfigRecentScheduledExecutionRecordErrorStatus>>>,
    /// (Output)
    /// The timestamp of this workflow attempt.
    #[builder(into, default)]
    #[serde(rename = "executionTime")]
    pub r#execution_time: Box<Option<String>>,
    /// (Output)
    /// The name of the created workflow invocation, if one was successfully created. In the format projects/*/locations/*/repositories/*/workflowInvocations/*.
    #[builder(into, default)]
    #[serde(rename = "workflowInvocation")]
    pub r#workflow_invocation: Box<Option<String>>,
}
