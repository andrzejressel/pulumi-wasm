#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetDataCollectionRuleDestination {
    /// A `azure_monitor_metrics` block as defined above.
    #[builder(into)]
    #[serde(rename = "azureMonitorMetrics")]
    pub r#azure_monitor_metrics: Box<Vec<super::super::types::monitoring::GetDataCollectionRuleDestinationAzureMonitorMetric>>,
    /// One or more `event_hub` blocks as defined below.
    #[builder(into, default)]
    #[serde(rename = "eventHub")]
    pub r#event_hub: Box<Option<super::super::types::monitoring::GetDataCollectionRuleDestinationEventHub>>,
    /// One or more `event_hub_direct` blocks as defined below.
    #[builder(into, default)]
    #[serde(rename = "eventHubDirect")]
    pub r#event_hub_direct: Box<Option<super::super::types::monitoring::GetDataCollectionRuleDestinationEventHubDirect>>,
    /// One or more `log_analytics` blocks as defined below.
    #[builder(into)]
    #[serde(rename = "logAnalytics")]
    pub r#log_analytics: Box<Vec<super::super::types::monitoring::GetDataCollectionRuleDestinationLogAnalytic>>,
    /// One or more `monitor_account` blocks as defined below.
    #[builder(into)]
    #[serde(rename = "monitorAccounts")]
    pub r#monitor_accounts: Box<Vec<super::super::types::monitoring::GetDataCollectionRuleDestinationMonitorAccount>>,
    /// One or more `storage_blob_direct` blocks as defined below.
    #[builder(into)]
    #[serde(rename = "storageBlobDirects")]
    pub r#storage_blob_directs: Box<Vec<super::super::types::monitoring::GetDataCollectionRuleDestinationStorageBlobDirect>>,
    /// One or more `storage_blob` blocks as defined below.
    #[builder(into)]
    #[serde(rename = "storageBlobs")]
    pub r#storage_blobs: Box<Vec<super::super::types::monitoring::GetDataCollectionRuleDestinationStorageBlob>>,
    /// One or more `storage_table_direct` blocks as defined below.
    #[builder(into)]
    #[serde(rename = "storageTableDirects")]
    pub r#storage_table_directs: Box<Vec<super::super::types::monitoring::GetDataCollectionRuleDestinationStorageTableDirect>>,
}
