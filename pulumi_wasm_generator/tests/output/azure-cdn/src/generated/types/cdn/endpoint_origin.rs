#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct EndpointOrigin {
    /// A string that determines the hostname/IP address of the origin server. This string can be a domain name, Storage Account endpoint, Web App endpoint, IPv4 address or IPv6 address. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "hostName")]
    pub r#host_name: Box<String>,
    /// The HTTP port of the origin. Defaults to `80`. Changing this forces a new resource to be created.
    #[builder(into, default)]
    #[serde(rename = "httpPort")]
    pub r#http_port: Box<Option<i32>>,
    /// The HTTPS port of the origin. Defaults to `443`. Changing this forces a new resource to be created.
    #[builder(into, default)]
    #[serde(rename = "httpsPort")]
    pub r#https_port: Box<Option<i32>>,
    /// The name of the origin. This is an arbitrary value. However, this value needs to be unique under the endpoint. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
}
