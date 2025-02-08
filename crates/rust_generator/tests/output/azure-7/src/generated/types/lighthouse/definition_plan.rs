#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct DefinitionPlan {
    /// The plan name of the marketplace offer.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// The product code of the plan.
    #[builder(into)]
    #[serde(rename = "product")]
    pub r#product: Box<String>,
    /// The publisher ID of the plan.
    #[builder(into)]
    #[serde(rename = "publisher")]
    pub r#publisher: Box<String>,
    /// The version of the plan.
    #[builder(into)]
    #[serde(rename = "version")]
    pub r#version: Box<String>,
}
