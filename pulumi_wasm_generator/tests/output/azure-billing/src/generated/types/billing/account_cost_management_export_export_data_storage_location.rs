#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct AccountCostManagementExportExportDataStorageLocation {
    /// The Resource Manager ID of the container where exports will be uploaded. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "containerId")]
    pub r#container_id: Box<String>,
    /// The path of the directory where exports will be uploaded. Changing this forces a new resource to be created.
    /// 
    /// > **Note:** The Resource Manager ID of a Storage Container is exposed via the `resource_manager_id` attribute of the `azure.storage.Container` resource.
    #[builder(into)]
    #[serde(rename = "rootFolderPath")]
    pub r#root_folder_path: Box<String>,
}