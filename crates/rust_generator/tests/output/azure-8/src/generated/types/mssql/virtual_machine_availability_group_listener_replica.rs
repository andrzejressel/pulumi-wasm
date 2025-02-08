#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct VirtualMachineAvailabilityGroupListenerReplica {
    /// The replica commit mode for the availability group. Possible values are `Synchronous_Commit` and `Asynchronous_Commit`. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "commit")]
    pub r#commit: Box<String>,
    /// The replica failover mode for the availability group. Possible values are `Manual` and `Automatic`. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "failoverMode")]
    pub r#failover_mode: Box<String>,
    /// The replica readable secondary mode for the availability group. Possible values are `No`, `Read_Only` and `All`. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "readableSecondary")]
    pub r#readable_secondary: Box<String>,
    /// The replica role for the availability group. Possible values are `Primary` and `Secondary`. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "role")]
    pub r#role: Box<String>,
    /// The ID of the SQL Virtual Machine. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "sqlVirtualMachineId")]
    pub r#sql_virtual_machine_id: Box<String>,
}
