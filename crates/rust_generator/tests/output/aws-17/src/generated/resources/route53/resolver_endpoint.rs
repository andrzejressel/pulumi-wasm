/// Provides a Route 53 Resolver endpoint resource.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   foo:
///     type: aws:route53:ResolverEndpoint
///     properties:
///       name: foo
///       direction: INBOUND
///       resolverEndpointType: IPV4
///       securityGroupIds:
///         - ${sg1.id}
///         - ${sg2.id}
///       ipAddresses:
///         - subnetId: ${sn1.id}
///         - subnetId: ${sn2.id}
///           ip: 10.0.64.4
///       protocols:
///         - Do53
///         - DoH
///       tags:
///         Environment: Prod
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import  Route 53 Resolver endpoints using the Route 53 Resolver endpoint ID. For example:
///
/// ```sh
/// $ pulumi import aws:route53/resolverEndpoint:ResolverEndpoint foo rslvr-in-abcdef01234567890
/// ```
pub mod resolver_endpoint {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ResolverEndpointArgs {
        /// Direction of DNS queries to or from the Route 53 Resolver endpoint.
        /// Valid values are `INBOUND` (resolver forwards DNS queries to the DNS service for a VPC from your network or another VPC)
        /// or `OUTBOUND` (resolver forwards DNS queries from the DNS service for a VPC to your network or another VPC).
        #[builder(into)]
        pub direction: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Subnets and IP addresses in your VPC that you want DNS queries to pass through on the way from your VPCs
        /// to your network (for outbound endpoints) or on the way from your network to your VPCs (for inbound endpoints). Described below.
        #[builder(into)]
        pub ip_addresses: pulumi_gestalt_rust::InputOrOutput<
            Vec<super::super::types::route53::ResolverEndpointIpAddress>,
        >,
        /// Friendly name of the Route 53 Resolver endpoint.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Protocols you want to use for the Route 53 Resolver endpoint. Valid values: `DoH`, `Do53`, `DoH-FIPS`.
        #[builder(into, default)]
        pub protocols: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// Route 53 Resolver endpoint IP address type. Valid values: `IPV4`, `IPV6`, `DUALSTACK`.
        #[builder(into, default)]
        pub resolver_endpoint_type: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// ID of one or more security groups that you want to use to control access to this VPC.
        #[builder(into)]
        pub security_group_ids: pulumi_gestalt_rust::InputOrOutput<Vec<String>>,
        /// Map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct ResolverEndpointResult {
        /// ARN of the Route 53 Resolver endpoint.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// Direction of DNS queries to or from the Route 53 Resolver endpoint.
        /// Valid values are `INBOUND` (resolver forwards DNS queries to the DNS service for a VPC from your network or another VPC)
        /// or `OUTBOUND` (resolver forwards DNS queries from the DNS service for a VPC to your network or another VPC).
        pub direction: pulumi_gestalt_rust::Output<String>,
        /// ID of the VPC that you want to create the resolver endpoint in.
        pub host_vpc_id: pulumi_gestalt_rust::Output<String>,
        /// Subnets and IP addresses in your VPC that you want DNS queries to pass through on the way from your VPCs
        /// to your network (for outbound endpoints) or on the way from your network to your VPCs (for inbound endpoints). Described below.
        pub ip_addresses: pulumi_gestalt_rust::Output<
            Vec<super::super::types::route53::ResolverEndpointIpAddress>,
        >,
        /// Friendly name of the Route 53 Resolver endpoint.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Protocols you want to use for the Route 53 Resolver endpoint. Valid values: `DoH`, `Do53`, `DoH-FIPS`.
        pub protocols: pulumi_gestalt_rust::Output<Vec<String>>,
        /// Route 53 Resolver endpoint IP address type. Valid values: `IPV4`, `IPV6`, `DUALSTACK`.
        pub resolver_endpoint_type: pulumi_gestalt_rust::Output<String>,
        /// ID of one or more security groups that you want to use to control access to this VPC.
        pub security_group_ids: pulumi_gestalt_rust::Output<Vec<String>>,
        /// Map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: ResolverEndpointArgs,
    ) -> ResolverEndpointResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let direction_binding = args.direction.get_output(context).get_inner();
        let ip_addresses_binding = args.ip_addresses.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let protocols_binding = args.protocols.get_output(context).get_inner();
        let resolver_endpoint_type_binding = args
            .resolver_endpoint_type
            .get_output(context)
            .get_inner();
        let security_group_ids_binding = args
            .security_group_ids
            .get_output(context)
            .get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:route53/resolverEndpoint:ResolverEndpoint".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "direction".into(),
                    value: &direction_binding,
                },
                register_interface::ObjectField {
                    name: "ipAddresses".into(),
                    value: &ip_addresses_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "protocols".into(),
                    value: &protocols_binding,
                },
                register_interface::ObjectField {
                    name: "resolverEndpointType".into(),
                    value: &resolver_endpoint_type_binding,
                },
                register_interface::ObjectField {
                    name: "securityGroupIds".into(),
                    value: &security_group_ids_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        ResolverEndpointResult {
            arn: pulumi_gestalt_rust::__private::into_domain(o.extract_field("arn")),
            direction: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("direction"),
            ),
            host_vpc_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("hostVpcId"),
            ),
            ip_addresses: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("ipAddresses"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            protocols: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("protocols"),
            ),
            resolver_endpoint_type: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("resolverEndpointType"),
            ),
            security_group_ids: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("securityGroupIds"),
            ),
            tags: pulumi_gestalt_rust::__private::into_domain(o.extract_field("tags")),
            tags_all: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("tagsAll"),
            ),
        }
    }
}
