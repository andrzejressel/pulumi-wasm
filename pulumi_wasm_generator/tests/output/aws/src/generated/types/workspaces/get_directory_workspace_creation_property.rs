#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetDirectoryWorkspaceCreationProperty {
    /// The identifier of your custom security group. Should relate to the same VPC, where workspaces reside in.
    #[builder(into)]
    #[serde(rename = "customSecurityGroupId")]
    pub r#custom_security_group_id: Box<String>,
    /// The default organizational unit (OU) for your WorkSpace directories.
    #[builder(into)]
    #[serde(rename = "defaultOu")]
    pub r#default_ou: Box<String>,
    /// Indicates whether internet access is enabled for your WorkSpaces.
    #[builder(into)]
    #[serde(rename = "enableInternetAccess")]
    pub r#enable_internet_access: Box<bool>,
    /// Indicates whether maintenance mode is enabled for your WorkSpaces. For more information, see [WorkSpace Maintenance](https://docs.aws.amazon.com/workspaces/latest/adminguide/workspace-maintenance.html).
    #[builder(into)]
    #[serde(rename = "enableMaintenanceMode")]
    pub r#enable_maintenance_mode: Box<bool>,
    /// Indicates whether users are local administrators of their WorkSpaces.
    #[builder(into)]
    #[serde(rename = "userEnabledAsLocalAdministrator")]
    pub r#user_enabled_as_local_administrator: Box<bool>,
}
