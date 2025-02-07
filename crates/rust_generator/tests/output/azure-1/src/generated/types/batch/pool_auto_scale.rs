#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct PoolAutoScale {
    /// The interval to wait before evaluating if the pool needs to be scaled. Defaults to `PT15M`.
    #[builder(into, default)]
    #[serde(rename = "evaluationInterval")]
    pub r#evaluation_interval: Box<Option<String>>,
    /// The autoscale formula that needs to be used for scaling the Batch pool.
    #[builder(into)]
    #[serde(rename = "formula")]
    pub r#formula: Box<String>,
}
