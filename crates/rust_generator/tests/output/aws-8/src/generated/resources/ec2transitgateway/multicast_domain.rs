/// Manages an EC2 Transit Gateway Multicast Domain.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   vpc1:
///     type: aws:ec2:Vpc
///     properties:
///       cidrBlock: 10.0.0.0/16
///   vpc2:
///     type: aws:ec2:Vpc
///     properties:
///       cidrBlock: 10.1.0.0/16
///   subnet1:
///     type: aws:ec2:Subnet
///     properties:
///       vpcId: ${vpc1.id}
///       cidrBlock: 10.0.1.0/24
///       availabilityZone: ${available.names[0]}
///   subnet2:
///     type: aws:ec2:Subnet
///     properties:
///       vpcId: ${vpc1.id}
///       cidrBlock: 10.0.2.0/24
///       availabilityZone: ${available.names[1]}
///   subnet3:
///     type: aws:ec2:Subnet
///     properties:
///       vpcId: ${vpc2.id}
///       cidrBlock: 10.1.1.0/24
///       availabilityZone: ${available.names[0]}
///   instance1:
///     type: aws:ec2:Instance
///     properties:
///       ami: ${amazonLinux.id}
///       instanceType: t2.micro
///       subnetId: ${subnet1.id}
///   instance2:
///     type: aws:ec2:Instance
///     properties:
///       ami: ${amazonLinux.id}
///       instanceType: t2.micro
///       subnetId: ${subnet2.id}
///   instance3:
///     type: aws:ec2:Instance
///     properties:
///       ami: ${amazonLinux.id}
///       instanceType: t2.micro
///       subnetId: ${subnet3.id}
///   tgw:
///     type: aws:ec2transitgateway:TransitGateway
///     properties:
///       multicastSupport: enable
///   attachment1:
///     type: aws:ec2transitgateway:VpcAttachment
///     properties:
///       subnetIds:
///         - ${subnet1.id}
///         - ${subnet2.id}
///       transitGatewayId: ${tgw.id}
///       vpcId: ${vpc1.id}
///   attachment2:
///     type: aws:ec2transitgateway:VpcAttachment
///     properties:
///       subnetIds:
///         - ${subnet3.id}
///       transitGatewayId: ${tgw.id}
///       vpcId: ${vpc2.id}
///   domain:
///     type: aws:ec2transitgateway:MulticastDomain
///     properties:
///       transitGatewayId: ${tgw.id}
///       staticSourcesSupport: enable
///       tags:
///         Name: Transit_Gateway_Multicast_Domain_Example
///   association3:
///     type: aws:ec2transitgateway:MulticastDomainAssociation
///     properties:
///       subnetId: ${subnet3.id}
///       transitGatewayAttachmentId: ${attachment2.id}
///       transitGatewayMulticastDomainId: ${domain.id}
///   source:
///     type: aws:ec2transitgateway:MulticastGroupSource
///     properties:
///       groupIpAddress: 224.0.0.1
///       networkInterfaceId: ${instance3.primaryNetworkInterfaceId}
///       transitGatewayMulticastDomainId: ${association3.transitGatewayMulticastDomainId}
///   association1:
///     type: aws:ec2transitgateway:MulticastDomainAssociation
///     properties:
///       subnetId: ${subnet1.id}
///       transitGatewayAttachmentId: ${attachment1.id}
///       transitGatewayMulticastDomainId: ${domain.id}
///   association2:
///     type: aws:ec2transitgateway:MulticastDomainAssociation
///     properties:
///       subnetId: ${subnet2.id}
///       transitGatewayAttachmentId: ${attachment2.id}
///       transitGatewayMulticastDomainId: ${domain.id}
///   member1:
///     type: aws:ec2transitgateway:MulticastGroupMember
///     properties:
///       groupIpAddress: 224.0.0.1
///       networkInterfaceId: ${instance1.primaryNetworkInterfaceId}
///       transitGatewayMulticastDomainId: ${association1.transitGatewayMulticastDomainId}
///   member2:
///     type: aws:ec2transitgateway:MulticastGroupMember
///     properties:
///       groupIpAddress: 224.0.0.1
///       networkInterfaceId: ${instance2.primaryNetworkInterfaceId}
///       transitGatewayMulticastDomainId: ${association1.transitGatewayMulticastDomainId}
/// variables:
///   available:
///     fn::invoke:
///       function: aws:getAvailabilityZones
///       arguments:
///         state: available
///   amazonLinux:
///     fn::invoke:
///       function: aws:ec2:getAmi
///       arguments:
///         mostRecent: true
///         owners:
///           - amazon
///         filters:
///           - name: name
///             values:
///               - amzn-ami-hvm-*-x86_64-gp2
///           - name: owner-alias
///             values:
///               - amazon
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import `aws_ec2_transit_gateway_multicast_domain` using the EC2 Transit Gateway Multicast Domain identifier. For example:
///
/// ```sh
/// $ pulumi import aws:ec2transitgateway/multicastDomain:MulticastDomain example tgw-mcast-domain-12345
/// ```
pub mod multicast_domain {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct MulticastDomainArgs {
        /// Whether to automatically accept cross-account subnet associations that are associated with the EC2 Transit Gateway Multicast Domain. Valid values: `disable`, `enable`. Default value: `disable`.
        #[builder(into, default)]
        pub auto_accept_shared_associations: pulumi_wasm_rust::InputOrOutput<
            Option<String>,
        >,
        /// Whether to enable Internet Group Management Protocol (IGMP) version 2 for the EC2 Transit Gateway Multicast Domain. Valid values: `disable`, `enable`. Default value: `disable`.
        #[builder(into, default)]
        pub igmpv2_support: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Whether to enable support for statically configuring multicast group sources for the EC2 Transit Gateway Multicast Domain. Valid values: `disable`, `enable`. Default value: `disable`.
        #[builder(into, default)]
        pub static_sources_support: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Key-value tags for the EC2 Transit Gateway Multicast Domain. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// EC2 Transit Gateway identifier. The EC2 Transit Gateway must have `multicast_support` enabled.
        #[builder(into)]
        pub transit_gateway_id: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct MulticastDomainResult {
        /// EC2 Transit Gateway Multicast Domain Amazon Resource Name (ARN).
        pub arn: pulumi_wasm_rust::Output<String>,
        /// Whether to automatically accept cross-account subnet associations that are associated with the EC2 Transit Gateway Multicast Domain. Valid values: `disable`, `enable`. Default value: `disable`.
        pub auto_accept_shared_associations: pulumi_wasm_rust::Output<Option<String>>,
        /// Whether to enable Internet Group Management Protocol (IGMP) version 2 for the EC2 Transit Gateway Multicast Domain. Valid values: `disable`, `enable`. Default value: `disable`.
        pub igmpv2_support: pulumi_wasm_rust::Output<Option<String>>,
        /// Identifier of the AWS account that owns the EC2 Transit Gateway Multicast Domain.
        pub owner_id: pulumi_wasm_rust::Output<String>,
        /// Whether to enable support for statically configuring multicast group sources for the EC2 Transit Gateway Multicast Domain. Valid values: `disable`, `enable`. Default value: `disable`.
        pub static_sources_support: pulumi_wasm_rust::Output<Option<String>>,
        /// Key-value tags for the EC2 Transit Gateway Multicast Domain. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// EC2 Transit Gateway identifier. The EC2 Transit Gateway must have `multicast_support` enabled.
        pub transit_gateway_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: MulticastDomainArgs,
    ) -> MulticastDomainResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let auto_accept_shared_associations_binding = args
            .auto_accept_shared_associations
            .get_output(context)
            .get_inner();
        let igmpv2_support_binding = args.igmpv2_support.get_output(context).get_inner();
        let static_sources_support_binding = args
            .static_sources_support
            .get_output(context)
            .get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let transit_gateway_id_binding = args
            .transit_gateway_id
            .get_output(context)
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:ec2transitgateway/multicastDomain:MulticastDomain".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "autoAcceptSharedAssociations".into(),
                    value: &auto_accept_shared_associations_binding,
                },
                register_interface::ObjectField {
                    name: "igmpv2Support".into(),
                    value: &igmpv2_support_binding,
                },
                register_interface::ObjectField {
                    name: "staticSourcesSupport".into(),
                    value: &static_sources_support_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "transitGatewayId".into(),
                    value: &transit_gateway_id_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        MulticastDomainResult {
            arn: pulumi_wasm_rust::__private::into_domain(o.extract_field("arn")),
            auto_accept_shared_associations: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("autoAcceptSharedAssociations"),
            ),
            igmpv2_support: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("igmpv2Support"),
            ),
            owner_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("ownerId"),
            ),
            static_sources_support: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("staticSourcesSupport"),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(o.extract_field("tags")),
            tags_all: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("tagsAll"),
            ),
            transit_gateway_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("transitGatewayId"),
            ),
        }
    }
}
