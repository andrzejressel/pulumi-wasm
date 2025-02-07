#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetSkus {
    /// The name of this Elastic SAN.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// The SKU tier.
    #[builder(into)]
    #[serde(rename = "tier")]
    pub r#tier: Box<String>,
}
