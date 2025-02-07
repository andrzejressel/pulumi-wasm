#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct AppIngress {
    /// Should this ingress allow insecure connections?
    #[builder(into, default)]
    #[serde(rename = "allowInsecureConnections")]
    pub r#allow_insecure_connections: Box<Option<bool>>,
    /// One or more `custom_domain` block as detailed below.
    #[builder(into, default)]
    #[serde(rename = "customDomains")]
    pub r#custom_domains: Box<Option<Vec<super::super::types::containerapp::AppIngressCustomDomain>>>,
    /// The exposed port on the container for the Ingress traffic.
    /// 
    /// > **Note:** `exposed_port` can only be specified when `transport` is set to `tcp`.
    #[builder(into, default)]
    #[serde(rename = "exposedPort")]
    pub r#exposed_port: Box<Option<i32>>,
    /// Are connections to this Ingress from outside the Container App Environment enabled? Defaults to `false`.
    #[builder(into, default)]
    #[serde(rename = "externalEnabled")]
    pub r#external_enabled: Box<Option<bool>>,
    /// The FQDN of the ingress.
    #[builder(into, default)]
    #[serde(rename = "fqdn")]
    pub r#fqdn: Box<Option<String>>,
    /// One or more `ip_security_restriction` blocks for IP-filtering rules as defined below.
    #[builder(into, default)]
    #[serde(rename = "ipSecurityRestrictions")]
    pub r#ip_security_restrictions: Box<Option<Vec<super::super::types::containerapp::AppIngressIpSecurityRestriction>>>,
    /// The target port on the container for the Ingress traffic.
    #[builder(into)]
    #[serde(rename = "targetPort")]
    pub r#target_port: Box<i32>,
    /// One or more `traffic_weight` blocks as detailed below.
    #[builder(into)]
    #[serde(rename = "trafficWeights")]
    pub r#traffic_weights: Box<Vec<super::super::types::containerapp::AppIngressTrafficWeight>>,
    /// The transport method for the Ingress. Possible values are `auto`, `http`, `http2` and `tcp`. Defaults to `auto`.
    /// 
    /// > **Note:**  if `transport` is set to `tcp`, `exposed_port` and `target_port` should be set at the same time.
    #[builder(into, default)]
    #[serde(rename = "transport")]
    pub r#transport: Box<Option<String>>,
}
