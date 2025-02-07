#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct AlertRuleAnomalyDuplicateSingleSelectObservation {
    /// The description of the single select observation.
    #[builder(into, default)]
    #[serde(rename = "description")]
    pub r#description: Box<Option<String>>,
    /// The name of the single select observation.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// A list of supported values of the single select observation.
    #[builder(into, default)]
    #[serde(rename = "supportedValues")]
    pub r#supported_values: Box<Option<Vec<String>>>,
    /// The value of the multi select observation.
    #[builder(into)]
    #[serde(rename = "value")]
    pub r#value: Box<String>,
}
