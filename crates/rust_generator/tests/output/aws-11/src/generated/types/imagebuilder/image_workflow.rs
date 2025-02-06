#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ImageWorkflow {
    /// The action to take if the workflow fails. Must be one of `CONTINUE` or `ABORT`.
    #[builder(into, default)]
    #[serde(rename = "onFailure")]
    pub r#on_failure: Box<Option<String>>,
    /// The parallel group in which to run a test Workflow.
    #[builder(into, default)]
    #[serde(rename = "parallelGroup")]
    pub r#parallel_group: Box<Option<String>>,
    /// Configuration block for the workflow parameters. Detailed below.
    #[builder(into, default)]
    #[serde(rename = "parameters")]
    pub r#parameters: Box<Option<Vec<super::super::types::imagebuilder::ImageWorkflowParameter>>>,
    /// Amazon Resource Name (ARN) of the Image Builder Workflow.
    /// 
    /// The following arguments are optional:
    #[builder(into)]
    #[serde(rename = "workflowArn")]
    pub r#workflow_arn: Box<String>,
}
