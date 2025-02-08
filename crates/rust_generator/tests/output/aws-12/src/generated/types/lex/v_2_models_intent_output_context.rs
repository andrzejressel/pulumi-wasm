#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct V2ModelsIntentOutputContext {
    /// Name of the output context.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// Amount of time, in seconds, that the output context should remain active. The time is figured from the first time the context is sent to the user.
    #[builder(into)]
    #[serde(rename = "timeToLiveInSeconds")]
    pub r#time_to_live_in_seconds: Box<i32>,
    /// Number of conversation turns that the output context should remain active. The number of turns is counted from the first time that the context is sent to the user.
    #[builder(into)]
    #[serde(rename = "turnsToLive")]
    pub r#turns_to_live: Box<i32>,
}
