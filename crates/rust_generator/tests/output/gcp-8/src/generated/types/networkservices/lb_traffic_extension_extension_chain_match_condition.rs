#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct LbTrafficExtensionExtensionChainMatchCondition {
    /// A Common Expression Language (CEL) expression that is used to match requests for which the extension chain is executed.
    #[builder(into)]
    #[serde(rename = "celExpression")]
    pub r#cel_expression: Box<String>,
}
