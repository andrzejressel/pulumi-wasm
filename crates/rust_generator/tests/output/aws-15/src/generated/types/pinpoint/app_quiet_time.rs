#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct AppQuietTime {
    /// The default end time for quiet time in ISO 8601 format. Required if `start` is set
    #[builder(into, default)]
    #[serde(rename = "end")]
    pub r#end: Box<Option<String>>,
    /// The default start time for quiet time in ISO 8601 format. Required if `end` is set
    #[builder(into, default)]
    #[serde(rename = "start")]
    pub r#start: Box<Option<String>>,
}
