#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetGroupsGroup {
    /// Description of the specified group.
    #[builder(into)]
    #[serde(rename = "description")]
    pub r#description: Box<String>,
    /// Group's display name.
    #[builder(into)]
    #[serde(rename = "displayName")]
    pub r#display_name: Box<String>,
    /// List of identifiers issued to this resource by an external identity provider.
    #[builder(into)]
    #[serde(rename = "externalIds")]
    pub r#external_ids: Box<Vec<super::super::types::identitystore::GetGroupsGroupExternalId>>,
    /// Identifier of the group in the Identity Store.
    #[builder(into)]
    #[serde(rename = "groupId")]
    pub r#group_id: Box<String>,
    /// Identity Store ID associated with the Single Sign-On (SSO) Instance.
    #[builder(into)]
    #[serde(rename = "identityStoreId")]
    pub r#identity_store_id: Box<String>,
}
