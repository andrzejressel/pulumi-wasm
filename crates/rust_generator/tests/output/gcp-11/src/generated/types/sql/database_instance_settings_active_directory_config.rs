#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct DatabaseInstanceSettingsActiveDirectoryConfig {
    /// The domain name for the active directory (e.g., mydomain.com).
    /// Can only be used with SQL Server.
    #[builder(into)]
    #[serde(rename = "domain")]
    pub r#domain: Box<String>,
}
