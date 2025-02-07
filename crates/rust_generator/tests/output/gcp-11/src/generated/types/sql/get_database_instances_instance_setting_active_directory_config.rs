#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetDatabaseInstancesInstanceSettingActiveDirectoryConfig {
    /// Domain name of the Active Directory for SQL Server (e.g., mydomain.com).
    #[builder(into)]
    #[serde(rename = "domain")]
    pub r#domain: Box<String>,
}
