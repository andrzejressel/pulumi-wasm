#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct DirectorySamlProperties {
    /// The relay state parameter name supported by the SAML 2.0 identity provider (IdP). Default `RelayState`.
    #[builder(into, default)]
    #[serde(rename = "relayStateParameterName")]
    pub r#relay_state_parameter_name: Box<Option<String>>,
    /// Status of SAML 2.0 authentication. Default `DISABLED`.
    #[builder(into, default)]
    #[serde(rename = "status")]
    pub r#status: Box<Option<String>>,
    /// The SAML 2.0 identity provider (IdP) user access URL.
    #[builder(into, default)]
    #[serde(rename = "userAccessUrl")]
    pub r#user_access_url: Box<Option<String>>,
}
