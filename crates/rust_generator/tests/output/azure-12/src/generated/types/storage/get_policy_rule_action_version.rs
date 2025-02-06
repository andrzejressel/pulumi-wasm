#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetPolicyRuleActionVersion {
    /// The age in days after creation to tier blob version to archive storage.
    #[builder(into)]
    #[serde(rename = "changeTierToArchiveAfterDaysSinceCreation")]
    pub r#change_tier_to_archive_after_days_since_creation: Box<i32>,
    /// The age in days after creation to tier blob version to cool storage.
    #[builder(into)]
    #[serde(rename = "changeTierToCoolAfterDaysSinceCreation")]
    pub r#change_tier_to_cool_after_days_since_creation: Box<i32>,
    /// The age in days after creation to delete the blob version.
    #[builder(into)]
    #[serde(rename = "deleteAfterDaysSinceCreation")]
    pub r#delete_after_days_since_creation: Box<i32>,
    /// The age in days after last tier change to the blobs to skip to be archived.
    #[builder(into)]
    #[serde(rename = "tierToArchiveAfterDaysSinceLastTierChangeGreaterThan")]
    pub r#tier_to_archive_after_days_since_last_tier_change_greater_than: Box<i32>,
    /// Optional The age in days after creation to cold storage. Supports blob currently at Hot tier.
    #[builder(into)]
    #[serde(rename = "tierToColdAfterDaysSinceCreationGreaterThan")]
    pub r#tier_to_cold_after_days_since_creation_greater_than: Box<i32>,
}
