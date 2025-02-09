/// Provides a VPC Endpoint resource.
///
/// > **NOTE on VPC Endpoints and VPC Endpoint Associations:** The provider provides both standalone VPC Endpoint Associations for
/// Route Tables - (an association between a VPC endpoint and a single `route_table_id`),
/// Security Groups - (an association between a VPC endpoint and a single `security_group_id`),
/// and Subnets - (an association between a VPC endpoint and a single `subnet_id`) and
/// a VPC Endpoint resource with `route_table_ids` and `subnet_ids` attributes.
/// Do not use the same resource ID in both a VPC Endpoint resource and a VPC Endpoint Association resource.
/// Doing so will cause a conflict of associations and will overwrite the association.
///
/// ## Example Usage
///
/// ### Basic
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let s3 = vpc_endpoint::create(
///         "s3",
///         VpcEndpointArgs::builder()
///             .service_name("com.amazonaws.us-west-2.s3")
///             .vpc_id("${main.id}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### Basic w/ Tags
///
/// ```yaml
/// resources:
///   s3:
///     type: aws:ec2:VpcEndpoint
///     properties:
///       vpcId: ${main.id}
///       serviceName: com.amazonaws.us-west-2.s3
///       tags:
///         Environment: test
/// ```
///
/// ### Interface Endpoint Type
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let ec2 = vpc_endpoint::create(
///         "ec2",
///         VpcEndpointArgs::builder()
///             .private_dns_enabled(true)
///             .security_group_ids(vec!["${sg1.id}",])
///             .service_name("com.amazonaws.us-west-2.ec2")
///             .vpc_endpoint_type("Interface")
///             .vpc_id("${main.id}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### Interface Endpoint Type with User-Defined IP Address
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let ec2 = vpc_endpoint::create(
///         "ec2",
///         VpcEndpointArgs::builder()
///             .service_name("com.amazonaws.us-west-2.ec2")
///             .subnet_configurations(
///                 vec![
///                     VpcEndpointSubnetConfiguration::builder().ipv4("10.0.1.10")
///                     .subnetId("${example1.id}").build_struct(),
///                     VpcEndpointSubnetConfiguration::builder().ipv4("10.0.2.10")
///                     .subnetId("${example2.id}").build_struct(),
///                 ],
///             )
///             .subnet_ids(vec!["${example1.id}", "${example2.id}",])
///             .vpc_endpoint_type("Interface")
///             .vpc_id("${example.id}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### Gateway Load Balancer Endpoint Type
///
/// ```yaml
/// resources:
///   example:
///     type: aws:ec2:VpcEndpointService
///     properties:
///       acceptanceRequired: false
///       allowedPrincipals:
///         - ${current.arn}
///       gatewayLoadBalancerArns:
///         - ${exampleAwsLb.arn}
///   exampleVpcEndpoint:
///     type: aws:ec2:VpcEndpoint
///     name: example
///     properties:
///       serviceName: ${example.serviceName}
///       subnetIds:
///         - ${exampleAwsSubnet.id}
///       vpcEndpointType: ${example.serviceType}
///       vpcId: ${exampleAwsVpc.id}
/// variables:
///   current:
///     fn::invoke:
///       function: aws:getCallerIdentity
///       arguments: {}
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import VPC Endpoints using the VPC endpoint `id`. For example:
///
/// ```sh
/// $ pulumi import aws:ec2/vpcEndpoint:VpcEndpoint endpoint1 vpce-3ecf2a57
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod vpc_endpoint {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct VpcEndpointArgs {
        /// Accept the VPC endpoint (the VPC endpoint and service need to be in the same AWS account).
        #[builder(into, default)]
        pub auto_accept: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// The DNS options for the endpoint. See dns_options below.
        #[builder(into, default)]
        pub dns_options: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::ec2::VpcEndpointDnsOptions>,
        >,
        /// The IP address type for the endpoint. Valid values are `ipv4`, `dualstack`, and `ipv6`.
        #[builder(into, default)]
        pub ip_address_type: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A policy to attach to the endpoint that controls access to the service. This is a JSON formatted string. Defaults to full access. All `Gateway` and some `Interface` endpoints support policies - see the [relevant AWS documentation](https://docs.aws.amazon.com/vpc/latest/userguide/vpc-endpoints-access.html) for more details.
        #[builder(into, default)]
        pub policy: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Whether or not to associate a private hosted zone with the specified VPC. Applicable for endpoints of type `Interface`. Most users will want this enabled to allow services within the VPC to automatically use the endpoint.
        /// Defaults to `false`.
        #[builder(into, default)]
        pub private_dns_enabled: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// One or more route table IDs. Applicable for endpoints of type `Gateway`.
        #[builder(into, default)]
        pub route_table_ids: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// The ID of one or more security groups to associate with the network interface. Applicable for endpoints of type `Interface`.
        /// If no security groups are specified, the VPC's [default security group](https://docs.aws.amazon.com/vpc/latest/userguide/VPC_SecurityGroups.html#DefaultSecurityGroup) is associated with the endpoint.
        #[builder(into, default)]
        pub security_group_ids: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// The service name. For AWS services the service name is usually in the form `com.amazonaws.<region>.<service>` (the SageMaker Notebook service is an exception to this rule, the service name is in the form `aws.sagemaker.<region>.notebook`).
        #[builder(into)]
        pub service_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The AWS region of the VPC Endpoint Service. If specified, the VPC endpoint will connect to the service in the provided region. Applicable for endpoints of type `Interface`.
        #[builder(into, default)]
        pub service_region: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Subnet configuration for the endpoint, used to select specific IPv4 and/or IPv6 addresses to the endpoint. See subnet_configuration below.
        #[builder(into, default)]
        pub subnet_configurations: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::ec2::VpcEndpointSubnetConfiguration>>,
        >,
        /// The ID of one or more subnets in which to create a network interface for the endpoint. Applicable for endpoints of type `GatewayLoadBalancer` and `Interface`. Interface type endpoints cannot function without being assigned to a subnet.
        #[builder(into, default)]
        pub subnet_ids: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// A map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The VPC endpoint type, `Gateway`, `GatewayLoadBalancer`, or `Interface`. Defaults to `Gateway`.
        #[builder(into, default)]
        pub vpc_endpoint_type: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ID of the VPC in which the endpoint will be used.
        #[builder(into)]
        pub vpc_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct VpcEndpointResult {
        /// The Amazon Resource Name (ARN) of the VPC endpoint.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// Accept the VPC endpoint (the VPC endpoint and service need to be in the same AWS account).
        pub auto_accept: pulumi_gestalt_rust::Output<Option<bool>>,
        /// The list of CIDR blocks for the exposed AWS service. Applicable for endpoints of type `Gateway`.
        pub cidr_blocks: pulumi_gestalt_rust::Output<Vec<String>>,
        /// The DNS entries for the VPC Endpoint. Applicable for endpoints of type `Interface`. DNS blocks are documented below.
        pub dns_entries: pulumi_gestalt_rust::Output<
            Vec<super::super::types::ec2::VpcEndpointDnsEntry>,
        >,
        /// The DNS options for the endpoint. See dns_options below.
        pub dns_options: pulumi_gestalt_rust::Output<
            super::super::types::ec2::VpcEndpointDnsOptions,
        >,
        /// The IP address type for the endpoint. Valid values are `ipv4`, `dualstack`, and `ipv6`.
        pub ip_address_type: pulumi_gestalt_rust::Output<String>,
        /// One or more network interfaces for the VPC Endpoint. Applicable for endpoints of type `Interface`.
        pub network_interface_ids: pulumi_gestalt_rust::Output<Vec<String>>,
        /// The ID of the AWS account that owns the VPC endpoint.
        pub owner_id: pulumi_gestalt_rust::Output<String>,
        /// A policy to attach to the endpoint that controls access to the service. This is a JSON formatted string. Defaults to full access. All `Gateway` and some `Interface` endpoints support policies - see the [relevant AWS documentation](https://docs.aws.amazon.com/vpc/latest/userguide/vpc-endpoints-access.html) for more details.
        pub policy: pulumi_gestalt_rust::Output<String>,
        /// The prefix list ID of the exposed AWS service. Applicable for endpoints of type `Gateway`.
        pub prefix_list_id: pulumi_gestalt_rust::Output<String>,
        /// Whether or not to associate a private hosted zone with the specified VPC. Applicable for endpoints of type `Interface`. Most users will want this enabled to allow services within the VPC to automatically use the endpoint.
        /// Defaults to `false`.
        pub private_dns_enabled: pulumi_gestalt_rust::Output<bool>,
        /// Whether or not the VPC Endpoint is being managed by its service - `true` or `false`.
        pub requester_managed: pulumi_gestalt_rust::Output<bool>,
        /// One or more route table IDs. Applicable for endpoints of type `Gateway`.
        pub route_table_ids: pulumi_gestalt_rust::Output<Vec<String>>,
        /// The ID of one or more security groups to associate with the network interface. Applicable for endpoints of type `Interface`.
        /// If no security groups are specified, the VPC's [default security group](https://docs.aws.amazon.com/vpc/latest/userguide/VPC_SecurityGroups.html#DefaultSecurityGroup) is associated with the endpoint.
        pub security_group_ids: pulumi_gestalt_rust::Output<Vec<String>>,
        /// The service name. For AWS services the service name is usually in the form `com.amazonaws.<region>.<service>` (the SageMaker Notebook service is an exception to this rule, the service name is in the form `aws.sagemaker.<region>.notebook`).
        pub service_name: pulumi_gestalt_rust::Output<String>,
        /// The AWS region of the VPC Endpoint Service. If specified, the VPC endpoint will connect to the service in the provided region. Applicable for endpoints of type `Interface`.
        pub service_region: pulumi_gestalt_rust::Output<String>,
        /// The state of the VPC endpoint.
        pub state: pulumi_gestalt_rust::Output<String>,
        /// Subnet configuration for the endpoint, used to select specific IPv4 and/or IPv6 addresses to the endpoint. See subnet_configuration below.
        pub subnet_configurations: pulumi_gestalt_rust::Output<
            Vec<super::super::types::ec2::VpcEndpointSubnetConfiguration>,
        >,
        /// The ID of one or more subnets in which to create a network interface for the endpoint. Applicable for endpoints of type `GatewayLoadBalancer` and `Interface`. Interface type endpoints cannot function without being assigned to a subnet.
        pub subnet_ids: pulumi_gestalt_rust::Output<Vec<String>>,
        /// A map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The VPC endpoint type, `Gateway`, `GatewayLoadBalancer`, or `Interface`. Defaults to `Gateway`.
        pub vpc_endpoint_type: pulumi_gestalt_rust::Output<Option<String>>,
        /// The ID of the VPC in which the endpoint will be used.
        pub vpc_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: VpcEndpointArgs,
    ) -> VpcEndpointResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let auto_accept_binding = args.auto_accept.get_output(context);
        let dns_options_binding = args.dns_options.get_output(context);
        let ip_address_type_binding = args.ip_address_type.get_output(context);
        let policy_binding = args.policy.get_output(context);
        let private_dns_enabled_binding = args.private_dns_enabled.get_output(context);
        let route_table_ids_binding = args.route_table_ids.get_output(context);
        let security_group_ids_binding = args.security_group_ids.get_output(context);
        let service_name_binding = args.service_name.get_output(context);
        let service_region_binding = args.service_region.get_output(context);
        let subnet_configurations_binding = args
            .subnet_configurations
            .get_output(context);
        let subnet_ids_binding = args.subnet_ids.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let vpc_endpoint_type_binding = args.vpc_endpoint_type.get_output(context);
        let vpc_id_binding = args.vpc_id.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:ec2/vpcEndpoint:VpcEndpoint".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "autoAccept".into(),
                    value: auto_accept_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "dnsOptions".into(),
                    value: dns_options_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "ipAddressType".into(),
                    value: ip_address_type_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "policy".into(),
                    value: policy_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "privateDnsEnabled".into(),
                    value: private_dns_enabled_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "routeTableIds".into(),
                    value: route_table_ids_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "securityGroupIds".into(),
                    value: security_group_ids_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "serviceName".into(),
                    value: service_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "serviceRegion".into(),
                    value: service_region_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "subnetConfigurations".into(),
                    value: subnet_configurations_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "subnetIds".into(),
                    value: subnet_ids_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "vpcEndpointType".into(),
                    value: vpc_endpoint_type_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "vpcId".into(),
                    value: vpc_id_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        VpcEndpointResult {
            arn: o.get_field("arn"),
            auto_accept: o.get_field("autoAccept"),
            cidr_blocks: o.get_field("cidrBlocks"),
            dns_entries: o.get_field("dnsEntries"),
            dns_options: o.get_field("dnsOptions"),
            ip_address_type: o.get_field("ipAddressType"),
            network_interface_ids: o.get_field("networkInterfaceIds"),
            owner_id: o.get_field("ownerId"),
            policy: o.get_field("policy"),
            prefix_list_id: o.get_field("prefixListId"),
            private_dns_enabled: o.get_field("privateDnsEnabled"),
            requester_managed: o.get_field("requesterManaged"),
            route_table_ids: o.get_field("routeTableIds"),
            security_group_ids: o.get_field("securityGroupIds"),
            service_name: o.get_field("serviceName"),
            service_region: o.get_field("serviceRegion"),
            state: o.get_field("state"),
            subnet_configurations: o.get_field("subnetConfigurations"),
            subnet_ids: o.get_field("subnetIds"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
            vpc_endpoint_type: o.get_field("vpcEndpointType"),
            vpc_id: o.get_field("vpcId"),
        }
    }
}
