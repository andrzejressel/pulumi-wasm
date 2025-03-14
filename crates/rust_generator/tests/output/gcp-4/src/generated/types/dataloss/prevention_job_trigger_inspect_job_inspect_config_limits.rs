#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct PreventionJobTriggerInspectJobInspectConfigLimits {
    /// Configuration of findings limit given for specified infoTypes.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "maxFindingsPerInfoTypes")]
    pub r#max_findings_per_info_types: Box<Option<Vec<super::super::types::dataloss::PreventionJobTriggerInspectJobInspectConfigLimitsMaxFindingsPerInfoType>>>,
    /// Max number of findings that will be returned for each item scanned. The maximum returned is 2000.
    #[builder(into, default)]
    #[serde(rename = "maxFindingsPerItem")]
    pub r#max_findings_per_item: Box<Option<i32>>,
    /// Max number of findings that will be returned per request/job. The maximum returned is 2000.
    #[builder(into, default)]
    #[serde(rename = "maxFindingsPerRequest")]
    pub r#max_findings_per_request: Box<Option<i32>>,
}
