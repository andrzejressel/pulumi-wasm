#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetApplicationGatewayBackendHttpSetting {
    /// The name of the affinity cookie.
    #[builder(into)]
    #[serde(rename = "affinityCookieName")]
    pub r#affinity_cookie_name: Box<String>,
    /// One or more `authentication_certificate` blocks as defined below.
    #[builder(into)]
    #[serde(rename = "authenticationCertificates")]
    pub r#authentication_certificates: Box<Vec<super::super::types::network::GetApplicationGatewayBackendHttpSettingAuthenticationCertificate>>,
    /// A `connection_draining` block as defined below.
    #[builder(into)]
    #[serde(rename = "connectionDrainings")]
    pub r#connection_drainings: Box<Vec<super::super::types::network::GetApplicationGatewayBackendHttpSettingConnectionDraining>>,
    /// Is Cookie-Based Affinity enabled?
    #[builder(into)]
    #[serde(rename = "cookieBasedAffinity")]
    pub r#cookie_based_affinity: Box<String>,
    /// The Hostname which is used for this HTTP Listener.
    #[builder(into)]
    #[serde(rename = "hostName")]
    pub r#host_name: Box<String>,
    /// The ID of the Rewrite Rule Set
    #[builder(into)]
    #[serde(rename = "id")]
    pub r#id: Box<String>,
    /// The name of this Application Gateway.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// The URL path to rewrite.
    #[builder(into)]
    #[serde(rename = "path")]
    pub r#path: Box<String>,
    /// Whether host header will be picked from the host name of the backend server.
    #[builder(into)]
    #[serde(rename = "pickHostNameFromBackendAddress")]
    pub r#pick_host_name_from_backend_address: Box<bool>,
    /// Custom port which is used for probing the backend servers.
    #[builder(into)]
    #[serde(rename = "port")]
    pub r#port: Box<i32>,
    /// The ID of the associated Probe.
    #[builder(into)]
    #[serde(rename = "probeId")]
    pub r#probe_id: Box<String>,
    /// The name of the associated HTTP Probe.
    #[builder(into)]
    #[serde(rename = "probeName")]
    pub r#probe_name: Box<String>,
    /// The Protocol used for this Probe.
    #[builder(into)]
    #[serde(rename = "protocol")]
    pub r#protocol: Box<String>,
    /// The request timeout in seconds.
    #[builder(into)]
    #[serde(rename = "requestTimeout")]
    pub r#request_timeout: Box<i32>,
    /// A list of `trusted_root_certificate` names.
    #[builder(into)]
    #[serde(rename = "trustedRootCertificateNames")]
    pub r#trusted_root_certificate_names: Box<Vec<String>>,
}
