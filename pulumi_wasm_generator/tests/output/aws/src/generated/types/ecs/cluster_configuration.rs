#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ClusterConfiguration {
    /// Details of the execute command configuration. See `execute_command_configuration` Block for details.
    #[builder(into, default)]
    #[serde(rename = "executeCommandConfiguration")]
    pub r#execute_command_configuration: Box<Option<super::super::types::ecs::ClusterConfigurationExecuteCommandConfiguration>>,
    /// Details of the managed storage configuration. See `managed_storage_configuration` Block for details.
    #[builder(into, default)]
    #[serde(rename = "managedStorageConfiguration")]
    pub r#managed_storage_configuration: Box<Option<super::super::types::ecs::ClusterConfigurationManagedStorageConfiguration>>,
}