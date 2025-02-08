#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GroupPolicyAssignmentResourceSelectorSelector {
    #[builder(into, default)]
    #[serde(rename = "ins")]
    pub r#ins: Box<Option<Vec<String>>>,
    /// Specifies which characteristic will narrow down the set of evaluated resources. Possible values are `resourceLocation`, `resourceType` and `resourceWithoutLocation`.
    #[builder(into)]
    #[serde(rename = "kind")]
    pub r#kind: Box<String>,
    #[builder(into, default)]
    #[serde(rename = "notIns")]
    pub r#not_ins: Box<Option<Vec<String>>>,
}
