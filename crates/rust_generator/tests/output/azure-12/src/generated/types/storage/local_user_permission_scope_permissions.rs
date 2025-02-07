#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct LocalUserPermissionScopePermissions {
    /// Specifies if the Local User has the create permission for this scope. Defaults to `false`.
    #[builder(into, default)]
    #[serde(rename = "create")]
    pub r#create: Box<Option<bool>>,
    /// Specifies if the Local User has the delete permission for this scope. Defaults to `false`.
    #[builder(into, default)]
    #[serde(rename = "delete")]
    pub r#delete: Box<Option<bool>>,
    /// Specifies if the Local User has the list permission for this scope. Defaults to `false`.
    #[builder(into, default)]
    #[serde(rename = "list")]
    pub r#list: Box<Option<bool>>,
    /// Specifies if the Local User has the read permission for this scope. Defaults to `false`.
    #[builder(into, default)]
    #[serde(rename = "read")]
    pub r#read: Box<Option<bool>>,
    /// Specifies if the Local User has the write permission for this scope. Defaults to `false`.
    #[builder(into, default)]
    #[serde(rename = "write")]
    pub r#write: Box<Option<bool>>,
}
