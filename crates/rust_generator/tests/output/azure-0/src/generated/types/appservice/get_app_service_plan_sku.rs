#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetAppServicePlanSku {
    /// Specifies the number of workers associated with this App Service Plan.
    #[builder(into)]
    #[serde(rename = "capacity")]
    pub r#capacity: Box<i32>,
    /// Specifies the plan's instance size.
    #[builder(into)]
    #[serde(rename = "size")]
    pub r#size: Box<String>,
    /// Specifies the plan's pricing tier.
    #[builder(into)]
    #[serde(rename = "tier")]
    pub r#tier: Box<String>,
}
