#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct DomainMatching {
    /// A block that specifies the configuration about the auto-merging process. Documented below.
    #[builder(into, default)]
    #[serde(rename = "autoMerging")]
    pub r#auto_merging: Box<Option<super::super::types::customerprofiles::DomainMatchingAutoMerging>>,
    /// The flag that enables the matching process of duplicate profiles.
    #[builder(into)]
    #[serde(rename = "enabled")]
    pub r#enabled: Box<bool>,
    /// A block that specifies the configuration for exporting Identity Resolution results. Documented below.
    #[builder(into, default)]
    #[serde(rename = "exportingConfig")]
    pub r#exporting_config: Box<Option<super::super::types::customerprofiles::DomainMatchingExportingConfig>>,
    /// A block that specifies the day and time when you want to start the Identity Resolution Job every week. Documented below.
    #[builder(into, default)]
    #[serde(rename = "jobSchedule")]
    pub r#job_schedule: Box<Option<super::super::types::customerprofiles::DomainMatchingJobSchedule>>,
}
