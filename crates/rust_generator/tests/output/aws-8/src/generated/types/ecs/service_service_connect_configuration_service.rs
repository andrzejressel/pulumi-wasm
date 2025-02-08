#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ServiceServiceConnectConfigurationService {
    /// List of client aliases for this Service Connect service. You use these to assign names that can be used by client applications. The maximum number of client aliases that you can have in this list is 1. See below.
    #[builder(into, default)]
    #[serde(rename = "clientAlias")]
    pub r#client_alias: Box<Option<Vec<super::super::types::ecs::ServiceServiceConnectConfigurationServiceClientAlias>>>,
    /// Name of the new AWS Cloud Map service that Amazon ECS creates for this Amazon ECS service.
    #[builder(into, default)]
    #[serde(rename = "discoveryName")]
    pub r#discovery_name: Box<Option<String>>,
    /// Port number for the Service Connect proxy to listen on.
    #[builder(into, default)]
    #[serde(rename = "ingressPortOverride")]
    pub r#ingress_port_override: Box<Option<i32>>,
    /// Name of one of the `portMappings` from all the containers in the task definition of this Amazon ECS service.
    #[builder(into)]
    #[serde(rename = "portName")]
    pub r#port_name: Box<String>,
    /// Configuration timeouts for Service Connect
    #[builder(into, default)]
    #[serde(rename = "timeout")]
    pub r#timeout: Box<Option<super::super::types::ecs::ServiceServiceConnectConfigurationServiceTimeout>>,
    /// Configuration for enabling Transport Layer Security (TLS)
    #[builder(into, default)]
    #[serde(rename = "tls")]
    pub r#tls: Box<Option<super::super::types::ecs::ServiceServiceConnectConfigurationServiceTls>>,
}
