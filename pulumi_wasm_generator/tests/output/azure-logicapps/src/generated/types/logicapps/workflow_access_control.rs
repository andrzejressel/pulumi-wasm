#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct WorkflowAccessControl {
    /// A `action` block as defined below.
    #[builder(into, default)]
    #[serde(rename = "action")]
    pub r#action: Box<Option<super::super::types::logicapps::WorkflowAccessControlAction>>,
    /// A `content` block as defined below.
    #[builder(into, default)]
    #[serde(rename = "content")]
    pub r#content: Box<Option<super::super::types::logicapps::WorkflowAccessControlContent>>,
    /// A `trigger` block as defined below.
    #[builder(into, default)]
    #[serde(rename = "trigger")]
    pub r#trigger: Box<Option<super::super::types::logicapps::WorkflowAccessControlTrigger>>,
    /// A `workflow_management` block as defined below.
    #[builder(into, default)]
    #[serde(rename = "workflowManagement")]
    pub r#workflow_management: Box<Option<super::super::types::logicapps::WorkflowAccessControlWorkflowManagement>>,
}
