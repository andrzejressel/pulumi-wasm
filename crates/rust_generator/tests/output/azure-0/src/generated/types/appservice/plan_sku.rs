#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct PlanSku {
    /// Specifies the number of workers associated with this App Service Plan.
    #[builder(into, default)]
    #[serde(rename = "capacity")]
    pub r#capacity: Box<Option<i32>>,
    /// Specifies the plan's instance size.
    #[builder(into)]
    #[serde(rename = "size")]
    pub r#size: Box<String>,
    /// Specifies the plan's pricing tier.
    #[builder(into)]
    #[serde(rename = "tier")]
    pub r#tier: Box<String>,
}
