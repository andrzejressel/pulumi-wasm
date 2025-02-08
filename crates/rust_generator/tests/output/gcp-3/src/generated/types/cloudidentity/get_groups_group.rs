#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct GetGroupsGroup {
    /// Additional group keys associated with the Group
    #[builder(into)]
    #[serde(rename = "additionalGroupKeys")]
    pub r#additional_group_keys: Box<Vec<super::super::types::cloudidentity::GetGroupsGroupAdditionalGroupKey>>,
    /// The time when the Group was created.
    #[builder(into)]
    #[serde(rename = "createTime")]
    pub r#create_time: Box<String>,
    /// An extended description to help users determine the purpose of a Group.
    #[builder(into)]
    #[serde(rename = "description")]
    pub r#description: Box<String>,
    /// The display name of the Group.
    #[builder(into)]
    #[serde(rename = "displayName")]
    pub r#display_name: Box<String>,
    /// EntityKey of the Group.  Structure is documented below.
    #[builder(into)]
    #[serde(rename = "groupKeys")]
    pub r#group_keys: Box<Vec<super::super::types::cloudidentity::GetGroupsGroupGroupKey>>,
    /// The initial configuration options for creating a Group.
    /// 
    /// See the
    /// [API reference](https://cloud.google.com/identity/docs/reference/rest/v1beta1/groups/create#initialgroupconfig)
    /// for possible values. Default value: "EMPTY" Possible values: ["INITIAL_GROUP_CONFIG_UNSPECIFIED", "WITH_INITIAL_OWNER", "EMPTY"]
    #[builder(into)]
    #[serde(rename = "initialGroupConfig")]
    pub r#initial_group_config: Box<String>,
    /// The labels that apply to the Group.
    /// Contains 'cloudidentity.googleapis.com/groups.discussion_forum': '' if the Group is a Google Group or
    /// 'system/groups/external': '' if the Group is an external-identity-mapped group.
    #[builder(into)]
    #[serde(rename = "labels")]
    pub r#labels: Box<std::collections::HashMap<String, String>>,
    /// Resource name of the Group in the format: groups/{group_id}, where `group_id` is the unique ID assigned to the Group.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// The parent resource under which to list all Groups. Must be of the form identitysources/{identity_source_id} for external- identity-mapped groups or customers/{customer_id} for Google Groups.
    #[builder(into)]
    #[serde(rename = "parent")]
    pub r#parent: Box<String>,
    /// The time when the Group was last updated.
    #[builder(into)]
    #[serde(rename = "updateTime")]
    pub r#update_time: Box<String>,
}
