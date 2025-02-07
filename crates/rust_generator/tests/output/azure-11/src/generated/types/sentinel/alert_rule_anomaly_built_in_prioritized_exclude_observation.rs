#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct AlertRuleAnomalyBuiltInPrioritizedExcludeObservation {
    /// The description of the threshold observation.
    #[builder(into, default)]
    #[serde(rename = "description")]
    pub r#description: Box<Option<String>>,
    /// The excluded value per `description`.
    #[builder(into, default)]
    #[serde(rename = "exclude")]
    pub r#exclude: Box<Option<String>>,
    /// The Name of the built-in Anomaly Alert Rule.
    #[builder(into, default)]
    #[serde(rename = "name")]
    pub r#name: Box<Option<String>>,
    /// The prioritized value per `description`.
    #[builder(into, default)]
    #[serde(rename = "prioritize")]
    pub r#prioritize: Box<Option<String>>,
}
