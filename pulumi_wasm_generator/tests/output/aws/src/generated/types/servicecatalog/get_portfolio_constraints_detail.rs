#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetPortfolioConstraintsDetail {
    /// Identifier of the constraint.
    #[builder(into)]
    #[serde(rename = "constraintId")]
    pub r#constraint_id: Box<String>,
    /// Description of the constraint.
    #[builder(into)]
    #[serde(rename = "description")]
    pub r#description: Box<String>,
    #[builder(into)]
    #[serde(rename = "owner")]
    pub r#owner: Box<String>,
    /// Portfolio identifier.
    /// 
    /// The following arguments are optional:
    #[builder(into)]
    #[serde(rename = "portfolioId")]
    pub r#portfolio_id: Box<String>,
    /// Product identifier.
    #[builder(into)]
    #[serde(rename = "productId")]
    pub r#product_id: Box<String>,
    /// Type of constraint. Valid values are `LAUNCH`, `NOTIFICATION`, `STACKSET`, and `TEMPLATE`.
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: Box<String>,
}