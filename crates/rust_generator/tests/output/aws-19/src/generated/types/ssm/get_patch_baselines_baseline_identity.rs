#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetPatchBaselinesBaselineIdentity {
    /// Description of the patch baseline.
    #[builder(into)]
    #[serde(rename = "baselineDescription")]
    pub r#baseline_description: Box<String>,
    /// ID of the patch baseline.
    #[builder(into)]
    #[serde(rename = "baselineId")]
    pub r#baseline_id: Box<String>,
    /// Name of the patch baseline.
    #[builder(into)]
    #[serde(rename = "baselineName")]
    pub r#baseline_name: Box<String>,
    /// Indicates whether this is the default baseline. AWS Systems Manager supports creating multiple default patch baselines. For example, you can create a default patch baseline for each operating system.
    #[builder(into)]
    #[serde(rename = "defaultBaseline")]
    pub r#default_baseline: Box<bool>,
    /// Operating system the patch baseline applies to.
    #[builder(into)]
    #[serde(rename = "operatingSystem")]
    pub r#operating_system: Box<String>,
}
