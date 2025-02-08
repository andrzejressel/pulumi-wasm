#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct PermissionsLfTagPolicy {
    /// Identifier for the Data Catalog. By default, it is the account ID of the caller.
    #[builder(into, default)]
    #[serde(rename = "catalogId")]
    pub r#catalog_id: Box<Option<String>>,
    /// A list of tag conditions that apply to the resource's tag policy. Configuration block for tag conditions that apply to the policy. See `expression` below.
    /// 
    /// The following argument is optional:
    #[builder(into)]
    #[serde(rename = "expressions")]
    pub r#expressions: Box<Vec<super::super::types::lakeformation::PermissionsLfTagPolicyExpression>>,
    /// The resource type for which the tag policy applies. Valid values are `DATABASE` and `TABLE`.
    #[builder(into)]
    #[serde(rename = "resourceType")]
    pub r#resource_type: Box<String>,
}
