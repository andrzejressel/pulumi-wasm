#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct ServerWorkflowDetailsOnPartialUpload {
    /// Includes the necessary permissions for S3, EFS, and Lambda operations that Transfer can assume, so that all workflow steps can operate on the required resources.
    #[builder(into)]
    #[serde(rename = "executionRole")]
    pub r#execution_role: Box<String>,
    /// A unique identifier for the workflow.
    #[builder(into)]
    #[serde(rename = "workflowId")]
    pub r#workflow_id: Box<String>,
}
