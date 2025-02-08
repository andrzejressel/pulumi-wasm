#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct PlanStage {
    /// The time to wait until beginning the next stage. The duration can only be set to 0 if a target is specified.
    #[builder(into)]
    #[serde(rename = "durationInMinutes")]
    pub r#duration_in_minutes: Box<i32>,
    /// One or more configuration blocks for specifying the contacts or contact methods that the escalation plan or engagement plan is engaging. See Target below for more details.
    #[builder(into, default)]
    #[serde(rename = "targets")]
    pub r#targets: Box<Option<Vec<super::super::types::ssmcontacts::PlanStageTarget>>>,
}
