/// Resource for managing an AWS EC2 (Elastic Compute Cloud) Verified Access Endpoint.
///
/// ## Example Usage
///
/// ### ALB Example
///
///
/// ### Network Interface Example
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = endpoint::create(
///         "example",
///         EndpointArgs::builder()
///             .application_domain("example.com")
///             .attachment_type("vpc")
///             .description("example")
///             .domain_certificate_arn("${exampleAwsAcmCertificate.arn}")
///             .endpoint_domain_prefix("example")
///             .endpoint_type("network-interface")
///             .network_interface_options(
///                 EndpointNetworkInterfaceOptions::builder()
///                     .networkInterfaceId("${exampleAwsNetworkInterface.id}")
///                     .port(443)
///                     .protocol("https")
///                     .build_struct(),
///             )
///             .security_group_ids(vec!["${exampleAwsSecurityGroup.id}",])
///             .verified_access_group_id("${exampleAwsVerifiedaccessGroup.id}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Verified Access Instances using the  `id`. For example:
///
/// ```sh
/// $ pulumi import aws:verifiedaccess/endpoint:Endpoint example vae-8012925589
/// ```
pub mod endpoint {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct EndpointArgs {
        /// The DNS name for users to reach your application.
        #[builder(into)]
        pub application_domain: pulumi_wasm_rust::Output<String>,
        /// The type of attachment. Currently, only `vpc` is supported.
        #[builder(into)]
        pub attachment_type: pulumi_wasm_rust::Output<String>,
        /// A description for the Verified Access endpoint.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// The ARN of the public TLS/SSL certificate in AWS Certificate Manager to associate with the endpoint. The CN in the certificate must match the DNS name your end users will use to reach your application.
        #[builder(into)]
        pub domain_certificate_arn: pulumi_wasm_rust::Output<String>,
        /// A custom identifier that is prepended to the DNS name that is generated for the endpoint.
        #[builder(into)]
        pub endpoint_domain_prefix: pulumi_wasm_rust::Output<String>,
        /// The type of Verified Access endpoint to create. Currently `load-balancer` or `network-interface` are supported.
        #[builder(into)]
        pub endpoint_type: pulumi_wasm_rust::Output<String>,
        /// The load balancer details. This parameter is required if the endpoint type is `load-balancer`.
        #[builder(into, default)]
        pub load_balancer_options: pulumi_wasm_rust::Output<
            Option<super::super::types::verifiedaccess::EndpointLoadBalancerOptions>,
        >,
        /// The network interface details. This parameter is required if the endpoint type is `network-interface`.
        #[builder(into, default)]
        pub network_interface_options: pulumi_wasm_rust::Output<
            Option<super::super::types::verifiedaccess::EndpointNetworkInterfaceOptions>,
        >,
        /// The policy document that is associated with this resource.
        #[builder(into, default)]
        pub policy_document: pulumi_wasm_rust::Output<Option<String>>,
        /// List of the the security groups IDs to associate with the Verified Access endpoint.
        #[builder(into, default)]
        pub security_group_ids: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// The options in use for server side encryption.
        #[builder(into, default)]
        pub sse_specification: pulumi_wasm_rust::Output<
            Option<super::super::types::verifiedaccess::EndpointSseSpecification>,
        >,
        /// Key-value tags for the Verified Access Endpoint. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The ID of the Verified Access group to associate the endpoint with.
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub verified_access_group_id: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct EndpointResult {
        /// The DNS name for users to reach your application.
        pub application_domain: pulumi_wasm_rust::Output<String>,
        /// The type of attachment. Currently, only `vpc` is supported.
        pub attachment_type: pulumi_wasm_rust::Output<String>,
        /// A description for the Verified Access endpoint.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// Returned if endpoint has a device trust provider attached.
        pub device_validation_domain: pulumi_wasm_rust::Output<String>,
        /// The ARN of the public TLS/SSL certificate in AWS Certificate Manager to associate with the endpoint. The CN in the certificate must match the DNS name your end users will use to reach your application.
        pub domain_certificate_arn: pulumi_wasm_rust::Output<String>,
        /// A DNS name that is generated for the endpoint.
        pub endpoint_domain: pulumi_wasm_rust::Output<String>,
        /// A custom identifier that is prepended to the DNS name that is generated for the endpoint.
        pub endpoint_domain_prefix: pulumi_wasm_rust::Output<String>,
        /// The type of Verified Access endpoint to create. Currently `load-balancer` or `network-interface` are supported.
        pub endpoint_type: pulumi_wasm_rust::Output<String>,
        /// The load balancer details. This parameter is required if the endpoint type is `load-balancer`.
        pub load_balancer_options: pulumi_wasm_rust::Output<
            Option<super::super::types::verifiedaccess::EndpointLoadBalancerOptions>,
        >,
        /// The network interface details. This parameter is required if the endpoint type is `network-interface`.
        pub network_interface_options: pulumi_wasm_rust::Output<
            Option<super::super::types::verifiedaccess::EndpointNetworkInterfaceOptions>,
        >,
        /// The policy document that is associated with this resource.
        pub policy_document: pulumi_wasm_rust::Output<Option<String>>,
        /// List of the the security groups IDs to associate with the Verified Access endpoint.
        pub security_group_ids: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// The options in use for server side encryption.
        pub sse_specification: pulumi_wasm_rust::Output<
            super::super::types::verifiedaccess::EndpointSseSpecification,
        >,
        /// Key-value tags for the Verified Access Endpoint. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The ID of the Verified Access group to associate the endpoint with.
        ///
        /// The following arguments are optional:
        pub verified_access_group_id: pulumi_wasm_rust::Output<String>,
        pub verified_access_instance_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: EndpointArgs) -> EndpointResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let application_domain_binding = args.application_domain.get_inner();
        let attachment_type_binding = args.attachment_type.get_inner();
        let description_binding = args.description.get_inner();
        let domain_certificate_arn_binding = args.domain_certificate_arn.get_inner();
        let endpoint_domain_prefix_binding = args.endpoint_domain_prefix.get_inner();
        let endpoint_type_binding = args.endpoint_type.get_inner();
        let load_balancer_options_binding = args.load_balancer_options.get_inner();
        let network_interface_options_binding = args
            .network_interface_options
            .get_inner();
        let policy_document_binding = args.policy_document.get_inner();
        let security_group_ids_binding = args.security_group_ids.get_inner();
        let sse_specification_binding = args.sse_specification.get_inner();
        let tags_binding = args.tags.get_inner();
        let verified_access_group_id_binding = args.verified_access_group_id.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:verifiedaccess/endpoint:Endpoint".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "applicationDomain".into(),
                    value: &application_domain_binding,
                },
                register_interface::ObjectField {
                    name: "attachmentType".into(),
                    value: &attachment_type_binding,
                },
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "domainCertificateArn".into(),
                    value: &domain_certificate_arn_binding,
                },
                register_interface::ObjectField {
                    name: "endpointDomainPrefix".into(),
                    value: &endpoint_domain_prefix_binding,
                },
                register_interface::ObjectField {
                    name: "endpointType".into(),
                    value: &endpoint_type_binding,
                },
                register_interface::ObjectField {
                    name: "loadBalancerOptions".into(),
                    value: &load_balancer_options_binding,
                },
                register_interface::ObjectField {
                    name: "networkInterfaceOptions".into(),
                    value: &network_interface_options_binding,
                },
                register_interface::ObjectField {
                    name: "policyDocument".into(),
                    value: &policy_document_binding,
                },
                register_interface::ObjectField {
                    name: "securityGroupIds".into(),
                    value: &security_group_ids_binding,
                },
                register_interface::ObjectField {
                    name: "sseSpecification".into(),
                    value: &sse_specification_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "verifiedAccessGroupId".into(),
                    value: &verified_access_group_id_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "applicationDomain".into(),
                },
                register_interface::ResultField {
                    name: "attachmentType".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "deviceValidationDomain".into(),
                },
                register_interface::ResultField {
                    name: "domainCertificateArn".into(),
                },
                register_interface::ResultField {
                    name: "endpointDomain".into(),
                },
                register_interface::ResultField {
                    name: "endpointDomainPrefix".into(),
                },
                register_interface::ResultField {
                    name: "endpointType".into(),
                },
                register_interface::ResultField {
                    name: "loadBalancerOptions".into(),
                },
                register_interface::ResultField {
                    name: "networkInterfaceOptions".into(),
                },
                register_interface::ResultField {
                    name: "policyDocument".into(),
                },
                register_interface::ResultField {
                    name: "securityGroupIds".into(),
                },
                register_interface::ResultField {
                    name: "sseSpecification".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "tagsAll".into(),
                },
                register_interface::ResultField {
                    name: "verifiedAccessGroupId".into(),
                },
                register_interface::ResultField {
                    name: "verifiedAccessInstanceId".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        EndpointResult {
            application_domain: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("applicationDomain").unwrap(),
            ),
            attachment_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("attachmentType").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            device_validation_domain: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("deviceValidationDomain").unwrap(),
            ),
            domain_certificate_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("domainCertificateArn").unwrap(),
            ),
            endpoint_domain: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("endpointDomain").unwrap(),
            ),
            endpoint_domain_prefix: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("endpointDomainPrefix").unwrap(),
            ),
            endpoint_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("endpointType").unwrap(),
            ),
            load_balancer_options: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("loadBalancerOptions").unwrap(),
            ),
            network_interface_options: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("networkInterfaceOptions").unwrap(),
            ),
            policy_document: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("policyDocument").unwrap(),
            ),
            security_group_ids: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("securityGroupIds").unwrap(),
            ),
            sse_specification: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("sseSpecification").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            tags_all: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tagsAll").unwrap(),
            ),
            verified_access_group_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("verifiedAccessGroupId").unwrap(),
            ),
            verified_access_instance_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("verifiedAccessInstanceId").unwrap(),
            ),
        }
    }
}
