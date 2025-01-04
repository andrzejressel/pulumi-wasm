#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct KafkaClusterStorageAccountGen2 {
    /// The ID of the Gen2 Filesystem. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "filesystemId")]
    pub r#filesystem_id: Box<String>,
    /// Is this the Default Storage Account for the HDInsight Hadoop Cluster? Changing this forces a new resource to be created.
    /// 
    /// > **NOTE:** One of the `storage_account` or `storage_account_gen2` blocks must be marked as the default.
    #[builder(into)]
    #[serde(rename = "isDefault")]
    pub r#is_default: Box<bool>,
    /// The ID of Managed Identity to use for accessing the Gen2 filesystem. Changing this forces a new resource to be created.
    /// 
    /// > **NOTE:** This can be obtained from the `id` of the `azure.storage.Container` resource.
    #[builder(into)]
    #[serde(rename = "managedIdentityResourceId")]
    pub r#managed_identity_resource_id: Box<String>,
    /// The ID of the Storage Account. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "storageResourceId")]
    pub r#storage_resource_id: Box<String>,
}
