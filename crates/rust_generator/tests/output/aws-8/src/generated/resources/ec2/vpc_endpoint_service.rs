/// Provides a VPC Endpoint Service resource.
/// Service consumers can create an _Interface_ VPC Endpoint to connect to the service.
///
/// > **NOTE on VPC Endpoint Services and VPC Endpoint Service Allowed Principals:** This provider provides
/// both a standalone VPC Endpoint Service Allowed Principal resource
/// and a VPC Endpoint Service resource with an `allowed_principals` attribute. Do not use the same principal ARN in both
/// a VPC Endpoint Service resource and a VPC Endpoint Service Allowed Principal resource. Doing so will cause a conflict
/// and will overwrite the association.
///
/// ## Example Usage
///
/// ### Network Load Balancers
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = vpc_endpoint_service::create(
///         "example",
///         VpcEndpointServiceArgs::builder()
///             .acceptance_required(false)
///             .network_load_balancer_arns(vec!["${exampleAwsLb.arn}",])
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### Gateway Load Balancers
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = vpc_endpoint_service::create(
///         "example",
///         VpcEndpointServiceArgs::builder()
///             .acceptance_required(false)
///             .gateway_load_balancer_arns(vec!["${exampleAwsLb.arn}",])
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import VPC Endpoint Services using the VPC endpoint service `id`. For example:
///
/// ```sh
/// $ pulumi import aws:ec2/vpcEndpointService:VpcEndpointService foo vpce-svc-0f97a19d3fa8220bc
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod vpc_endpoint_service {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct VpcEndpointServiceArgs {
        /// Whether or not VPC endpoint connection requests to the service must be accepted by the service owner - `true` or `false`.
        #[builder(into)]
        pub acceptance_required: pulumi_gestalt_rust::InputOrOutput<bool>,
        /// The ARNs of one or more principals allowed to discover the endpoint service.
        #[builder(into, default)]
        pub allowed_principals: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// Amazon Resource Names (ARNs) of one or more Gateway Load Balancers for the endpoint service.
        #[builder(into, default)]
        pub gateway_load_balancer_arns: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<String>>,
        >,
        /// Amazon Resource Names (ARNs) of one or more Network Load Balancers for the endpoint service.
        #[builder(into, default)]
        pub network_load_balancer_arns: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<String>>,
        >,
        /// The private DNS name for the service.
        #[builder(into, default)]
        pub private_dns_name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The supported IP address types. The possible values are `ipv4` and `ipv6`.
        #[builder(into, default)]
        pub supported_ip_address_types: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<String>>,
        >,
        /// The set of regions from which service consumers can access the service.
        #[builder(into, default)]
        pub supported_regions: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// A map of tags to assign to the resource. .If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct VpcEndpointServiceResult {
        /// Whether or not VPC endpoint connection requests to the service must be accepted by the service owner - `true` or `false`.
        pub acceptance_required: pulumi_gestalt_rust::Output<bool>,
        /// The ARNs of one or more principals allowed to discover the endpoint service.
        pub allowed_principals: pulumi_gestalt_rust::Output<Vec<String>>,
        /// The Amazon Resource Name (ARN) of the VPC endpoint service.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// A set of Availability Zones in which the service is available.
        pub availability_zones: pulumi_gestalt_rust::Output<Vec<String>>,
        /// A set of DNS names for the service.
        pub base_endpoint_dns_names: pulumi_gestalt_rust::Output<Vec<String>>,
        /// Amazon Resource Names (ARNs) of one or more Gateway Load Balancers for the endpoint service.
        pub gateway_load_balancer_arns: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// Whether or not the service manages its VPC endpoints - `true` or `false`.
        pub manages_vpc_endpoints: pulumi_gestalt_rust::Output<bool>,
        /// Amazon Resource Names (ARNs) of one or more Network Load Balancers for the endpoint service.
        pub network_load_balancer_arns: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// The private DNS name for the service.
        pub private_dns_name: pulumi_gestalt_rust::Output<String>,
        /// List of objects containing information about the endpoint service private DNS name configuration.
        pub private_dns_name_configurations: pulumi_gestalt_rust::Output<
            Vec<super::super::types::ec2::VpcEndpointServicePrivateDnsNameConfiguration>,
        >,
        /// The service name.
        pub service_name: pulumi_gestalt_rust::Output<String>,
        /// The service type, `Gateway` or `Interface`.
        pub service_type: pulumi_gestalt_rust::Output<String>,
        /// Verification state of the VPC endpoint service. Consumers of the endpoint service can use the private name only when the state is `verified`.
        pub state: pulumi_gestalt_rust::Output<String>,
        /// The supported IP address types. The possible values are `ipv4` and `ipv6`.
        pub supported_ip_address_types: pulumi_gestalt_rust::Output<Vec<String>>,
        /// The set of regions from which service consumers can access the service.
        pub supported_regions: pulumi_gestalt_rust::Output<Vec<String>>,
        /// A map of tags to assign to the resource. .If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: VpcEndpointServiceArgs,
    ) -> VpcEndpointServiceResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let acceptance_required_binding = args.acceptance_required.get_output(context);
        let allowed_principals_binding = args.allowed_principals.get_output(context);
        let gateway_load_balancer_arns_binding = args
            .gateway_load_balancer_arns
            .get_output(context);
        let network_load_balancer_arns_binding = args
            .network_load_balancer_arns
            .get_output(context);
        let private_dns_name_binding = args.private_dns_name.get_output(context);
        let supported_ip_address_types_binding = args
            .supported_ip_address_types
            .get_output(context);
        let supported_regions_binding = args.supported_regions.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:ec2/vpcEndpointService:VpcEndpointService".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "acceptanceRequired".into(),
                    value: &acceptance_required_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "allowedPrincipals".into(),
                    value: &allowed_principals_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "gatewayLoadBalancerArns".into(),
                    value: &gateway_load_balancer_arns_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "networkLoadBalancerArns".into(),
                    value: &network_load_balancer_arns_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "privateDnsName".into(),
                    value: &private_dns_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "supportedIpAddressTypes".into(),
                    value: &supported_ip_address_types_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "supportedRegions".into(),
                    value: &supported_regions_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: &tags_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        VpcEndpointServiceResult {
            acceptance_required: o.get_field("acceptanceRequired"),
            allowed_principals: o.get_field("allowedPrincipals"),
            arn: o.get_field("arn"),
            availability_zones: o.get_field("availabilityZones"),
            base_endpoint_dns_names: o.get_field("baseEndpointDnsNames"),
            gateway_load_balancer_arns: o.get_field("gatewayLoadBalancerArns"),
            manages_vpc_endpoints: o.get_field("managesVpcEndpoints"),
            network_load_balancer_arns: o.get_field("networkLoadBalancerArns"),
            private_dns_name: o.get_field("privateDnsName"),
            private_dns_name_configurations: o.get_field("privateDnsNameConfigurations"),
            service_name: o.get_field("serviceName"),
            service_type: o.get_field("serviceType"),
            state: o.get_field("state"),
            supported_ip_address_types: o.get_field("supportedIpAddressTypes"),
            supported_regions: o.get_field("supportedRegions"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
        }
    }
}
