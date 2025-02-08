#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct RestoreTestingPlanRecoveryPointSelection {
    /// Specifies the algorithm used for selecting recovery points. Valid values are "RANDOM_WITHIN_WINDOW" and "LATEST_WITHIN_WINDOW".
    #[builder(into)]
    #[serde(rename = "algorithm")]
    pub r#algorithm: Box<String>,
    /// Specifies the backup vaults to exclude from the recovery point selection. Each value must be a valid AWS ARN for a backup vault or "*" to exclude all backup vaults.
    #[builder(into, default)]
    #[serde(rename = "excludeVaults")]
    pub r#exclude_vaults: Box<Option<Vec<String>>>,
    /// Specifies the backup vaults to include in the recovery point selection. Each value must be a valid AWS ARN for a backup vault or "*" to include all backup vaults.
    #[builder(into)]
    #[serde(rename = "includeVaults")]
    pub r#include_vaults: Box<Vec<String>>,
    /// Specifies the types of recovery points to include in the selection. Valid values are "CONTINUOUS" and "SNAPSHOT".
    #[builder(into)]
    #[serde(rename = "recoveryPointTypes")]
    pub r#recovery_point_types: Box<Vec<String>>,
    /// Specifies the number of days within which the recovery points should be selected. Must be a value between 1 and 365.
    #[builder(into, default)]
    #[serde(rename = "selectionWindowDays")]
    pub r#selection_window_days: Box<Option<i32>>,
}
