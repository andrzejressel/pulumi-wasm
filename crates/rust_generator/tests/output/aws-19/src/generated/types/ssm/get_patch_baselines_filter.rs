#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetPatchBaselinesFilter {
    /// Filter key. See the [AWS SSM documentation](https://docs.aws.amazon.com/systems-manager/latest/APIReference/API_DescribePatchBaselines.html) for valid values.
    #[builder(into)]
    #[serde(rename = "key")]
    pub r#key: Box<String>,
    /// Filter values. See the [AWS SSM documentation](https://docs.aws.amazon.com/systems-manager/latest/APIReference/API_DescribePatchBaselines.html) for example values.
    #[builder(into)]
    #[serde(rename = "values")]
    pub r#values: Box<Vec<String>>,
}
