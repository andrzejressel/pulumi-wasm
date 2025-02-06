#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetPoolStartTaskUserIdentityAutoUser {
    /// The elevation level of the user account. "NonAdmin" - The auto user is a standard user without elevated access. "Admin" - The auto user is a user with elevated access and operates with full Administrator permissions. The default value is nonAdmin.
    #[builder(into)]
    #[serde(rename = "elevationLevel")]
    pub r#elevation_level: Box<String>,
    /// The scope of the user identity under which the start task runs.
    #[builder(into)]
    #[serde(rename = "scope")]
    pub r#scope: Box<String>,
}
