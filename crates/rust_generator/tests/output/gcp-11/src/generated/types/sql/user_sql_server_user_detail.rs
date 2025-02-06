#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct UserSqlServerUserDetail {
    /// If the user has been disabled.
    #[builder(into, default)]
    #[serde(rename = "disabled")]
    pub r#disabled: Box<Option<bool>>,
    /// The server roles for this user in the database.
    #[builder(into, default)]
    #[serde(rename = "serverRoles")]
    pub r#server_roles: Box<Option<Vec<String>>>,
}
