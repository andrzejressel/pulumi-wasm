#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct GetPolicyRuleActionBaseBlob {
    /// Whether a blob should automatically be tiered from cool back to hot if it's accessed again after being tiered to cool.
    #[builder(into)]
    #[serde(rename = "autoTierToHotFromCoolEnabled")]
    pub r#auto_tier_to_hot_from_cool_enabled: Box<bool>,
    /// The age in days after creation to delete the blob snapshot.
    #[builder(into)]
    #[serde(rename = "deleteAfterDaysSinceCreationGreaterThan")]
    pub r#delete_after_days_since_creation_greater_than: Box<i32>,
    /// The age in days after last access time to delete the blob.
    #[builder(into)]
    #[serde(rename = "deleteAfterDaysSinceLastAccessTimeGreaterThan")]
    pub r#delete_after_days_since_last_access_time_greater_than: Box<i32>,
    /// The age in days after last modification to delete the blob.
    #[builder(into)]
    #[serde(rename = "deleteAfterDaysSinceModificationGreaterThan")]
    pub r#delete_after_days_since_modification_greater_than: Box<i32>,
    /// The age in days after creation to archive storage.
    #[builder(into)]
    #[serde(rename = "tierToArchiveAfterDaysSinceCreationGreaterThan")]
    pub r#tier_to_archive_after_days_since_creation_greater_than: Box<i32>,
    /// The age in days after last access time to tier blobs to archive storage.
    #[builder(into)]
    #[serde(rename = "tierToArchiveAfterDaysSinceLastAccessTimeGreaterThan")]
    pub r#tier_to_archive_after_days_since_last_access_time_greater_than: Box<i32>,
    /// The age in days after last tier change to the blobs to skip to be archived.
    #[builder(into)]
    #[serde(rename = "tierToArchiveAfterDaysSinceLastTierChangeGreaterThan")]
    pub r#tier_to_archive_after_days_since_last_tier_change_greater_than: Box<i32>,
    /// The age in days after last modification to tier blobs to archive storage.
    #[builder(into)]
    #[serde(rename = "tierToArchiveAfterDaysSinceModificationGreaterThan")]
    pub r#tier_to_archive_after_days_since_modification_greater_than: Box<i32>,
    /// Optional The age in days after creation to cold storage. Supports blob currently at Hot tier.
    #[builder(into)]
    #[serde(rename = "tierToColdAfterDaysSinceCreationGreaterThan")]
    pub r#tier_to_cold_after_days_since_creation_greater_than: Box<i32>,
    /// The age in days after last access time to tier blobs to cold storage. Supports blob currently at Hot tier.
    #[builder(into)]
    #[serde(rename = "tierToColdAfterDaysSinceLastAccessTimeGreaterThan")]
    pub r#tier_to_cold_after_days_since_last_access_time_greater_than: Box<i32>,
    /// The age in days after last modification to tier blobs to cold storage. Supports blob currently at Hot tier.
    #[builder(into)]
    #[serde(rename = "tierToColdAfterDaysSinceModificationGreaterThan")]
    pub r#tier_to_cold_after_days_since_modification_greater_than: Box<i32>,
    /// Optional The age in days after creation to cool storage. Supports blob currently at Hot tier.
    #[builder(into)]
    #[serde(rename = "tierToCoolAfterDaysSinceCreationGreaterThan")]
    pub r#tier_to_cool_after_days_since_creation_greater_than: Box<i32>,
    /// The age in days after last access time to tier blobs to cool storage. Supports blob currently at Hot tier.
    #[builder(into)]
    #[serde(rename = "tierToCoolAfterDaysSinceLastAccessTimeGreaterThan")]
    pub r#tier_to_cool_after_days_since_last_access_time_greater_than: Box<i32>,
    /// The age in days after last modification to tier blobs to cool storage. Supports blob currently at Hot tier.
    #[builder(into)]
    #[serde(rename = "tierToCoolAfterDaysSinceModificationGreaterThan")]
    pub r#tier_to_cool_after_days_since_modification_greater_than: Box<i32>,
}
