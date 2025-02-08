#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct OntapVolumeSnaplockConfigurationRetentionPeriodMaximumRetention {
    /// The type of time for the retention period of an FSx for ONTAP SnapLock volume. Set it to one of the valid types. If you set it to `INFINITE`, the files are retained forever. If you set it to `UNSPECIFIED`, the files are retained until you set an explicit retention period. Valid values: `SECONDS`, `MINUTES`, `HOURS`, `DAYS`, `MONTHS`, `YEARS`, `INFINITE`, `UNSPECIFIED`.
    #[builder(into, default)]
    #[serde(rename = "type")]
    pub r#type_: Box<Option<String>>,
    /// The amount of time for the autocommit period of a file in an FSx for ONTAP SnapLock volume.
    #[builder(into, default)]
    #[serde(rename = "value")]
    pub r#value: Box<Option<i32>>,
}
