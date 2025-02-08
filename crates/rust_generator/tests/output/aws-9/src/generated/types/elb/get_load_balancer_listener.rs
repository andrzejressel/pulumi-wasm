#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetLoadBalancerListener {
    #[builder(into)]
    #[serde(rename = "instancePort")]
    pub r#instance_port: Box<i32>,
    #[builder(into)]
    #[serde(rename = "instanceProtocol")]
    pub r#instance_protocol: Box<String>,
    #[builder(into)]
    #[serde(rename = "lbPort")]
    pub r#lb_port: Box<i32>,
    #[builder(into)]
    #[serde(rename = "lbProtocol")]
    pub r#lb_protocol: Box<String>,
    #[builder(into)]
    #[serde(rename = "sslCertificateId")]
    pub r#ssl_certificate_id: Box<String>,
}
