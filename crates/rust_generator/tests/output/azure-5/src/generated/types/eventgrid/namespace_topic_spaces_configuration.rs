#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct NamespaceTopicSpacesConfiguration {
    /// Specifies a list of alternative sources for the client authentication name from the client certificate. Possible values are `ClientCertificateDns`, `ClientCertificateEmail`, `ClientCertificateIp`, `ClientCertificateSubject` and `ClientCertificateUri`.
    #[builder(into, default)]
    #[serde(rename = "alternativeAuthenticationNameSources")]
    pub r#alternative_authentication_name_sources: Box<Option<Vec<String>>>,
    /// One or more `dynamic_routing_enrichment` blocks as defined below.
    #[builder(into, default)]
    #[serde(rename = "dynamicRoutingEnrichments")]
    pub r#dynamic_routing_enrichments: Box<Option<Vec<super::super::types::eventgrid::NamespaceTopicSpacesConfigurationDynamicRoutingEnrichment>>>,
    /// Specifies the maximum number of client sessions per authentication name. Valid values can be between `1` and `100`.
    #[builder(into, default)]
    #[serde(rename = "maximumClientSessionsPerAuthenticationName")]
    pub r#maximum_client_sessions_per_authentication_name: Box<Option<i32>>,
    /// Specifies the maximum session expiry interval allowed for all MQTT clients connecting to the Event Grid namespace. Valid values can be between `1` and `8`.
    #[builder(into, default)]
    #[serde(rename = "maximumSessionExpiryInHours")]
    pub r#maximum_session_expiry_in_hours: Box<Option<i32>>,
    /// Specifies the Event Grid topic resource ID to route messages to.
    #[builder(into, default)]
    #[serde(rename = "routeTopicId")]
    pub r#route_topic_id: Box<Option<String>>,
    /// One or more `static_routing_enrichment` blocks as defined below.
    #[builder(into, default)]
    #[serde(rename = "staticRoutingEnrichments")]
    pub r#static_routing_enrichments: Box<Option<Vec<super::super::types::eventgrid::NamespaceTopicSpacesConfigurationStaticRoutingEnrichment>>>,
}
