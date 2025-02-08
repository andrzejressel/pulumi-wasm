#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct PreventionJobTriggerInspectJobInspectConfigLimitsMaxFindingsPerInfoType {
    /// Type of information the findings limit applies to. Only one limit per infoType should be provided. If InfoTypeLimit does
    /// not have an infoType, the DLP API applies the limit against all infoTypes that are found but not
    /// specified in another InfoTypeLimit.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "infoType")]
    pub r#info_type: Box<Option<super::super::types::dataloss::PreventionJobTriggerInspectJobInspectConfigLimitsMaxFindingsPerInfoTypeInfoType>>,
    /// Max findings limit for the given infoType.
    #[builder(into, default)]
    #[serde(rename = "maxFindings")]
    pub r#max_findings: Box<Option<i32>>,
}
