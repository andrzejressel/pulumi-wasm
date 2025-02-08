#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct GetApplicationGatewayWafConfigurationExclusion {
    /// Match variable of the exclusion rule.
    #[builder(into)]
    #[serde(rename = "matchVariable")]
    pub r#match_variable: Box<String>,
    /// String value which will be used for the filter operation.
    #[builder(into)]
    #[serde(rename = "selector")]
    pub r#selector: Box<String>,
    /// Operator which will be used to search in the variable content.
    #[builder(into)]
    #[serde(rename = "selectorMatchOperator")]
    pub r#selector_match_operator: Box<String>,
}
