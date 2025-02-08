#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_vpc_endpoint_service {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetVpcEndpointServiceArgs {
        /// Configuration block(s) for filtering. Detailed below.
        #[builder(into, default)]
        pub filters: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::super::types::ec2::GetVpcEndpointServiceFilter>>,
        >,
        /// Common name of an AWS service (e.g., `s3`).
        #[builder(into, default)]
        pub service: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Service name that is specified when creating a VPC endpoint. For AWS services the service name is usually in the form `com.amazonaws.<region>.<service>` (the SageMaker Notebook service is an exception to this rule, the service name is in the form `aws.sagemaker.<region>.notebook`).
        #[builder(into, default)]
        pub service_name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Service type, `Gateway` or `Interface`.
        #[builder(into, default)]
        pub service_type: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Map of tags, each pair of which must exactly match a pair on the desired VPC Endpoint Service.
        ///
        /// > **NOTE:** Specifying `service` will not work for non-AWS services or AWS services that don't follow the standard `service_name` pattern of `com.amazonaws.<region>.<service>`.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct GetVpcEndpointServiceResult {
        /// Whether or not VPC endpoint connection requests to the service must be accepted by the service owner - `true` or `false`.
        pub acceptance_required: pulumi_gestalt_rust::Output<bool>,
        /// ARN of the VPC endpoint service.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// Availability Zones in which the service is available.
        pub availability_zones: pulumi_gestalt_rust::Output<Vec<String>>,
        /// The DNS names for the service.
        pub base_endpoint_dns_names: pulumi_gestalt_rust::Output<Vec<String>>,
        pub filters: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::super::types::ec2::GetVpcEndpointServiceFilter>>,
        >,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Whether or not the service manages its VPC endpoints - `true` or `false`.
        pub manages_vpc_endpoints: pulumi_gestalt_rust::Output<bool>,
        /// AWS account ID of the service owner or `amazon`.
        pub owner: pulumi_gestalt_rust::Output<String>,
        /// Private DNS name for the service.
        pub private_dns_name: pulumi_gestalt_rust::Output<String>,
        /// Private DNS names assigned to the VPC endpoint service.
        pub private_dns_names: pulumi_gestalt_rust::Output<Vec<String>>,
        pub service: pulumi_gestalt_rust::Output<Option<String>>,
        /// ID of the endpoint service.
        pub service_id: pulumi_gestalt_rust::Output<String>,
        pub service_name: pulumi_gestalt_rust::Output<String>,
        pub service_type: pulumi_gestalt_rust::Output<String>,
        /// The supported IP address types.
        pub supported_ip_address_types: pulumi_gestalt_rust::Output<Vec<String>>,
        /// Map of tags assigned to the resource.
        pub tags: pulumi_gestalt_rust::Output<std::collections::HashMap<String, String>>,
        /// Whether or not the service supports endpoint policies - `true` or `false`.
        pub vpc_endpoint_policy_supported: pulumi_gestalt_rust::Output<bool>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::PulumiContext,
        args: GetVpcEndpointServiceArgs,
    ) -> GetVpcEndpointServiceResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let filters_binding = args.filters.get_output(context).get_inner();
        let service_binding = args.service.get_output(context).get_inner();
        let service_name_binding = args.service_name.get_output(context).get_inner();
        let service_type_binding = args.service_type.get_output(context).get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
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
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetVpcEndpointServiceResult {
            acceptance_required: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("acceptanceRequired"),
            ),
            arn: pulumi_gestalt_rust::__private::into_domain(o.extract_field("arn")),
            availability_zones: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("availabilityZones"),
            ),
            base_endpoint_dns_names: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("baseEndpointDnsNames"),
            ),
            filters: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("filters"),
            ),
            id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("id")),
            manages_vpc_endpoints: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("managesVpcEndpoints"),
            ),
            owner: pulumi_gestalt_rust::__private::into_domain(o.extract_field("owner")),
            private_dns_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("privateDnsName"),
            ),
            private_dns_names: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("privateDnsNames"),
            ),
            service: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("service"),
            ),
            service_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("serviceId"),
            ),
            service_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("serviceName"),
            ),
            service_type: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("serviceType"),
            ),
            supported_ip_address_types: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("supportedIpAddressTypes"),
            ),
            tags: pulumi_gestalt_rust::__private::into_domain(o.extract_field("tags")),
            vpc_endpoint_policy_supported: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("vpcEndpointPolicySupported"),
            ),
        }
    }
}
