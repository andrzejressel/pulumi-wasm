#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ScheduleTargetEventbridgeParameters {
    /// Free-form string used to decide what fields to expect in the event detail. Up to 128 characters.
    #[builder(into)]
    #[serde(rename = "detailType")]
    pub r#detail_type: Box<String>,
    /// Source of the event.
    #[builder(into)]
    #[serde(rename = "source")]
    pub r#source: Box<String>,
}
