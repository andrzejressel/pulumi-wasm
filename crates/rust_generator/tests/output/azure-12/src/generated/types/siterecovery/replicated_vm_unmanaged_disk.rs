#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct ReplicatedVmUnmanagedDisk {
    /// Id of disk that should be replicated. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "diskUri")]
    pub r#disk_uri: Box<String>,
    /// Storage account that should be used for caching. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "stagingStorageAccountId")]
    pub r#staging_storage_account_id: Box<String>,
    /// Storage account disk should belong to when a failover is done. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "targetStorageAccountId")]
    pub r#target_storage_account_id: Box<String>,
}
