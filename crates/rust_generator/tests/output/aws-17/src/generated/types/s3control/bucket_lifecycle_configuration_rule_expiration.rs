#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct BucketLifecycleConfigurationRuleExpiration {
    /// Date the object is to be deleted. Should be in `YYYY-MM-DD` date format, e.g., `2020-09-30`.
    #[builder(into, default)]
    #[serde(rename = "date")]
    pub r#date: Box<Option<String>>,
    /// Number of days before the object is to be deleted.
    #[builder(into, default)]
    #[serde(rename = "days")]
    pub r#days: Box<Option<i32>>,
    /// Enable to remove a delete marker with no noncurrent versions. Cannot be specified with `date` or `days`.
    #[builder(into, default)]
    #[serde(rename = "expiredObjectDeleteMarker")]
    pub r#expired_object_delete_marker: Box<Option<bool>>,
}
