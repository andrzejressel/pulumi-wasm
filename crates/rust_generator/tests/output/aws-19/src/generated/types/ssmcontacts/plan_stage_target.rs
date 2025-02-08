#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct PlanStageTarget {
    /// A configuration block for specifying information about the contact channel that Incident Manager engages. See Channel Target Info for more details.
    #[builder(into, default)]
    #[serde(rename = "channelTargetInfo")]
    pub r#channel_target_info: Box<Option<super::super::types::ssmcontacts::PlanStageTargetChannelTargetInfo>>,
    /// A configuration block for specifying information about the contact that Incident Manager engages. See Contact Target Info for more details.
    #[builder(into, default)]
    #[serde(rename = "contactTargetInfo")]
    pub r#contact_target_info: Box<Option<super::super::types::ssmcontacts::PlanStageTargetContactTargetInfo>>,
}
