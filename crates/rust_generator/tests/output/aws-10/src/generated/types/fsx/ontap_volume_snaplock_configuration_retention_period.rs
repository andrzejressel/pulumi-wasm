#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct OntapVolumeSnaplockConfigurationRetentionPeriod {
    /// The retention period assigned to a write once, read many (WORM) file by default if an explicit retention period is not set for an FSx for ONTAP SnapLock volume. The default retention period must be greater than or equal to the minimum retention period and less than or equal to the maximum retention period. See `default_retention` Block for details.
    #[builder(into, default)]
    #[serde(rename = "defaultRetention")]
    pub r#default_retention: Box<Option<super::super::types::fsx::OntapVolumeSnaplockConfigurationRetentionPeriodDefaultRetention>>,
    /// The longest retention period that can be assigned to a WORM file on an FSx for ONTAP SnapLock volume. See `maximum_retention` Block for details.
    #[builder(into, default)]
    #[serde(rename = "maximumRetention")]
    pub r#maximum_retention: Box<Option<super::super::types::fsx::OntapVolumeSnaplockConfigurationRetentionPeriodMaximumRetention>>,
    /// The shortest retention period that can be assigned to a WORM file on an FSx for ONTAP SnapLock volume. See `minimum_retention` Block for details.
    #[builder(into, default)]
    #[serde(rename = "minimumRetention")]
    pub r#minimum_retention: Box<Option<super::super::types::fsx::OntapVolumeSnaplockConfigurationRetentionPeriodMinimumRetention>>,
}
