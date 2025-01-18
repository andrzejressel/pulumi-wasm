pub mod get_vpc_endpoint_service {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetVpcEndpointServiceArgs {
        /// Configuration block(s) for filtering. Detailed below.
        #[builder(into, default)]
        pub filters: pulumi_wasm_rust::Output<
            Option<Vec<super::super::super::types::ec2::GetVpcEndpointServiceFilter>>,
        >,
        /// Common name of an AWS service (e.g., `s3`).
        #[builder(into, default)]
        pub service: pulumi_wasm_rust::Output<Option<String>>,
        /// Service name that is specified when creating a VPC endpoint. For AWS services the service name is usually in the form `com.amazonaws.<region>.<service>` (the SageMaker Notebook service is an exception to this rule, the service name is in the form `aws.sagemaker.<region>.notebook`).
        #[builder(into, default)]
        pub service_name: pulumi_wasm_rust::Output<Option<String>>,
        /// Service type, `Gateway` or `Interface`.
        #[builder(into, default)]
        pub service_type: pulumi_wasm_rust::Output<Option<String>>,
        /// Map of tags, each pair of which must exactly match a pair on the desired VPC Endpoint Service.
        ///
        /// > **NOTE:** Specifying `service` will not work for non-AWS services or AWS services that don't follow the standard `service_name` pattern of `com.amazonaws.<region>.<service>`.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct GetVpcEndpointServiceResult {
        /// Whether or not VPC endpoint connection requests to the service must be accepted by the service owner - `true` or `false`.
        pub acceptance_required: pulumi_wasm_rust::Output<bool>,
        /// ARN of the VPC endpoint service.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// Availability Zones in which the service is available.
        pub availability_zones: pulumi_wasm_rust::Output<Vec<String>>,
        /// The DNS names for the service.
        pub base_endpoint_dns_names: pulumi_wasm_rust::Output<Vec<String>>,
        pub filters: pulumi_wasm_rust::Output<
            Option<Vec<super::super::super::types::ec2::GetVpcEndpointServiceFilter>>,
        >,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        /// Whether or not the service manages its VPC endpoints - `true` or `false`.
        pub manages_vpc_endpoints: pulumi_wasm_rust::Output<bool>,
        /// AWS account ID of the service owner or `amazon`.
        pub owner: pulumi_wasm_rust::Output<String>,
        /// Private DNS name for the service.
        pub private_dns_name: pulumi_wasm_rust::Output<String>,
        /// Private DNS names assigned to the VPC endpoint service.
        pub private_dns_names: pulumi_wasm_rust::Output<Vec<String>>,
        pub service: pulumi_wasm_rust::Output<Option<String>>,
        /// ID of the endpoint service.
        pub service_id: pulumi_wasm_rust::Output<String>,
        pub service_name: pulumi_wasm_rust::Output<String>,
        pub service_type: pulumi_wasm_rust::Output<String>,
        /// The supported IP address types.
        pub supported_ip_address_types: pulumi_wasm_rust::Output<Vec<String>>,
        /// Map of tags assigned to the resource.
        pub tags: pulumi_wasm_rust::Output<std::collections::HashMap<String, String>>,
        /// Whether or not the service supports endpoint policies - `true` or `false`.
        pub vpc_endpoint_policy_supported: pulumi_wasm_rust::Output<bool>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(args: GetVpcEndpointServiceArgs) -> GetVpcEndpointServiceResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let filters_binding = args.filters.get_inner();
        let service_binding = args.service.get_inner();
        let service_name_binding = args.service_name.get_inner();
        let service_type_binding = args.service_type.get_inner();
        let tags_binding = args.tags.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:ec2/getVpcEndpointService:getVpcEndpointService".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "filters".into(),
                    value: &filters_binding,
                },
                register_interface::ObjectField {
                    name: "service".into(),
                    value: &service_binding,
                },
                register_interface::ObjectField {
                    name: "serviceName".into(),
                    value: &service_name_binding,
                },
                register_interface::ObjectField {
                    name: "serviceType".into(),
                    value: &service_type_binding,
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
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "availabilityZones".into(),
                },
                register_interface::ResultField {
                    name: "baseEndpointDnsNames".into(),
                },
                register_interface::ResultField {
                    name: "filters".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "managesVpcEndpoints".into(),
                },
                register_interface::ResultField {
                    name: "owner".into(),
                },
                register_interface::ResultField {
                    name: "privateDnsName".into(),
                },
                register_interface::ResultField {
                    name: "privateDnsNames".into(),
                },
                register_interface::ResultField {
                    name: "service".into(),
                },
                register_interface::ResultField {
                    name: "serviceId".into(),
                },
                register_interface::ResultField {
                    name: "serviceName".into(),
                },
                register_interface::ResultField {
                    name: "serviceType".into(),
                },
                register_interface::ResultField {
                    name: "supportedIpAddressTypes".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "vpcEndpointPolicySupported".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetVpcEndpointServiceResult {
            acceptance_required: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("acceptanceRequired").unwrap(),
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
            filters: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("filters").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            manages_vpc_endpoints: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("managesVpcEndpoints").unwrap(),
            ),
            owner: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("owner").unwrap(),
            ),
            private_dns_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("privateDnsName").unwrap(),
            ),
            private_dns_names: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("privateDnsNames").unwrap(),
            ),
            service: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("service").unwrap(),
            ),
            service_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("serviceId").unwrap(),
            ),
            service_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("serviceName").unwrap(),
            ),
            service_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("serviceType").unwrap(),
            ),
            supported_ip_address_types: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("supportedIpAddressTypes").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            vpc_endpoint_policy_supported: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("vpcEndpointPolicySupported").unwrap(),
            ),
        }
    }
}
