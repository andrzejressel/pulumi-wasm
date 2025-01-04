#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct LinuxFunctionAppAuthSettingsV2ActiveDirectoryV2 {
    /// The list of allowed Applications for the Default Authorisation Policy.
    #[builder(into, default)]
    #[serde(rename = "allowedApplications")]
    pub r#allowed_applications: Box<Option<Vec<String>>>,
    /// Specifies a list of Allowed audience values to consider when validating JWTs issued by Azure Active Directory.
    /// 
    /// > **NOTE:** This is configured on the Authentication Provider side and is Read Only here.
    #[builder(into, default)]
    #[serde(rename = "allowedAudiences")]
    pub r#allowed_audiences: Box<Option<Vec<String>>>,
    /// The list of allowed Group Names for the Default Authorisation Policy.
    #[builder(into, default)]
    #[serde(rename = "allowedGroups")]
    pub r#allowed_groups: Box<Option<Vec<String>>>,
    /// The list of allowed Identities for the Default Authorisation Policy.
    #[builder(into, default)]
    #[serde(rename = "allowedIdentities")]
    pub r#allowed_identities: Box<Option<Vec<String>>>,
    /// The ID of the Client to use to authenticate with Azure Active Directory.
    #[builder(into)]
    #[serde(rename = "clientId")]
    pub r#client_id: Box<String>,
    /// The thumbprint of the certificate used for signing purposes.
    #[builder(into, default)]
    #[serde(rename = "clientSecretCertificateThumbprint")]
    pub r#client_secret_certificate_thumbprint: Box<Option<String>>,
    /// The App Setting name that contains the client secret of the Client.
    /// 
    /// !> **NOTE:** A setting with this name must exist in `app_settings` to function correctly.
    #[builder(into, default)]
    #[serde(rename = "clientSecretSettingName")]
    pub r#client_secret_setting_name: Box<Option<String>>,
    /// A list of Allowed Client Applications in the JWT Claim.
    #[builder(into, default)]
    #[serde(rename = "jwtAllowedClientApplications")]
    pub r#jwt_allowed_client_applications: Box<Option<Vec<String>>>,
    /// A list of Allowed Groups in the JWT Claim.
    #[builder(into, default)]
    #[serde(rename = "jwtAllowedGroups")]
    pub r#jwt_allowed_groups: Box<Option<Vec<String>>>,
    /// A map of key-value pairs to send to the Authorisation Endpoint when a user logs in.
    #[builder(into, default)]
    #[serde(rename = "loginParameters")]
    pub r#login_parameters: Box<Option<std::collections::HashMap<String, String>>>,
    /// The Azure Tenant Endpoint for the Authenticating Tenant. e.g. `https://login.microsoftonline.com/{tenant-guid}/v2.0/`
    /// 
    /// > **NOTE:** [Here](https://learn.microsoft.com/en-us/entra/identity-platform/authentication-national-cloud#microsoft-entra-authentication-endpoints) is a list of possible authentication endpoints based on the cloud environment. [Here](https://learn.microsoft.com/en-us/azure/app-service/configure-authentication-provider-aad?tabs=workforce-tenant) is more information to better understand how to configure authentication for Azure App Service or Azure Functions.
    #[builder(into)]
    #[serde(rename = "tenantAuthEndpoint")]
    pub r#tenant_auth_endpoint: Box<String>,
    /// Should the www-authenticate provider should be omitted from the request? Defaults to `false`.
    #[builder(into, default)]
    #[serde(rename = "wwwAuthenticationDisabled")]
    pub r#www_authentication_disabled: Box<Option<bool>>,
}
