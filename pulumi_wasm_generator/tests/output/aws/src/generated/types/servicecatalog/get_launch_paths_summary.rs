#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetLaunchPathsSummary {
    /// Block for constraints on the portfolio-product relationship. See details below.
    #[builder(into)]
    #[serde(rename = "constraintSummaries")]
    pub r#constraint_summaries: Box<Vec<super::super::types::servicecatalog::GetLaunchPathsSummaryConstraintSummary>>,
    /// Name of the portfolio to which the path was assigned.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// Identifier of the product path.
    #[builder(into)]
    #[serde(rename = "pathId")]
    pub r#path_id: Box<String>,
    /// Tags associated with this product path.
    #[builder(into)]
    #[serde(rename = "tags")]
    pub r#tags: Box<std::collections::HashMap<String, String>>,
}
