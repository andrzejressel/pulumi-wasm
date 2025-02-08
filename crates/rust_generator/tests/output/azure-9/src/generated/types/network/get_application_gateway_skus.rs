#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetApplicationGatewaySkus {
    /// The Capacity of the SKU in use for this Application Gateway.
    #[builder(into)]
    #[serde(rename = "capacity")]
    pub r#capacity: Box<i32>,
    /// The name of this Application Gateway.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// The Tier of the SKU in use for this Application Gateway.
    #[builder(into)]
    #[serde(rename = "tier")]
    pub r#tier: Box<String>,
}
