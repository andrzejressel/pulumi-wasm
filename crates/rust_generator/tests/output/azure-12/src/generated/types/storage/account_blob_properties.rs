#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct AccountBlobProperties {
    /// Is the blob service properties for change feed events enabled? Default to `false`.
    /// 
    /// > **Note:** This field cannot be configured when `kind` is set to `Storage` (V1).
    #[builder(into, default)]
    #[serde(rename = "changeFeedEnabled")]
    pub r#change_feed_enabled: Box<Option<bool>>,
    /// The duration of change feed events retention in days. The possible values are between 1 and 146000 days (400 years). Setting this to null (or omit this in the configuration file) indicates an infinite retention of the change feed.
    /// 
    /// > **Note:** This field cannot be configured when `kind` is set to `Storage` (V1).
    #[builder(into, default)]
    #[serde(rename = "changeFeedRetentionInDays")]
    pub r#change_feed_retention_in_days: Box<Option<i32>>,
    /// A `container_delete_retention_policy` block as defined below.
    #[builder(into, default)]
    #[serde(rename = "containerDeleteRetentionPolicy")]
    pub r#container_delete_retention_policy: Box<Option<super::super::types::storage::AccountBlobPropertiesContainerDeleteRetentionPolicy>>,
    /// A `cors_rule` block as defined below.
    #[builder(into, default)]
    #[serde(rename = "corsRules")]
    pub r#cors_rules: Box<Option<Vec<super::super::types::storage::AccountBlobPropertiesCorsRule>>>,
    /// The API Version which should be used by default for requests to the Data Plane API if an incoming request doesn't specify an API Version.
    #[builder(into, default)]
    #[serde(rename = "defaultServiceVersion")]
    pub r#default_service_version: Box<Option<String>>,
    /// A `delete_retention_policy` block as defined below.
    #[builder(into, default)]
    #[serde(rename = "deleteRetentionPolicy")]
    pub r#delete_retention_policy: Box<Option<super::super::types::storage::AccountBlobPropertiesDeleteRetentionPolicy>>,
    /// Is the last access time based tracking enabled? Default to `false`.
    /// 
    /// > **Note:** This field cannot be configured when `kind` is set to `Storage` (V1).
    #[builder(into, default)]
    #[serde(rename = "lastAccessTimeEnabled")]
    pub r#last_access_time_enabled: Box<Option<bool>>,
    /// A `restore_policy` block as defined below. This must be used together with `delete_retention_policy` set, `versioning_enabled` and `change_feed_enabled` set to `true`.
    /// 
    /// > **Note:** This field cannot be configured when `kind` is set to `Storage` (V1).
    /// 
    /// > **Note:** `restore_policy` can not be configured when `dns_endpoint_type` is `AzureDnsZone`.
    #[builder(into, default)]
    #[serde(rename = "restorePolicy")]
    pub r#restore_policy: Box<Option<super::super::types::storage::AccountBlobPropertiesRestorePolicy>>,
    /// Is versioning enabled? Default to `false`.
    /// 
    /// > **Note:** This field cannot be configured when `kind` is set to `Storage` (V1).
    #[builder(into, default)]
    #[serde(rename = "versioningEnabled")]
    pub r#versioning_enabled: Box<Option<bool>>,
}
