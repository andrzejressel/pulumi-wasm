#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct ConnectorProfileConnectorProfileConfigConnectorProfilePropertiesSapoData {
    /// The location of the SAPOData resource.
    #[builder(into)]
    #[serde(rename = "applicationHostUrl")]
    pub r#application_host_url: Box<String>,
    /// The application path to catalog service.
    #[builder(into)]
    #[serde(rename = "applicationServicePath")]
    pub r#application_service_path: Box<String>,
    /// The client number for the client creating the connection.
    #[builder(into)]
    #[serde(rename = "clientNumber")]
    pub r#client_number: Box<String>,
    /// The logon language of SAPOData instance.
    #[builder(into, default)]
    #[serde(rename = "logonLanguage")]
    pub r#logon_language: Box<Option<String>>,
    /// The SAPOData OAuth properties required for OAuth type authentication.
    #[builder(into, default)]
    #[serde(rename = "oauthProperties")]
    pub r#oauth_properties: Box<Option<super::super::types::appflow::ConnectorProfileConnectorProfileConfigConnectorProfilePropertiesSapoDataOauthProperties>>,
    /// The port number of the SAPOData instance.
    #[builder(into)]
    #[serde(rename = "portNumber")]
    pub r#port_number: Box<i32>,
    #[builder(into, default)]
    #[serde(rename = "privateLinkServiceName")]
    pub r#private_link_service_name: Box<Option<String>>,
}
