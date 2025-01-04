#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ReplicationRecoveryPlanFailoverRecoveryGroupPreAction {
    /// The fabric location of runbook or script. Possible values are `Primary` and `Recovery`. It must not be specified when `type` is `ManualActionDetails`.
    /// 
    /// > **NOTE:** This is required when `type` is set to `AutomationRunbookActionDetails` or `ScriptActionDetails`.
    #[builder(into, default)]
    #[serde(rename = "fabricLocation")]
    pub r#fabric_location: Box<Option<String>>,
    /// Directions of fail over. Possible values are `PrimaryToRecovery` and `RecoveryToPrimary`
    #[builder(into)]
    #[serde(rename = "failOverDirections")]
    pub r#fail_over_directions: Box<Vec<String>>,
    /// Types of fail over. Possible values are `TestFailover`, `PlannedFailover` and `UnplannedFailover`
    #[builder(into)]
    #[serde(rename = "failOverTypes")]
    pub r#fail_over_types: Box<Vec<String>>,
    /// Instructions of manual action.
    /// 
    /// > **NOTE:** This property is required when `type` is set to `ManualActionDetails`.
    #[builder(into, default)]
    #[serde(rename = "manualActionInstruction")]
    pub r#manual_action_instruction: Box<Option<String>>,
    /// The name of the Replication Plan. The name can contain only letters, numbers, and hyphens. It should start with a letter and end with a letter or a number. Can be a maximum of 63 characters. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// Id of runbook.
    /// 
    /// > **NOTE:** This property is required when `type` is set to `AutomationRunbookActionDetails`.
    #[builder(into, default)]
    #[serde(rename = "runbookId")]
    pub r#runbook_id: Box<Option<String>>,
    /// Path of action script.
    /// 
    /// > **NOTE:** This property is required when `type` is set to `ScriptActionDetails`.
    #[builder(into, default)]
    #[serde(rename = "scriptPath")]
    pub r#script_path: Box<Option<String>>,
    /// Type of the action detail. Possible values are `AutomationRunbookActionDetails`, `ManualActionDetails` and `ScriptActionDetails`.
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: Box<String>,
}
