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
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod endpoint {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct EndpointArgs {
        /// The DNS name for users to reach your application.
        #[builder(into)]
        pub application_domain: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The type of attachment. Currently, only `vpc` is supported.
        #[builder(into)]
        pub attachment_type: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A description for the Verified Access endpoint.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ARN of the public TLS/SSL certificate in AWS Certificate Manager to associate with the endpoint. The CN in the certificate must match the DNS name your end users will use to reach your application.
        #[builder(into)]
        pub domain_certificate_arn: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A custom identifier that is prepended to the DNS name that is generated for the endpoint.
        #[builder(into)]
        pub endpoint_domain_prefix: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The type of Verified Access endpoint to create. Currently `load-balancer` or `network-interface` are supported.
        #[builder(into)]
        pub endpoint_type: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The load balancer details. This parameter is required if the endpoint type is `load-balancer`.
        #[builder(into, default)]
        pub load_balancer_options: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::verifiedaccess::EndpointLoadBalancerOptions>,
        >,
        /// The network interface details. This parameter is required if the endpoint type is `network-interface`.
        #[builder(into, default)]
        pub network_interface_options: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::verifiedaccess::EndpointNetworkInterfaceOptions>,
        >,
        /// The policy document that is associated with this resource.
        #[builder(into, default)]
        pub policy_document: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// List of the the security groups IDs to associate with the Verified Access endpoint.
        #[builder(into, default)]
        pub security_group_ids: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// The options in use for server side encryption.
        #[builder(into, default)]
        pub sse_specification: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::verifiedaccess::EndpointSseSpecification>,
        >,
        /// Key-value tags for the Verified Access Endpoint. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The ID of the Verified Access group to associate the endpoint with.
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub verified_access_group_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct EndpointResult {
        /// The DNS name for users to reach your application.
        pub application_domain: pulumi_gestalt_rust::Output<String>,
        /// The type of attachment. Currently, only `vpc` is supported.
        pub attachment_type: pulumi_gestalt_rust::Output<String>,
        /// A description for the Verified Access endpoint.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// Returned if endpoint has a device trust provider attached.
        pub device_validation_domain: pulumi_gestalt_rust::Output<String>,
        /// The ARN of the public TLS/SSL certificate in AWS Certificate Manager to associate with the endpoint. The CN in the certificate must match the DNS name your end users will use to reach your application.
        pub domain_certificate_arn: pulumi_gestalt_rust::Output<String>,
        /// A DNS name that is generated for the endpoint.
        pub endpoint_domain: pulumi_gestalt_rust::Output<String>,
        /// A custom identifier that is prepended to the DNS name that is generated for the endpoint.
        pub endpoint_domain_prefix: pulumi_gestalt_rust::Output<String>,
        /// The type of Verified Access endpoint to create. Currently `load-balancer` or `network-interface` are supported.
        pub endpoint_type: pulumi_gestalt_rust::Output<String>,
        /// The load balancer details. This parameter is required if the endpoint type is `load-balancer`.
        pub load_balancer_options: pulumi_gestalt_rust::Output<
            Option<super::super::types::verifiedaccess::EndpointLoadBalancerOptions>,
        >,
        /// The network interface details. This parameter is required if the endpoint type is `network-interface`.
        pub network_interface_options: pulumi_gestalt_rust::Output<
            Option<super::super::types::verifiedaccess::EndpointNetworkInterfaceOptions>,
        >,
        /// The policy document that is associated with this resource.
        pub policy_document: pulumi_gestalt_rust::Output<Option<String>>,
        /// List of the the security groups IDs to associate with the Verified Access endpoint.
        pub security_group_ids: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// The options in use for server side encryption.
        pub sse_specification: pulumi_gestalt_rust::Output<
            super::super::types::verifiedaccess::EndpointSseSpecification,
        >,
        /// Key-value tags for the Verified Access Endpoint. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The ID of the Verified Access group to associate the endpoint with.
        ///
        /// The following arguments are optional:
        pub verified_access_group_id: pulumi_gestalt_rust::Output<String>,
        pub verified_access_instance_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: EndpointArgs,
    ) -> EndpointResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let application_domain_binding = args.application_domain.get_output(context);
        let attachment_type_binding = args.attachment_type.get_output(context);
        let description_binding = args.description.get_output(context);
        let domain_certificate_arn_binding = args
            .domain_certificate_arn
            .get_output(context);
        let endpoint_domain_prefix_binding = args
            .endpoint_domain_prefix
            .get_output(context);
        let endpoint_type_binding = args.endpoint_type.get_output(context);
        let load_balancer_options_binding = args
            .load_balancer_options
            .get_output(context);
        let network_interface_options_binding = args
            .network_interface_options
            .get_output(context);
        let policy_document_binding = args.policy_document.get_output(context);
        let security_group_ids_binding = args.security_group_ids.get_output(context);
        let sse_specification_binding = args.sse_specification.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let verified_access_group_id_binding = args
            .verified_access_group_id
            .get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:verifiedaccess/endpoint:Endpoint".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "applicationDomain".into(),
                    value: &application_domain_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "attachmentType".into(),
                    value: &attachment_type_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: &description_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "domainCertificateArn".into(),
                    value: &domain_certificate_arn_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "endpointDomainPrefix".into(),
                    value: &endpoint_domain_prefix_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "endpointType".into(),
                    value: &endpoint_type_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "loadBalancerOptions".into(),
                    value: &load_balancer_options_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "networkInterfaceOptions".into(),
                    value: &network_interface_options_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "policyDocument".into(),
                    value: &policy_document_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "securityGroupIds".into(),
                    value: &security_group_ids_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "sseSpecification".into(),
                    value: &sse_specification_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: &tags_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "verifiedAccessGroupId".into(),
                    value: &verified_access_group_id_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        EndpointResult {
            application_domain: o.get_field("applicationDomain"),
            attachment_type: o.get_field("attachmentType"),
            description: o.get_field("description"),
            device_validation_domain: o.get_field("deviceValidationDomain"),
            domain_certificate_arn: o.get_field("domainCertificateArn"),
            endpoint_domain: o.get_field("endpointDomain"),
            endpoint_domain_prefix: o.get_field("endpointDomainPrefix"),
            endpoint_type: o.get_field("endpointType"),
            load_balancer_options: o.get_field("loadBalancerOptions"),
            network_interface_options: o.get_field("networkInterfaceOptions"),
            policy_document: o.get_field("policyDocument"),
            security_group_ids: o.get_field("securityGroupIds"),
            sse_specification: o.get_field("sseSpecification"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
            verified_access_group_id: o.get_field("verifiedAccessGroupId"),
            verified_access_instance_id: o.get_field("verifiedAccessInstanceId"),
        }
    }
}
