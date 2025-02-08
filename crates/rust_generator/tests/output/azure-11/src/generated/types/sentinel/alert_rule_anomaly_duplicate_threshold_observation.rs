#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct AlertRuleAnomalyDuplicateThresholdObservation {
    /// The description of the threshold observation.
    #[builder(into, default)]
    #[serde(rename = "description")]
    pub r#description: Box<Option<String>>,
    /// The max value of the threshold observation.
    #[builder(into, default)]
    #[serde(rename = "max")]
    pub r#max: Box<Option<String>>,
    /// The min value of the threshold observation.
    #[builder(into, default)]
    #[serde(rename = "min")]
    pub r#min: Box<Option<String>>,
    /// The name of the threshold observation.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// The value of the threshold observation.
    #[builder(into)]
    #[serde(rename = "value")]
    pub r#value: Box<String>,
}
