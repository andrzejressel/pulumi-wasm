#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetTableAcl {
    #[builder(into)]
    #[serde(rename = "accessPolicies")]
    pub r#access_policies: Box<Vec<super::super::types::storage::GetTableAclAccessPolicy>>,
    /// The ID of the Storage Table.
    #[builder(into)]
    #[serde(rename = "id")]
    pub r#id: Box<String>,
}
