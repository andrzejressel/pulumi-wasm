#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct GetServiceAuthenticationConfiguration {
    /// The intended audience to receive authentication tokens for the service.
    #[builder(into)]
    #[serde(rename = "audience")]
    pub r#audience: Box<String>,
    /// The Azure Active Directory (tenant) that serves as the authentication authority to access the service.
    #[builder(into)]
    #[serde(rename = "authority")]
    pub r#authority: Box<String>,
    /// Is the 'SMART on FHIR' option for mobile and web implementations enabled?
    #[builder(into)]
    #[serde(rename = "smartProxyEnabled")]
    pub r#smart_proxy_enabled: Box<bool>,
}
