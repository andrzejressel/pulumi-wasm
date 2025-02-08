#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetCacheRedisConfiguration {
    /// Specifies if Microsoft Entra (AAD) authentication is enabled.
    #[builder(into)]
    #[serde(rename = "activeDirectoryAuthenticationEnabled")]
    pub r#active_directory_authentication_enabled: Box<bool>,
    #[builder(into)]
    #[serde(rename = "aofBackupEnabled")]
    pub r#aof_backup_enabled: Box<bool>,
    #[builder(into)]
    #[serde(rename = "aofStorageConnectionString0")]
    pub r#aof_storage_connection_string_0: Box<String>,
    #[builder(into)]
    #[serde(rename = "aofStorageConnectionString1")]
    pub r#aof_storage_connection_string_1: Box<String>,
    #[builder(into)]
    #[serde(rename = "authenticationEnabled")]
    pub r#authentication_enabled: Box<bool>,
    #[builder(into)]
    #[serde(rename = "dataPersistenceAuthenticationMethod")]
    pub r#data_persistence_authentication_method: Box<String>,
    #[builder(into)]
    #[serde(rename = "maxclients")]
    pub r#maxclients: Box<i32>,
    /// Value in megabytes reserved to accommodate for memory fragmentation.
    #[builder(into)]
    #[serde(rename = "maxfragmentationmemoryReserved")]
    pub r#maxfragmentationmemory_reserved: Box<i32>,
    /// The max-memory delta for this Redis instance.
    #[builder(into)]
    #[serde(rename = "maxmemoryDelta")]
    pub r#maxmemory_delta: Box<i32>,
    /// How Redis will select what to remove when `maxmemory` is reached.
    #[builder(into)]
    #[serde(rename = "maxmemoryPolicy")]
    pub r#maxmemory_policy: Box<String>,
    /// The value in megabytes reserved for non-cache usage e.g. failover
    #[builder(into)]
    #[serde(rename = "maxmemoryReserved")]
    pub r#maxmemory_reserved: Box<i32>,
    #[builder(into)]
    #[serde(rename = "notifyKeyspaceEvents")]
    pub r#notify_keyspace_events: Box<String>,
    /// Is Backup Enabled? Only supported on Premium SKUs.
    #[builder(into)]
    #[serde(rename = "rdbBackupEnabled")]
    pub r#rdb_backup_enabled: Box<bool>,
    /// The Backup Frequency in Minutes. Only supported on Premium SKUs.
    #[builder(into)]
    #[serde(rename = "rdbBackupFrequency")]
    pub r#rdb_backup_frequency: Box<i32>,
    /// The maximum number of snapshots that can be created as a backup.
    #[builder(into)]
    #[serde(rename = "rdbBackupMaxSnapshotCount")]
    pub r#rdb_backup_max_snapshot_count: Box<i32>,
    /// The Connection String to the Storage Account. Only supported for Premium SKUs.
    #[builder(into)]
    #[serde(rename = "rdbStorageConnectionString")]
    pub r#rdb_storage_connection_string: Box<String>,
    /// The ID of the Subscription containing the Storage Account.
    #[builder(into)]
    #[serde(rename = "storageAccountSubscriptionId")]
    pub r#storage_account_subscription_id: Box<String>,
}
