#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct NetworkAclEgress {
    /// The action to take.
    #[builder(into)]
    #[serde(rename = "action")]
    pub r#action: Box<String>,
    /// The CIDR block to match. This must be a
    /// valid network mask.
    #[builder(into, default)]
    #[serde(rename = "cidrBlock")]
    pub r#cidr_block: Box<Option<String>>,
    /// The from port to match.
    #[builder(into)]
    #[serde(rename = "fromPort")]
    pub r#from_port: Box<i32>,
    /// The ICMP type code to be used. Default 0.
    /// 
    /// > Note: For more information on ICMP types and codes, see here: https://www.iana.org/assignments/icmp-parameters/icmp-parameters.xhtml
    #[builder(into, default)]
    #[serde(rename = "icmpCode")]
    pub r#icmp_code: Box<Option<i32>>,
    /// The ICMP type to be used. Default 0.
    #[builder(into, default)]
    #[serde(rename = "icmpType")]
    pub r#icmp_type: Box<Option<i32>>,
    /// The IPv6 CIDR block.
    #[builder(into, default)]
    #[serde(rename = "ipv6CidrBlock")]
    pub r#ipv_6_cidr_block: Box<Option<String>>,
    /// The protocol to match. If using the -1 'all'
    /// protocol, you must specify a from and to port of 0.
    #[builder(into)]
    #[serde(rename = "protocol")]
    pub r#protocol: Box<String>,
    /// The rule number. Used for ordering.
    #[builder(into)]
    #[serde(rename = "ruleNo")]
    pub r#rule_no: Box<i32>,
    /// The to port to match.
    #[builder(into)]
    #[serde(rename = "toPort")]
    pub r#to_port: Box<i32>,
}
