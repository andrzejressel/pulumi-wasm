#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct MlTransformParametersFindMatchesParameters {
    /// The value that is selected when tuning your transform for a balance between accuracy and cost.
    #[builder(into, default)]
    #[serde(rename = "accuracyCostTradeOff")]
    pub r#accuracy_cost_trade_off: Box<Option<f64>>,
    /// The value to switch on or off to force the output to match the provided labels from users.
    #[builder(into, default)]
    #[serde(rename = "enforceProvidedLabels")]
    pub r#enforce_provided_labels: Box<Option<bool>>,
    /// The value selected when tuning your transform for a balance between precision and recall.
    #[builder(into, default)]
    #[serde(rename = "precisionRecallTradeOff")]
    pub r#precision_recall_trade_off: Box<Option<f64>>,
    /// The name of a column that uniquely identifies rows in the source table.
    #[builder(into, default)]
    #[serde(rename = "primaryKeyColumnName")]
    pub r#primary_key_column_name: Box<Option<String>>,
}
