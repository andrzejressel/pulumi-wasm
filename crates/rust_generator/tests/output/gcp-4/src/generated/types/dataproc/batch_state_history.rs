#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct BatchStateHistory {
    /// (Output)
    /// The state of the batch at this point in history. For possible values, see the [API documentation](https://cloud.google.com/dataproc-serverless/docs/reference/rest/v1/projects.locations.batches#State).
    #[builder(into, default)]
    #[serde(rename = "state")]
    pub r#state: Box<Option<String>>,
    /// (Output)
    /// Details about the state at this point in history.
    #[builder(into, default)]
    #[serde(rename = "stateMessage")]
    pub r#state_message: Box<Option<String>>,
    /// (Output)
    /// The time when the batch entered the historical state.
    #[builder(into, default)]
    #[serde(rename = "stateStartTime")]
    pub r#state_start_time: Box<Option<String>>,
}
