#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct PoolUserAccount {
    /// The elevation level of the user account. "NonAdmin" - The auto user is a standard user without elevated access. "Admin" - The auto user is a user with elevated access and operates with full Administrator permissions. The default value is nonAdmin.
    #[builder(into)]
    #[serde(rename = "elevationLevel")]
    pub r#elevation_level: Box<String>,
    /// The `linux_user_configuration` block defined below is a linux-specific user configuration for the user account. This property is ignored if specified on a Windows pool. If not specified, the user is created with the default options.
    #[builder(into, default)]
    #[serde(rename = "linuxUserConfigurations")]
    pub r#linux_user_configurations: Box<Option<Vec<super::super::types::batch::PoolUserAccountLinuxUserConfiguration>>>,
    /// The name of the user account.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// The password for the user account.
    #[builder(into)]
    #[serde(rename = "password")]
    pub r#password: Box<String>,
    /// The `windows_user_configuration` block defined below is a windows-specific user configuration for the user account. This property can only be specified if the user is on a Windows pool. If not specified and on a Windows pool, the user is created with the default options.
    #[builder(into, default)]
    #[serde(rename = "windowsUserConfigurations")]
    pub r#windows_user_configurations: Box<Option<Vec<super::super::types::batch::PoolUserAccountWindowsUserConfiguration>>>,
}
