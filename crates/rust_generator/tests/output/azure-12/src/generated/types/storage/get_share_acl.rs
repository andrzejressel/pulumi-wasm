#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct GetShareAcl {
    /// An `access_policy` block as defined below.
    #[builder(into)]
    #[serde(rename = "accessPolicies")]
    pub r#access_policies: Box<Vec<super::super::types::storage::GetShareAclAccessPolicy>>,
    /// The ID which should be used for this Shared Identifier.
    #[builder(into)]
    #[serde(rename = "id")]
    pub r#id: Box<String>,
}
