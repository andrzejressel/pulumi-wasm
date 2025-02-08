#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ServerAzureadAdministrator {
    /// Specifies whether only AD Users and administrators (e.g. `azuread_administrator[0].login_username`) can be used to login, or also local database users (e.g. `administrator_login`). When `true`, the `administrator_login` and `administrator_login_password` properties can be omitted.
    #[builder(into, default)]
    #[serde(rename = "azureadAuthenticationOnly")]
    pub r#azuread_authentication_only: Box<Option<bool>>,
    /// The login username of the Azure AD Administrator of this SQL Server.
    #[builder(into)]
    #[serde(rename = "loginUsername")]
    pub r#login_username: Box<String>,
    /// The object id of the Azure AD Administrator of this SQL Server.
    #[builder(into)]
    #[serde(rename = "objectId")]
    pub r#object_id: Box<String>,
    /// The tenant id of the Azure AD Administrator of this SQL Server.
    #[builder(into, default)]
    #[serde(rename = "tenantId")]
    pub r#tenant_id: Box<Option<String>>,
}
