#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct JobStatus {
    /// Optional job state details, such as an error description if the state is ERROR.
    #[builder(into, default)]
    #[serde(rename = "details")]
    pub r#details: Box<Option<String>>,
    /// A state message specifying the overall job state.
    #[builder(into, default)]
    #[serde(rename = "state")]
    pub r#state: Box<Option<String>>,
    /// The time when this state was entered.
    #[builder(into, default)]
    #[serde(rename = "stateStartTime")]
    pub r#state_start_time: Box<Option<String>>,
    /// Additional state information, which includes status reported by the agent.
    #[builder(into, default)]
    #[serde(rename = "substate")]
    pub r#substate: Box<Option<String>>,
}
