#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct AuthzPolicyHttpRuleFromSourceResource {
    /// An IAM service account to match against the source service account of the VM sending the request.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "iamServiceAccount")]
    pub r#iam_service_account: Box<Option<super::super::types::networksecurity::AuthzPolicyHttpRuleFromSourceResourceIamServiceAccount>>,
    /// A list of resource tag value permanent IDs to match against the resource manager tags value associated with the source VM of a request.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "tagValueIdSet")]
    pub r#tag_value_id_set: Box<Option<super::super::types::networksecurity::AuthzPolicyHttpRuleFromSourceResourceTagValueIdSet>>,
}
