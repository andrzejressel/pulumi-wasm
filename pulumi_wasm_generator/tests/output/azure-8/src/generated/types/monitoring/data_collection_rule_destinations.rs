#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct DataCollectionRuleDestinations {
    /// A `azure_monitor_metrics` block as defined above.
    #[builder(into, default)]
    #[serde(rename = "azureMonitorMetrics")]
    pub r#azure_monitor_metrics: Box<Option<super::super::types::monitoring::DataCollectionRuleDestinationsAzureMonitorMetrics>>,
    /// One or more `event_hub` blocks as defined below.
    #[builder(into, default)]
    #[serde(rename = "eventHub")]
    pub r#event_hub: Box<Option<super::super::types::monitoring::DataCollectionRuleDestinationsEventHub>>,
    /// One or more `event_hub` blocks as defined below.
    #[builder(into, default)]
    #[serde(rename = "eventHubDirect")]
    pub r#event_hub_direct: Box<Option<super::super::types::monitoring::DataCollectionRuleDestinationsEventHubDirect>>,
    /// One or more `log_analytics` blocks as defined below.
    #[builder(into, default)]
    #[serde(rename = "logAnalytics")]
    pub r#log_analytics: Box<Option<Vec<super::super::types::monitoring::DataCollectionRuleDestinationsLogAnalytic>>>,
    /// One or more `monitor_account` blocks as defined below.
    #[builder(into, default)]
    #[serde(rename = "monitorAccounts")]
    pub r#monitor_accounts: Box<Option<Vec<super::super::types::monitoring::DataCollectionRuleDestinationsMonitorAccount>>>,
    /// One or more `storage_blob_direct` blocks as defined below.
    #[builder(into, default)]
    #[serde(rename = "storageBlobDirects")]
    pub r#storage_blob_directs: Box<Option<Vec<super::super::types::monitoring::DataCollectionRuleDestinationsStorageBlobDirect>>>,
    /// One or more `storage_blob` blocks as defined below.
    #[builder(into, default)]
    #[serde(rename = "storageBlobs")]
    pub r#storage_blobs: Box<Option<Vec<super::super::types::monitoring::DataCollectionRuleDestinationsStorageBlob>>>,
    /// One or more `storage_table_direct` blocks as defined below.
    /// 
    /// > **NOTE** `event_hub_direct`, `storage_blob_direct`, and `storage_table_direct` are only available for rules of kind `AgentDirectToStore`.
    /// 
    /// > **NOTE** At least one of `azure_monitor_metrics`, `event_hub`, `event_hub_direct`, `log_analytics`, `monitor_account`, `storage_blob`, `storage_blob_direct`,and `storage_table_direct` blocks must be specified.
    #[builder(into, default)]
    #[serde(rename = "storageTableDirects")]
    pub r#storage_table_directs: Box<Option<Vec<super::super::types::monitoring::DataCollectionRuleDestinationsStorageTableDirect>>>,
}
