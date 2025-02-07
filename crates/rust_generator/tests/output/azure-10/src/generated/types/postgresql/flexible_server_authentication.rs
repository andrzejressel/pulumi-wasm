#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct FlexibleServerAuthentication {
    /// Whether or not Active Directory authentication is allowed to access the PostgreSQL Flexible Server. Defaults to `false`.
    #[builder(into, default)]
    #[serde(rename = "activeDirectoryAuthEnabled")]
    pub r#active_directory_auth_enabled: Box<Option<bool>>,
    /// Whether or not password authentication is allowed to access the PostgreSQL Flexible Server. Defaults to `true`.
    #[builder(into, default)]
    #[serde(rename = "passwordAuthEnabled")]
    pub r#password_auth_enabled: Box<Option<bool>>,
    /// The Tenant ID of the Azure Active Directory which is used by the Active Directory authentication. `active_directory_auth_enabled` must be set to `true`.
    /// 
    /// > **Note:** Setting `active_directory_auth_enabled` to `true` requires a Service Principal for the Postgres Flexible Server. For more details see [this document](https://learn.microsoft.com/en-us/azure/postgresql/flexible-server/how-to-configure-sign-in-azure-ad-authentication).
    /// 
    /// > **Note:** `tenant_id` is required when `active_directory_auth_enabled` is set to `true`. And it should not be specified when `active_directory_auth_enabled` is set to `false`
    #[builder(into, default)]
    #[serde(rename = "tenantId")]
    pub r#tenant_id: Box<Option<String>>,
}
