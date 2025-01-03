#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct LoadBalancerListener {
    /// The port on the instance to route to
    #[builder(into)]
    #[serde(rename = "instancePort")]
    pub r#instance_port: Box<i32>,
    /// The protocol to use to the instance. Valid
    /// values are `HTTP`, `HTTPS`, `TCP`, or `SSL`
    #[builder(into)]
    #[serde(rename = "instanceProtocol")]
    pub r#instance_protocol: Box<String>,
    /// The port to listen on for the load balancer
    #[builder(into)]
    #[serde(rename = "lbPort")]
    pub r#lb_port: Box<i32>,
    /// The protocol to listen on. Valid values are `HTTP`,
    /// `HTTPS`, `TCP`, or `SSL`
    #[builder(into)]
    #[serde(rename = "lbProtocol")]
    pub r#lb_protocol: Box<String>,
    /// The ARN of an SSL certificate you have
    /// uploaded to AWS IAM. **Note ECDSA-specific restrictions below.  Only valid when `lb_protocol` is either HTTPS or SSL**
    #[builder(into, default)]
    #[serde(rename = "sslCertificateId")]
    pub r#ssl_certificate_id: Box<Option<String>>,
}
