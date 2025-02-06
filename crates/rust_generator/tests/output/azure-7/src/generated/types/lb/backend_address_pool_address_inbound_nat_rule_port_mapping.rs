#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct BackendAddressPoolAddressInboundNatRulePortMapping {
    /// The Backend Port of the Load Balancing Inbound NAT Rules associated with this Backend Address Pool Address.
    #[builder(into, default)]
    #[serde(rename = "backendPort")]
    pub r#backend_port: Box<Option<i32>>,
    /// The Frontend Port of the Load Balancing Inbound NAT Rules associated with this Backend Address Pool Address.
    #[builder(into, default)]
    #[serde(rename = "frontendPort")]
    pub r#frontend_port: Box<Option<i32>>,
    /// The name of the Load Balancing Inbound NAT Rules associated with this Backend Address Pool Address.
    #[builder(into, default)]
    #[serde(rename = "inboundNatRuleName")]
    pub r#inbound_nat_rule_name: Box<Option<String>>,
}
