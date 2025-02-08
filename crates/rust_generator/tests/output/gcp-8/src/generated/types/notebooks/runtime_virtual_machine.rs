#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct RuntimeVirtualMachine {
    /// (Output)
    /// The unique identifier of the Managed Compute Engine instance.
    #[builder(into, default)]
    #[serde(rename = "instanceId")]
    pub r#instance_id: Box<Option<String>>,
    /// (Output)
    /// The user-friendly name of the Managed Compute Engine instance.
    #[builder(into, default)]
    #[serde(rename = "instanceName")]
    pub r#instance_name: Box<Option<String>>,
    /// Virtual Machine configuration settings.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "virtualMachineConfig")]
    pub r#virtual_machine_config: Box<Option<super::super::types::notebooks::RuntimeVirtualMachineVirtualMachineConfig>>,
}
