#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct CacheDirectoryLdapBind {
    /// The Bind Distinguished Name (DN) identity to be used in the secure LDAP connection.
    #[builder(into)]
    #[serde(rename = "dn")]
    pub r#dn: Box<String>,
    /// The Bind password to be used in the secure LDAP connection.
    #[builder(into)]
    #[serde(rename = "password")]
    pub r#password: Box<String>,
}
