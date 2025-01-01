#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ProviderFeaturesVirtualMachine {
    #[builder(into, default)]
    #[serde(rename = "deleteOsDiskOnDeletion")]
    pub r#delete_os_disk_on_deletion: Box<Option<bool>>,
    #[builder(into, default)]
    #[serde(rename = "detachImplicitDataDiskOnDeletion")]
    pub r#detach_implicit_data_disk_on_deletion: Box<Option<bool>>,
    #[builder(into, default)]
    #[serde(rename = "gracefulShutdown")]
    pub r#graceful_shutdown: Box<Option<bool>>,
    #[builder(into, default)]
    #[serde(rename = "skipShutdownAndForceDelete")]
    pub r#skip_shutdown_and_force_delete: Box<Option<bool>>,
}
