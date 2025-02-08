#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetHoursOfOperationConfigEndTime {
    /// Hour of opening.
    #[builder(into)]
    #[serde(rename = "hours")]
    pub r#hours: Box<i32>,
    /// Minute of opening.
    #[builder(into)]
    #[serde(rename = "minutes")]
    pub r#minutes: Box<i32>,
}
