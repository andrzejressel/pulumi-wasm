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
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
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
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
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
pub mod vpc_endpoint_service {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct VpcEndpointServiceArgs {
        /// Whether or not VPC endpoint connection requests to the service must be accepted by the service owner - `true` or `false`.
        #[builder(into)]
        pub acceptance_required: pulumi_wasm_rust::Output<bool>,
        /// The ARNs of one or more principals allowed to discover the endpoint service.
        #[builder(into, default)]
        pub allowed_principals: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// Amazon Resource Names (ARNs) of one or more Gateway Load Balancers for the endpoint service.
        #[builder(into, default)]
        pub gateway_load_balancer_arns: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// Amazon Resource Names (ARNs) of one or more Network Load Balancers for the endpoint service.
        #[builder(into, default)]
        pub network_load_balancer_arns: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// The private DNS name for the service.
        #[builder(into, default)]
        pub private_dns_name: pulumi_wasm_rust::Output<Option<String>>,
        /// The supported IP address types. The possible values are `ipv4` and `ipv6`.
        #[builder(into, default)]
        pub supported_ip_address_types: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// A map of tags to assign to the resource. .If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct VpcEndpointServiceResult {
        /// Whether or not VPC endpoint connection requests to the service must be accepted by the service owner - `true` or `false`.
        pub acceptance_required: pulumi_wasm_rust::Output<bool>,
        /// The ARNs of one or more principals allowed to discover the endpoint service.
        pub allowed_principals: pulumi_wasm_rust::Output<Vec<String>>,
        /// The Amazon Resource Name (ARN) of the VPC endpoint service.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// A set of Availability Zones in which the service is available.
        pub availability_zones: pulumi_wasm_rust::Output<Vec<String>>,
        /// A set of DNS names for the service.
        pub base_endpoint_dns_names: pulumi_wasm_rust::Output<Vec<String>>,
        /// Amazon Resource Names (ARNs) of one or more Gateway Load Balancers for the endpoint service.
        pub gateway_load_balancer_arns: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// Whether or not the service manages its VPC endpoints - `true` or `false`.
        pub manages_vpc_endpoints: pulumi_wasm_rust::Output<bool>,
        /// Amazon Resource Names (ARNs) of one or more Network Load Balancers for the endpoint service.
        pub network_load_balancer_arns: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// The private DNS name for the service.
        pub private_dns_name: pulumi_wasm_rust::Output<String>,
        /// List of objects containing information about the endpoint service private DNS name configuration.
        pub private_dns_name_configurations: pulumi_wasm_rust::Output<
            Vec<super::super::types::ec2::VpcEndpointServicePrivateDnsNameConfiguration>,
        >,
        /// The service name.
        pub service_name: pulumi_wasm_rust::Output<String>,
        /// The service type, `Gateway` or `Interface`.
        pub service_type: pulumi_wasm_rust::Output<String>,
        /// Verification state of the VPC endpoint service. Consumers of the endpoint service can use the private name only when the state is `verified`.
        pub state: pulumi_wasm_rust::Output<String>,
        /// The supported IP address types. The possible values are `ipv4` and `ipv6`.
        pub supported_ip_address_types: pulumi_wasm_rust::Output<Vec<String>>,
        /// A map of tags to assign to the resource. .If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: VpcEndpointServiceArgs) -> VpcEndpointServiceResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let acceptance_required_binding = args.acceptance_required.get_inner();
        let allowed_principals_binding = args.allowed_principals.get_inner();
        let gateway_load_balancer_arns_binding = args
            .gateway_load_balancer_arns
            .get_inner();
        let network_load_balancer_arns_binding = args
            .network_load_balancer_arns
            .get_inner();
        let private_dns_name_binding = args.private_dns_name.get_inner();
        let supported_ip_address_types_binding = args
            .supported_ip_address_types
            .get_inner();
        let tags_binding = args.tags.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:ec2/vpcEndpointService:VpcEndpointService".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "acceptanceRequired".into(),
                    value: &acceptance_required_binding,
                },
                register_interface::ObjectField {
                    name: "allowedPrincipals".into(),
                    value: &allowed_principals_binding,
                },
                register_interface::ObjectField {
                    name: "gatewayLoadBalancerArns".into(),
                    value: &gateway_load_balancer_arns_binding,
                },
                register_interface::ObjectField {
                    name: "networkLoadBalancerArns".into(),
                    value: &network_load_balancer_arns_binding,
                },
                register_interface::ObjectField {
                    name: "privateDnsName".into(),
                    value: &private_dns_name_binding,
                },
                register_interface::ObjectField {
                    name: "supportedIpAddressTypes".into(),
                    value: &supported_ip_address_types_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "acceptanceRequired".into(),
                },
                register_interface::ResultField {
                    name: "allowedPrincipals".into(),
                },
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "availabilityZones".into(),
                },
                register_interface::ResultField {
                    name: "baseEndpointDnsNames".into(),
                },
                register_interface::ResultField {
                    name: "gatewayLoadBalancerArns".into(),
                },
                register_interface::ResultField {
                    name: "managesVpcEndpoints".into(),
                },
                register_interface::ResultField {
                    name: "networkLoadBalancerArns".into(),
                },
                register_interface::ResultField {
                    name: "privateDnsName".into(),
                },
                register_interface::ResultField {
                    name: "privateDnsNameConfigurations".into(),
                },
                register_interface::ResultField {
                    name: "serviceName".into(),
                },
                register_interface::ResultField {
                    name: "serviceType".into(),
                },
                register_interface::ResultField {
                    name: "state".into(),
                },
                register_interface::ResultField {
                    name: "supportedIpAddressTypes".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "tagsAll".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        VpcEndpointServiceResult {
            acceptance_required: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("acceptanceRequired").unwrap(),
            ),
            allowed_principals: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("allowedPrincipals").unwrap(),
            ),
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            availability_zones: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("availabilityZones").unwrap(),
            ),
            base_endpoint_dns_names: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("baseEndpointDnsNames").unwrap(),
            ),
            gateway_load_balancer_arns: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("gatewayLoadBalancerArns").unwrap(),
            ),
            manages_vpc_endpoints: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("managesVpcEndpoints").unwrap(),
            ),
            network_load_balancer_arns: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("networkLoadBalancerArns").unwrap(),
            ),
            private_dns_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("privateDnsName").unwrap(),
            ),
            private_dns_name_configurations: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("privateDnsNameConfigurations").unwrap(),
            ),
            service_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("serviceName").unwrap(),
            ),
            service_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("serviceType").unwrap(),
            ),
            state: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("state").unwrap(),
            ),
            supported_ip_address_types: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("supportedIpAddressTypes").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            tags_all: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tagsAll").unwrap(),
            ),
        }
    }
}