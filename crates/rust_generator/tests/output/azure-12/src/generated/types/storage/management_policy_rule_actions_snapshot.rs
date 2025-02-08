#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct ManagementPolicyRuleActionsSnapshot {
    /// The age in days after creation to tier blob snapshot to archive storage. Must be between `0` and `99999`. Defaults to `-1`.
    #[builder(into, default)]
    #[serde(rename = "changeTierToArchiveAfterDaysSinceCreation")]
    pub r#change_tier_to_archive_after_days_since_creation: Box<Option<i32>>,
    /// The age in days after creation to tier blob snapshot to cool storage. Must be between `0` and `99999`. Defaults to `-1`.
    #[builder(into, default)]
    #[serde(rename = "changeTierToCoolAfterDaysSinceCreation")]
    pub r#change_tier_to_cool_after_days_since_creation: Box<Option<i32>>,
    /// The age in days after creation to delete the blob snapshot. Must be between `0` and `99999`. Defaults to `-1`.
    #[builder(into, default)]
    #[serde(rename = "deleteAfterDaysSinceCreationGreaterThan")]
    pub r#delete_after_days_since_creation_greater_than: Box<Option<i32>>,
    /// The age in days after last tier change to the blobs to skip to be archved. Must be between `0` and `99999`. Defaults to `-1`.
    #[builder(into, default)]
    #[serde(rename = "tierToArchiveAfterDaysSinceLastTierChangeGreaterThan")]
    pub r#tier_to_archive_after_days_since_last_tier_change_greater_than: Box<Option<i32>>,
    /// The age in days after creation to cold storage. Supports blob currently at Hot tier. Must be between `0` and `99999`. Defaults to `-1`.
    #[builder(into, default)]
    #[serde(rename = "tierToColdAfterDaysSinceCreationGreaterThan")]
    pub r#tier_to_cold_after_days_since_creation_greater_than: Box<Option<i32>>,
}
