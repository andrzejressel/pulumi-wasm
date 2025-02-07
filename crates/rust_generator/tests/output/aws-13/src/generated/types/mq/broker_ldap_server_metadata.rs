#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct BrokerLdapServerMetadata {
    /// List of a fully qualified domain name of the LDAP server and an optional failover server.
    #[builder(into, default)]
    #[serde(rename = "hosts")]
    pub r#hosts: Box<Option<Vec<String>>>,
    /// Fully qualified name of the directory to search for a user’s groups.
    #[builder(into, default)]
    #[serde(rename = "roleBase")]
    pub r#role_base: Box<Option<String>>,
    /// Specifies the LDAP attribute that identifies the group name attribute in the object returned from the group membership query.
    #[builder(into, default)]
    #[serde(rename = "roleName")]
    pub r#role_name: Box<Option<String>>,
    /// Search criteria for groups.
    #[builder(into, default)]
    #[serde(rename = "roleSearchMatching")]
    pub r#role_search_matching: Box<Option<String>>,
    /// Whether the directory search scope is the entire sub-tree.
    #[builder(into, default)]
    #[serde(rename = "roleSearchSubtree")]
    pub r#role_search_subtree: Box<Option<bool>>,
    /// Service account password.
    #[builder(into, default)]
    #[serde(rename = "serviceAccountPassword")]
    pub r#service_account_password: Box<Option<String>>,
    /// Service account username.
    #[builder(into, default)]
    #[serde(rename = "serviceAccountUsername")]
    pub r#service_account_username: Box<Option<String>>,
    /// Fully qualified name of the directory where you want to search for users.
    #[builder(into, default)]
    #[serde(rename = "userBase")]
    pub r#user_base: Box<Option<String>>,
    /// Specifies the name of the LDAP attribute for the user group membership.
    #[builder(into, default)]
    #[serde(rename = "userRoleName")]
    pub r#user_role_name: Box<Option<String>>,
    /// Search criteria for users.
    #[builder(into, default)]
    #[serde(rename = "userSearchMatching")]
    pub r#user_search_matching: Box<Option<String>>,
    /// Whether the directory search scope is the entire sub-tree.
    #[builder(into, default)]
    #[serde(rename = "userSearchSubtree")]
    pub r#user_search_subtree: Box<Option<bool>>,
}
