#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ServiceAuthenticationConfiguration {
    /// The intended audience to receive authentication tokens for the service. The default value is https://azurehealthcareapis.com
    #[builder(into, default)]
    #[serde(rename = "audience")]
    pub r#audience: Box<Option<String>>,
    /// The Azure Active Directory (tenant) that serves as the authentication authority to access the service. The default authority is the Directory defined in the authentication scheme in use when running this provider.
    /// Authority must be registered to Azure AD and in the following format: https://{Azure-AD-endpoint}/{tenant-id}.
    #[builder(into, default)]
    #[serde(rename = "authority")]
    pub r#authority: Box<Option<String>>,
    /// (Boolean) Enables the 'SMART on FHIR' option for mobile and web implementations.
    #[builder(into, default)]
    #[serde(rename = "smartProxyEnabled")]
    pub r#smart_proxy_enabled: Box<Option<bool>>,
}
