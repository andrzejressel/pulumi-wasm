#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ManagementPolicyRuleActionsBaseBlob {
    /// Whether a blob should automatically be tiered from cool back to hot if it's accessed again after being tiered to cool. Defaults to `false`.
    /// 
    /// > **Note:** The `auto_tier_to_hot_from_cool_enabled` must be used together with `tier_to_cool_after_days_since_last_access_time_greater_than`.
    #[builder(into, default)]
    #[serde(rename = "autoTierToHotFromCoolEnabled")]
    pub r#auto_tier_to_hot_from_cool_enabled: Box<Option<bool>>,
    /// The age in days after creation to delete the blob. Must be between `0` and `99999`. Defaults to `-1`.
    /// 
    /// > **Note:** The `delete_after_days_since_modification_greater_than`, `delete_after_days_since_last_access_time_greater_than` and `delete_after_days_since_creation_greater_than` can not be set at the same time.
    /// 
    /// > **Note:** The `last_access_time_enabled` must be set to `true` in the `azure.storage.Account` in order to use `tier_to_cool_after_days_since_last_access_time_greater_than`, `tier_to_archive_after_days_since_last_access_time_greater_than` and `delete_after_days_since_last_access_time_greater_than`.
    #[builder(into, default)]
    #[serde(rename = "deleteAfterDaysSinceCreationGreaterThan")]
    pub r#delete_after_days_since_creation_greater_than: Box<Option<i32>>,
    /// The age in days after last access time to delete the blob. Must be between `0` and `99999`. Defaults to `-1`.
    #[builder(into, default)]
    #[serde(rename = "deleteAfterDaysSinceLastAccessTimeGreaterThan")]
    pub r#delete_after_days_since_last_access_time_greater_than: Box<Option<i32>>,
    /// The age in days after last modification to delete the blob. Must be between `0` and `99999`. Defaults to `-1`.
    #[builder(into, default)]
    #[serde(rename = "deleteAfterDaysSinceModificationGreaterThan")]
    pub r#delete_after_days_since_modification_greater_than: Box<Option<i32>>,
    /// The age in days after creation to archive storage. Supports blob currently at Hot or Cool tier. Must be between `0` and `99999`. Defaults to `-1`.
    /// 
    /// > **Note:** The `tier_to_archive_after_days_since_modification_greater_than`, `tier_to_archive_after_days_since_last_access_time_greater_than` and `tier_to_archive_after_days_since_creation_greater_than` can not be set at the same time.
    #[builder(into, default)]
    #[serde(rename = "tierToArchiveAfterDaysSinceCreationGreaterThan")]
    pub r#tier_to_archive_after_days_since_creation_greater_than: Box<Option<i32>>,
    /// The age in days after last access time to tier blobs to archive storage. Supports blob currently at Hot or Cool tier. Must be between `0` and `99999`. Defaults to `-1`.
    #[builder(into, default)]
    #[serde(rename = "tierToArchiveAfterDaysSinceLastAccessTimeGreaterThan")]
    pub r#tier_to_archive_after_days_since_last_access_time_greater_than: Box<Option<i32>>,
    /// The age in days after last tier change to the blobs to skip to be archved. Must be between `0` and `99999`. Defaults to `-1`.
    #[builder(into, default)]
    #[serde(rename = "tierToArchiveAfterDaysSinceLastTierChangeGreaterThan")]
    pub r#tier_to_archive_after_days_since_last_tier_change_greater_than: Box<Option<i32>>,
    /// The age in days after last modification to tier blobs to archive storage. Supports blob currently at Hot or Cool tier. Must be between `0` and `99999`. Defaults to `-1`.
    #[builder(into, default)]
    #[serde(rename = "tierToArchiveAfterDaysSinceModificationGreaterThan")]
    pub r#tier_to_archive_after_days_since_modification_greater_than: Box<Option<i32>>,
    /// The age in days after creation to cold storage. Supports blob currently at Hot tier. Must be between `0` and `99999`. Defaults to `-1`.
    /// 
    /// > **Note:** The `tier_to_cool_after_days_since_modification_greater_than`, `tier_to_cool_after_days_since_last_access_time_greater_than` and `tier_to_cool_after_days_since_creation_greater_than` can not be set at the same time.
    #[builder(into, default)]
    #[serde(rename = "tierToColdAfterDaysSinceCreationGreaterThan")]
    pub r#tier_to_cold_after_days_since_creation_greater_than: Box<Option<i32>>,
    /// The age in days after last access time to tier blobs to cold storage. Supports blob currently at Hot tier. Must be between `0` and `99999`. Defaults to `-1`.
    #[builder(into, default)]
    #[serde(rename = "tierToColdAfterDaysSinceLastAccessTimeGreaterThan")]
    pub r#tier_to_cold_after_days_since_last_access_time_greater_than: Box<Option<i32>>,
    /// The age in days after last modification to tier blobs to cold storage. Supports blob currently at Hot tier. Must be between `0` and `99999`. Defaults to `-1`.
    #[builder(into, default)]
    #[serde(rename = "tierToColdAfterDaysSinceModificationGreaterThan")]
    pub r#tier_to_cold_after_days_since_modification_greater_than: Box<Option<i32>>,
    /// The age in days after creation to cool storage. Supports blob currently at Hot tier. Must be between `0` and `99999`. Defaults to `-1`.
    /// 
    /// > **Note:** The `tier_to_cool_after_days_since_modification_greater_than`, `tier_to_cool_after_days_since_last_access_time_greater_than` and `tier_to_cool_after_days_since_creation_greater_than` can not be set at the same time.
    #[builder(into, default)]
    #[serde(rename = "tierToCoolAfterDaysSinceCreationGreaterThan")]
    pub r#tier_to_cool_after_days_since_creation_greater_than: Box<Option<i32>>,
    /// The age in days after last access time to tier blobs to cool storage. Supports blob currently at Hot tier. Must be between `0` and `99999`. Defaults to `-1`.
    #[builder(into, default)]
    #[serde(rename = "tierToCoolAfterDaysSinceLastAccessTimeGreaterThan")]
    pub r#tier_to_cool_after_days_since_last_access_time_greater_than: Box<Option<i32>>,
    /// The age in days after last modification to tier blobs to cool storage. Supports blob currently at Hot tier. Must be between `0` and `99999`. Defaults to `-1`.
    #[builder(into, default)]
    #[serde(rename = "tierToCoolAfterDaysSinceModificationGreaterThan")]
    pub r#tier_to_cool_after_days_since_modification_greater_than: Box<Option<i32>>,
}
