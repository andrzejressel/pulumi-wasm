#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetAppIngress {
    /// Should this ingress allow insecure connections?
    #[builder(into)]
    #[serde(rename = "allowInsecureConnections")]
    pub r#allow_insecure_connections: Box<bool>,
    /// One or more `custom_domain` block as detailed below.
    #[builder(into)]
    #[serde(rename = "customDomains")]
    pub r#custom_domains: Box<Vec<super::super::types::containerapp::GetAppIngressCustomDomain>>,
    /// The exposed port on the container for the Ingress traffic.
    #[builder(into)]
    #[serde(rename = "exposedPort")]
    pub r#exposed_port: Box<i32>,
    /// Is this an external Ingress.
    #[builder(into)]
    #[serde(rename = "externalEnabled")]
    pub r#external_enabled: Box<bool>,
    /// The FQDN of the ingress.
    #[builder(into)]
    #[serde(rename = "fqdn")]
    pub r#fqdn: Box<String>,
    /// One or more `ip_security_restriction` blocks for IP-filtering rules as defined below.
    #[builder(into)]
    #[serde(rename = "ipSecurityRestrictions")]
    pub r#ip_security_restrictions: Box<Vec<super::super::types::containerapp::GetAppIngressIpSecurityRestriction>>,
    /// The target port on the container for the Ingress traffic.
    #[builder(into)]
    #[serde(rename = "targetPort")]
    pub r#target_port: Box<i32>,
    /// A `traffic_weight` block as detailed below.
    #[builder(into)]
    #[serde(rename = "trafficWeights")]
    pub r#traffic_weights: Box<Vec<super::super::types::containerapp::GetAppIngressTrafficWeight>>,
    /// The transport method for the Ingress. Possible values include `auto`, `http`, and `http2`. Defaults to `auto`
    #[builder(into)]
    #[serde(rename = "transport")]
    pub r#transport: Box<String>,
}
