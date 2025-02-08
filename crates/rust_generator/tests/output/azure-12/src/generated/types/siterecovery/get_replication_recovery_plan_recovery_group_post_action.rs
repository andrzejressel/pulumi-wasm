#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct GetReplicationRecoveryPlanRecoveryGroupPostAction {
    /// The fabric location of runbook or script.
    #[builder(into)]
    #[serde(rename = "fabricLocation")]
    pub r#fabric_location: Box<String>,
    /// Directions of fail over.
    #[builder(into)]
    #[serde(rename = "failOverDirections")]
    pub r#fail_over_directions: Box<Vec<String>>,
    /// Types of fail over.
    #[builder(into)]
    #[serde(rename = "failOverTypes")]
    pub r#fail_over_types: Box<Vec<String>>,
    /// Instructions of manual action.
    #[builder(into)]
    #[serde(rename = "manualActionInstruction")]
    pub r#manual_action_instruction: Box<String>,
    /// The name of the Replication Plan.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// Id of runbook.
    #[builder(into)]
    #[serde(rename = "runbookId")]
    pub r#runbook_id: Box<String>,
    /// Path of action script.
    #[builder(into)]
    #[serde(rename = "scriptPath")]
    pub r#script_path: Box<String>,
    /// Type of the action detail.
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: Box<String>,
}
