#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct AppGatewayAllocatedConnection {
    /// The ingress port of an allocated connection.
    #[builder(into, default)]
    #[serde(rename = "ingressPort")]
    pub r#ingress_port: Box<Option<i32>>,
    /// The PSC uri of an allocated connection.
    #[builder(into, default)]
    #[serde(rename = "pscUri")]
    pub r#psc_uri: Box<Option<String>>,
}
