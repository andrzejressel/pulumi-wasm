#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetTaskExecutionPlacementStrategy {
    /// The field to apply the placement strategy against.
    #[builder(into, default)]
    #[serde(rename = "field")]
    pub r#field: Box<Option<String>>,
    /// The type of placement strategy. Valid values are `random`, `spread`, and `binpack`.
    /// 
    /// For more information, see the [Placement Strategy](https://docs.aws.amazon.com/AmazonECS/latest/APIReference/API_PlacementStrategy.html) documentation.
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: Box<String>,
}
