#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct GetBrokerLdapServerMetadata {
    #[builder(into)]
    #[serde(rename = "hosts")]
    pub r#hosts: Box<Vec<String>>,
    #[builder(into)]
    #[serde(rename = "roleBase")]
    pub r#role_base: Box<String>,
    #[builder(into)]
    #[serde(rename = "roleName")]
    pub r#role_name: Box<String>,
    #[builder(into)]
    #[serde(rename = "roleSearchMatching")]
    pub r#role_search_matching: Box<String>,
    #[builder(into)]
    #[serde(rename = "roleSearchSubtree")]
    pub r#role_search_subtree: Box<bool>,
    #[builder(into)]
    #[serde(rename = "serviceAccountPassword")]
    pub r#service_account_password: Box<String>,
    #[builder(into)]
    #[serde(rename = "serviceAccountUsername")]
    pub r#service_account_username: Box<String>,
    #[builder(into)]
    #[serde(rename = "userBase")]
    pub r#user_base: Box<String>,
    #[builder(into)]
    #[serde(rename = "userRoleName")]
    pub r#user_role_name: Box<String>,
    #[builder(into)]
    #[serde(rename = "userSearchMatching")]
    pub r#user_search_matching: Box<String>,
    #[builder(into)]
    #[serde(rename = "userSearchSubtree")]
    pub r#user_search_subtree: Box<bool>,
}
