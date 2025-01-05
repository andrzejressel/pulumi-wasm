#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetPermissionsLfTagPolicy {
    /// Identifier for the Data Catalog. By default, it is the account ID of the caller.
    #[builder(into)]
    #[serde(rename = "catalogId")]
    pub r#catalog_id: Box<String>,
    /// List of tag conditions that apply to the resource's tag policy. Configuration block for tag conditions that apply to the policy. See `expression` below.
    /// 
    /// The following argument is optional:
    #[builder(into)]
    #[serde(rename = "expressions")]
    pub r#expressions: Box<Vec<super::super::types::lakeformation::GetPermissionsLfTagPolicyExpression>>,
    /// Resource type for which the tag policy applies. Valid values are `DATABASE` and `TABLE`.
    #[builder(into)]
    #[serde(rename = "resourceType")]
    pub r#resource_type: Box<String>,
}
