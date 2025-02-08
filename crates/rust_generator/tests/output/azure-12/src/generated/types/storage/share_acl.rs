#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ShareAcl {
    /// An `access_policy` block as defined below.
    #[builder(into, default)]
    #[serde(rename = "accessPolicies")]
    pub r#access_policies: Box<Option<Vec<super::super::types::storage::ShareAclAccessPolicy>>>,
    /// The ID which should be used for this Shared Identifier.
    #[builder(into)]
    #[serde(rename = "id")]
    pub r#id: Box<String>,
}
