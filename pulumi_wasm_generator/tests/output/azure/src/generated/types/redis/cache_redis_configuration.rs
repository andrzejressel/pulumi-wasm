#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct CacheRedisConfiguration {
    /// Enable Microsoft Entra (AAD) authentication. Defaults to `false`.
    #[builder(into, default)]
    #[serde(rename = "activeDirectoryAuthenticationEnabled")]
    pub r#active_directory_authentication_enabled: Box<Option<bool>>,
    /// Enable or disable AOF persistence for this Redis Cache. Defaults to `false`.
    /// 
    /// > **NOTE:** `aof_backup_enabled` can only be set when SKU is `Premium`.
    #[builder(into, default)]
    #[serde(rename = "aofBackupEnabled")]
    pub r#aof_backup_enabled: Box<Option<bool>>,
    /// First Storage Account connection string for AOF persistence.
    #[builder(into, default)]
    #[serde(rename = "aofStorageConnectionString0")]
    pub r#aof_storage_connection_string_0: Box<Option<String>>,
    /// Second Storage Account connection string for AOF persistence.
    /// 
    /// Example usage:
    /// 
    #[builder(into, default)]
    #[serde(rename = "aofStorageConnectionString1")]
    pub r#aof_storage_connection_string_1: Box<Option<String>>,
    /// If set to `false`, the Redis instance will be accessible without authentication. Defaults to `true`.
    /// 
    /// > **NOTE:** `authentication_enabled` can only be set to `false` if a `subnet_id` is specified; and only works if there aren't existing instances within the subnet with `authentication_enabled` set to `true`.
    #[builder(into, default)]
    #[serde(rename = "authenticationEnabled")]
    pub r#authentication_enabled: Box<Option<bool>>,
    /// Preferred auth method to communicate to storage account used for data persistence. Possible values are `SAS` and `ManagedIdentity`.
    #[builder(into, default)]
    #[serde(rename = "dataPersistenceAuthenticationMethod")]
    pub r#data_persistence_authentication_method: Box<Option<String>>,
    /// Returns the max number of connected clients at the same time.
    #[builder(into, default)]
    #[serde(rename = "maxclients")]
    pub r#maxclients: Box<Option<i32>>,
    /// Value in megabytes reserved to accommodate for memory fragmentation. Defaults are shown below.
    #[builder(into, default)]
    #[serde(rename = "maxfragmentationmemoryReserved")]
    pub r#maxfragmentationmemory_reserved: Box<Option<i32>>,
    /// The max-memory delta for this Redis instance. Defaults are shown below.
    #[builder(into, default)]
    #[serde(rename = "maxmemoryDelta")]
    pub r#maxmemory_delta: Box<Option<i32>>,
    /// How Redis will select what to remove when `maxmemory` is reached. Defaults to `volatile-lru`.
    #[builder(into, default)]
    #[serde(rename = "maxmemoryPolicy")]
    pub r#maxmemory_policy: Box<Option<String>>,
    /// Value in megabytes reserved for non-cache usage e.g. failover. Defaults are shown below.
    #[builder(into, default)]
    #[serde(rename = "maxmemoryReserved")]
    pub r#maxmemory_reserved: Box<Option<i32>>,
    /// Keyspace notifications allows clients to subscribe to Pub/Sub channels in order to receive events affecting the Redis data set in some way. [Reference](https://redis.io/topics/notifications#configuration)
    /// 
    #[builder(into, default)]
    #[serde(rename = "notifyKeyspaceEvents")]
    pub r#notify_keyspace_events: Box<Option<String>>,
    /// Is Backup Enabled? Only supported on Premium SKUs. Defaults to `false`.
    /// 
    /// > **NOTE:** If `rdb_backup_enabled` set to `true`, `rdb_storage_connection_string` must also be set.
    #[builder(into, default)]
    #[serde(rename = "rdbBackupEnabled")]
    pub r#rdb_backup_enabled: Box<Option<bool>>,
    /// The Backup Frequency in Minutes. Only supported on Premium SKUs. Possible values are: `15`, `30`, `60`, `360`, `720` and `1440`.
    #[builder(into, default)]
    #[serde(rename = "rdbBackupFrequency")]
    pub r#rdb_backup_frequency: Box<Option<i32>>,
    /// The maximum number of snapshots to create as a backup. Only supported for Premium SKUs.
    #[builder(into, default)]
    #[serde(rename = "rdbBackupMaxSnapshotCount")]
    pub r#rdb_backup_max_snapshot_count: Box<Option<i32>>,
    /// The Connection String to the Storage Account. Only supported for Premium SKUs. In the format: `DefaultEndpointsProtocol=https;BlobEndpoint=${azurerm_storage_account.example.primary_blob_endpoint};AccountName=${azurerm_storage_account.example.name};AccountKey=${azurerm_storage_account.example.primary_access_key}`.
    /// 
    /// > **NOTE:** There's a bug in the Redis API where the original storage connection string isn't being returned, which [is being tracked in this issue](https://github.com/Azure/azure-rest-api-specs/issues/3037). In the interim you can use [the `ignoreChanges` attribute to ignore changes to this field](https://www.pulumi.com/docs/intro/concepts/programming-model/#ignorechanges) e.g.:
    #[builder(into, default)]
    #[serde(rename = "rdbStorageConnectionString")]
    pub r#rdb_storage_connection_string: Box<Option<String>>,
    /// The ID of the Subscription containing the Storage Account.
    /// 
    /// ```yaml
    /// resources:
    ///   example:
    ///     type: azure:redis:Cache
    ///     properties:
    ///       ignoreChanges:
    ///         - ${redisConfiguration[0].rdbStorageConnectionString}
    /// ```
    #[builder(into, default)]
    #[serde(rename = "storageAccountSubscriptionId")]
    pub r#storage_account_subscription_id: Box<Option<String>>,
}
