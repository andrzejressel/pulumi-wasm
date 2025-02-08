#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct GetBackendAddressPoolBackendAddressInboundNatRulePortMapping {
    /// The Backend Port of the Load Balancing Inbound NAT Rules associated with this Backend Address Pool Address.
    #[builder(into)]
    #[serde(rename = "backendPort")]
    pub r#backend_port: Box<i32>,
    /// The Frontend Port of the Load Balancing Inbound NAT Rules associated with this Backend Address Pool Address.
    #[builder(into)]
    #[serde(rename = "frontendPort")]
    pub r#frontend_port: Box<i32>,
    /// The name of the Load Balancing Inbound NAT Rules associated with this Backend Address Pool Address.
    #[builder(into)]
    #[serde(rename = "inboundNatRuleName")]
    pub r#inbound_nat_rule_name: Box<String>,
}
