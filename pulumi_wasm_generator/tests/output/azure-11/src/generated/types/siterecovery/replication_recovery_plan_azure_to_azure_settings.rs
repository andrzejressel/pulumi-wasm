#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ReplicationRecoveryPlanAzureToAzureSettings {
    /// The Edge Zone within the Azure Region where the VM exists. Changing this forces a new Site Recovery Replication Recovery Plan to be created.
    #[builder(into, default)]
    #[serde(rename = "primaryEdgeZone")]
    pub r#primary_edge_zone: Box<Option<String>>,
    /// The Availability Zone in which the VM is located. Changing this forces a new Site Recovery Replication Recovery Plan to be created.
    #[builder(into, default)]
    #[serde(rename = "primaryZone")]
    pub r#primary_zone: Box<Option<String>>,
    /// The Edge Zone within the Azure Region where the VM is recovered. Changing this forces a new Site Recovery Replication Recovery Plan to be created.
    /// 
    /// > **Note:** `primary_edge_zone` and `recovery_edge_zone` must be specified together.
    #[builder(into, default)]
    #[serde(rename = "recoveryEdgeZone")]
    pub r#recovery_edge_zone: Box<Option<String>>,
    /// The Availability Zone in which the VM is recovered. Changing this forces a new Site Recovery Replication Recovery Plan to be created.
    /// 
    /// > **Note:** `primary_zone` and `recovery_zone` must be specified together.
    #[builder(into, default)]
    #[serde(rename = "recoveryZone")]
    pub r#recovery_zone: Box<Option<String>>,
}
