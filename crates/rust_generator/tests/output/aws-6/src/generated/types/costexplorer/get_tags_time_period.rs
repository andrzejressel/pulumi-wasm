#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct GetTagsTimePeriod {
    /// Beginning of the time period.
    #[builder(into)]
    #[serde(rename = "end")]
    pub r#end: Box<String>,
    /// End of the time period.
    #[builder(into)]
    #[serde(rename = "start")]
    pub r#start: Box<String>,
}
