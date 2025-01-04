#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GroupTrafficSource {
    /// Identifies the traffic source. For Application Load Balancers, Gateway Load Balancers, Network Load Balancers, and VPC Lattice, this will be the Amazon Resource Name (ARN) for a target group in this account and Region. For Classic Load Balancers, this will be the name of the Classic Load Balancer in this account and Region.
    #[builder(into)]
    #[serde(rename = "identifier")]
    pub r#identifier: Box<String>,
    /// Provides additional context for the value of Identifier.
    /// The following lists the valid values:
    /// `elb` if `identifier` is the name of a Classic Load Balancer.
    /// `elbv2` if `identifier` is the ARN of an Application Load Balancer, Gateway Load Balancer, or Network Load Balancer target group.
    /// `vpc-lattice` if `identifier` is the ARN of a VPC Lattice target group.
    #[builder(into, default)]
    #[serde(rename = "type")]
    pub r#type_: Box<Option<String>>,
}
