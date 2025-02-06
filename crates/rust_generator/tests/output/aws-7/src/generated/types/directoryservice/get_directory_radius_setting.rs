#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetDirectoryRadiusSetting {
    /// The protocol specified for your RADIUS endpoints.
    #[builder(into)]
    #[serde(rename = "authenticationProtocol")]
    pub r#authentication_protocol: Box<String>,
    /// Display label.
    #[builder(into)]
    #[serde(rename = "displayLabel")]
    pub r#display_label: Box<String>,
    /// Port that your RADIUS server is using for communications.
    #[builder(into)]
    #[serde(rename = "radiusPort")]
    pub r#radius_port: Box<i32>,
    /// Maximum number of times that communication with the RADIUS server is attempted.
    #[builder(into)]
    #[serde(rename = "radiusRetries")]
    pub r#radius_retries: Box<i32>,
    /// Set of strings that contains the fully qualified domain name (FQDN) or IP addresses of the RADIUS server endpoints, or the FQDN or IP addresses of your RADIUS server load balancer.
    #[builder(into)]
    #[serde(rename = "radiusServers")]
    pub r#radius_servers: Box<Vec<String>>,
    /// Amount of time, in seconds, to wait for the RADIUS server to respond.
    #[builder(into)]
    #[serde(rename = "radiusTimeout")]
    pub r#radius_timeout: Box<i32>,
    /// Not currently used.
    #[builder(into)]
    #[serde(rename = "useSameUsername")]
    pub r#use_same_username: Box<bool>,
}
