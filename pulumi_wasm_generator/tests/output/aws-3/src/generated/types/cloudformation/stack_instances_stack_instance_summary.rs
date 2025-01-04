#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct StackInstancesStackInstanceSummary {
    /// Account ID in which the instance is deployed.
    #[builder(into, default)]
    #[serde(rename = "accountId")]
    pub r#account_id: Box<Option<String>>,
    /// Detailed status of the stack instance. Values include `PENDING`, `RUNNING`, `SUCCEEDED`, `FAILED`, `CANCELLED`, `INOPERABLE`, `SKIPPED_SUSPENDED_ACCOUNT`, `FAILED_IMPORT`.
    #[builder(into, default)]
    #[serde(rename = "detailedStatus")]
    pub r#detailed_status: Box<Option<String>>,
    /// Status of the stack instance's actual configuration compared to the expected template and parameter configuration of the stack set to which it belongs. Values include `DRIFTED`, `IN_SYNC`, `UNKNOWN`, `NOT_CHECKED`.
    #[builder(into, default)]
    #[serde(rename = "driftStatus")]
    pub r#drift_status: Box<Option<String>>,
    /// Organization root ID or organizational unit (OU) IDs that you specified for `deployment_targets`.
    #[builder(into, default)]
    #[serde(rename = "organizationalUnitId")]
    pub r#organizational_unit_id: Box<Option<String>>,
    /// Region that the stack instance is associated with.
    #[builder(into, default)]
    #[serde(rename = "region")]
    pub r#region: Box<Option<String>>,
    /// ID of the stack instance.
    #[builder(into, default)]
    #[serde(rename = "stackId")]
    pub r#stack_id: Box<Option<String>>,
    /// Name or unique ID of the stack set that the stack instance is associated with.
    #[builder(into, default)]
    #[serde(rename = "stackSetId")]
    pub r#stack_set_id: Box<Option<String>>,
    /// Status of the stack instance, in terms of its synchronization with its associated stack set. Values include `CURRENT`, `OUTDATED`, `INOPERABLE`.
    #[builder(into, default)]
    #[serde(rename = "status")]
    pub r#status: Box<Option<String>>,
    /// Explanation for the specific status code assigned to this stack instance.
    #[builder(into, default)]
    #[serde(rename = "statusReason")]
    pub r#status_reason: Box<Option<String>>,
}
