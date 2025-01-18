/// Manages an EC2 Transit Gateway VPC Attachment. For examples of custom route table association and propagation, see the EC2 Transit Gateway Networking Examples Guide.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = vpc_attachment::create(
///         "example",
///         VpcAttachmentArgs::builder()
///             .subnet_ids(vec!["${exampleAwsSubnet.id}",])
///             .transit_gateway_id("${exampleAwsEc2TransitGateway.id}")
///             .vpc_id("${exampleAwsVpc.id}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import `aws_ec2_transit_gateway_vpc_attachment` using the EC2 Transit Gateway Attachment identifier. For example:
///
/// ```sh
/// $ pulumi import aws:ec2transitgateway/vpcAttachment:VpcAttachment example tgw-attach-12345678
/// ```
pub mod vpc_attachment {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct VpcAttachmentArgs {
        /// Whether Appliance Mode support is enabled. If enabled, a traffic flow between a source and destination uses the same Availability Zone for the VPC attachment for the lifetime of that flow. Valid values: `disable`, `enable`. Default value: `disable`.
        #[builder(into, default)]
        pub appliance_mode_support: pulumi_wasm_rust::Output<Option<String>>,
        /// Whether DNS support is enabled. Valid values: `disable`, `enable`. Default value: `enable`.
        #[builder(into, default)]
        pub dns_support: pulumi_wasm_rust::Output<Option<String>>,
        /// Whether IPv6 support is enabled. Valid values: `disable`, `enable`. Default value: `disable`.
        #[builder(into, default)]
        pub ipv6_support: pulumi_wasm_rust::Output<Option<String>>,
        /// Whether Security Group Referencing Support is enabled. Valid values: `disable`, `enable`.
        #[builder(into, default)]
        pub security_group_referencing_support: pulumi_wasm_rust::Output<Option<String>>,
        /// Identifiers of EC2 Subnets.
        #[builder(into)]
        pub subnet_ids: pulumi_wasm_rust::Output<Vec<String>>,
        /// Key-value tags for the EC2 Transit Gateway VPC Attachment. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Boolean whether the VPC Attachment should be associated with the EC2 Transit Gateway association default route table. This cannot be configured or perform drift detection with Resource Access Manager shared EC2 Transit Gateways. Default value: `true`.
        #[builder(into, default)]
        pub transit_gateway_default_route_table_association: pulumi_wasm_rust::Output<
            Option<bool>,
        >,
        /// Boolean whether the VPC Attachment should propagate routes with the EC2 Transit Gateway propagation default route table. This cannot be configured or perform drift detection with Resource Access Manager shared EC2 Transit Gateways. Default value: `true`.
        #[builder(into, default)]
        pub transit_gateway_default_route_table_propagation: pulumi_wasm_rust::Output<
            Option<bool>,
        >,
        /// Identifier of EC2 Transit Gateway.
        #[builder(into)]
        pub transit_gateway_id: pulumi_wasm_rust::Output<String>,
        /// Identifier of EC2 VPC.
        #[builder(into)]
        pub vpc_id: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct VpcAttachmentResult {
        /// Whether Appliance Mode support is enabled. If enabled, a traffic flow between a source and destination uses the same Availability Zone for the VPC attachment for the lifetime of that flow. Valid values: `disable`, `enable`. Default value: `disable`.
        pub appliance_mode_support: pulumi_wasm_rust::Output<Option<String>>,
        /// Whether DNS support is enabled. Valid values: `disable`, `enable`. Default value: `enable`.
        pub dns_support: pulumi_wasm_rust::Output<Option<String>>,
        /// Whether IPv6 support is enabled. Valid values: `disable`, `enable`. Default value: `disable`.
        pub ipv6_support: pulumi_wasm_rust::Output<Option<String>>,
        /// Whether Security Group Referencing Support is enabled. Valid values: `disable`, `enable`.
        pub security_group_referencing_support: pulumi_wasm_rust::Output<String>,
        /// Identifiers of EC2 Subnets.
        pub subnet_ids: pulumi_wasm_rust::Output<Vec<String>>,
        /// Key-value tags for the EC2 Transit Gateway VPC Attachment. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Boolean whether the VPC Attachment should be associated with the EC2 Transit Gateway association default route table. This cannot be configured or perform drift detection with Resource Access Manager shared EC2 Transit Gateways. Default value: `true`.
        pub transit_gateway_default_route_table_association: pulumi_wasm_rust::Output<
            bool,
        >,
        /// Boolean whether the VPC Attachment should propagate routes with the EC2 Transit Gateway propagation default route table. This cannot be configured or perform drift detection with Resource Access Manager shared EC2 Transit Gateways. Default value: `true`.
        pub transit_gateway_default_route_table_propagation: pulumi_wasm_rust::Output<
            bool,
        >,
        /// Identifier of EC2 Transit Gateway.
        pub transit_gateway_id: pulumi_wasm_rust::Output<String>,
        /// Identifier of EC2 VPC.
        pub vpc_id: pulumi_wasm_rust::Output<String>,
        /// Identifier of the AWS account that owns the EC2 VPC.
        pub vpc_owner_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: VpcAttachmentArgs) -> VpcAttachmentResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let appliance_mode_support_binding = args.appliance_mode_support.get_inner();
        let dns_support_binding = args.dns_support.get_inner();
        let ipv6_support_binding = args.ipv6_support.get_inner();
        let security_group_referencing_support_binding = args
            .security_group_referencing_support
            .get_inner();
        let subnet_ids_binding = args.subnet_ids.get_inner();
        let tags_binding = args.tags.get_inner();
        let transit_gateway_default_route_table_association_binding = args
            .transit_gateway_default_route_table_association
            .get_inner();
        let transit_gateway_default_route_table_propagation_binding = args
            .transit_gateway_default_route_table_propagation
            .get_inner();
        let transit_gateway_id_binding = args.transit_gateway_id.get_inner();
        let vpc_id_binding = args.vpc_id.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:ec2transitgateway/vpcAttachment:VpcAttachment".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "applianceModeSupport".into(),
                    value: &appliance_mode_support_binding,
                },
                register_interface::ObjectField {
                    name: "dnsSupport".into(),
                    value: &dns_support_binding,
                },
                register_interface::ObjectField {
                    name: "ipv6Support".into(),
                    value: &ipv6_support_binding,
                },
                register_interface::ObjectField {
                    name: "securityGroupReferencingSupport".into(),
                    value: &security_group_referencing_support_binding,
                },
                register_interface::ObjectField {
                    name: "subnetIds".into(),
                    value: &subnet_ids_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "transitGatewayDefaultRouteTableAssociation".into(),
                    value: &transit_gateway_default_route_table_association_binding,
                },
                register_interface::ObjectField {
                    name: "transitGatewayDefaultRouteTablePropagation".into(),
                    value: &transit_gateway_default_route_table_propagation_binding,
                },
                register_interface::ObjectField {
                    name: "transitGatewayId".into(),
                    value: &transit_gateway_id_binding,
                },
                register_interface::ObjectField {
                    name: "vpcId".into(),
                    value: &vpc_id_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "applianceModeSupport".into(),
                },
                register_interface::ResultField {
                    name: "dnsSupport".into(),
                },
                register_interface::ResultField {
                    name: "ipv6Support".into(),
                },
                register_interface::ResultField {
                    name: "securityGroupReferencingSupport".into(),
                },
                register_interface::ResultField {
                    name: "subnetIds".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "tagsAll".into(),
                },
                register_interface::ResultField {
                    name: "transitGatewayDefaultRouteTableAssociation".into(),
                },
                register_interface::ResultField {
                    name: "transitGatewayDefaultRouteTablePropagation".into(),
                },
                register_interface::ResultField {
                    name: "transitGatewayId".into(),
                },
                register_interface::ResultField {
                    name: "vpcId".into(),
                },
                register_interface::ResultField {
                    name: "vpcOwnerId".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        VpcAttachmentResult {
            appliance_mode_support: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("applianceModeSupport").unwrap(),
            ),
            dns_support: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("dnsSupport").unwrap(),
            ),
            ipv6_support: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("ipv6Support").unwrap(),
            ),
            security_group_referencing_support: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("securityGroupReferencingSupport").unwrap(),
            ),
            subnet_ids: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("subnetIds").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            tags_all: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tagsAll").unwrap(),
            ),
            transit_gateway_default_route_table_association: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("transitGatewayDefaultRouteTableAssociation").unwrap(),
            ),
            transit_gateway_default_route_table_propagation: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("transitGatewayDefaultRouteTablePropagation").unwrap(),
            ),
            transit_gateway_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("transitGatewayId").unwrap(),
            ),
            vpc_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("vpcId").unwrap(),
            ),
            vpc_owner_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("vpcOwnerId").unwrap(),
            ),
        }
    }
}
