#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct HBaseClusterStorageAccount {
    /// Is this the Default Storage Account for the HDInsight Hadoop Cluster? Changing this forces a new resource to be created.
    /// 
    /// > **NOTE:** One of the `storage_account` or `storage_account_gen2` blocks must be marked as the default.
    #[builder(into)]
    #[serde(rename = "isDefault")]
    pub r#is_default: Box<bool>,
    /// The Access Key which should be used to connect to the Storage Account. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "storageAccountKey")]
    pub r#storage_account_key: Box<String>,
    /// The ID of the Storage Container. Changing this forces a new resource to be created.
    /// 
    /// > **NOTE:** This can be obtained from the `id` of the `azure.storage.Container` resource.
    #[builder(into)]
    #[serde(rename = "storageContainerId")]
    pub r#storage_container_id: Box<String>,
    /// The ID of the Storage Account. Changing this forces a new resource to be created.
    #[builder(into, default)]
    #[serde(rename = "storageResourceId")]
    pub r#storage_resource_id: Box<Option<String>>,
}
