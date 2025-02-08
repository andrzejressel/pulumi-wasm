#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ScheduleTargetEcsParametersPlacementConstraint {
    /// A cluster query language expression to apply to the constraint. You cannot specify an expression if the constraint type is `distinctInstance`. For more information, see [Cluster query language](https://docs.aws.amazon.com/AmazonECS/latest/developerguide/cluster-query-language.html) in the Amazon ECS Developer Guide.
    #[builder(into, default)]
    #[serde(rename = "expression")]
    pub r#expression: Box<Option<String>>,
    /// The type of constraint. One of: `distinctInstance`, `memberOf`.
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: Box<String>,
}
