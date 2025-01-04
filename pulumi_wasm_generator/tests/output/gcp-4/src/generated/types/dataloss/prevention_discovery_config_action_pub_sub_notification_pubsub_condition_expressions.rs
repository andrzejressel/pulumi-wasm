#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct PreventionDiscoveryConfigActionPubSubNotificationPubsubConditionExpressions {
    /// Conditions to apply to the expression
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "conditions")]
    pub r#conditions: Box<Option<Vec<super::super::types::dataloss::PreventionDiscoveryConfigActionPubSubNotificationPubsubConditionExpressionsCondition>>>,
    /// The operator to apply to the collection of conditions
    /// Possible values are: `OR`, `AND`.
    #[builder(into, default)]
    #[serde(rename = "logicalOperator")]
    pub r#logical_operator: Box<Option<String>>,
}
