#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_domain_name {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetDomainNameArgs {
        /// Fully-qualified domain name to look up. If no domain name is found, an error will be returned.
        #[builder(into)]
        pub domain_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The identifier for the domain name resource. Supported only for private custom domain names.
        #[builder(into, default)]
        pub domain_name_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Key-value map of tags for the resource.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct GetDomainNameResult {
        /// ARN of the found custom domain name.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// ARN for an AWS-managed certificate that is used by edge-optimized endpoint for this domain name.
        pub certificate_arn: pulumi_gestalt_rust::Output<String>,
        /// Name of the certificate that is used by edge-optimized endpoint for this domain name.
        pub certificate_name: pulumi_gestalt_rust::Output<String>,
        /// Upload date associated with the domain certificate.
        pub certificate_upload_date: pulumi_gestalt_rust::Output<String>,
        /// Hostname created by Cloudfront to represent the distribution that implements this domain name mapping.
        pub cloudfront_domain_name: pulumi_gestalt_rust::Output<String>,
        /// For convenience, the hosted zone ID (`Z2FDTNDATAQYW2`) that can be used to create a Route53 alias record for the distribution.
        pub cloudfront_zone_id: pulumi_gestalt_rust::Output<String>,
        pub domain_name: pulumi_gestalt_rust::Output<String>,
        pub domain_name_id: pulumi_gestalt_rust::Output<String>,
        /// List of objects with the endpoint configuration of this domain name.
        pub endpoint_configurations: pulumi_gestalt_rust::Output<
            Vec<
                super::super::super::types::apigateway::GetDomainNameEndpointConfiguration,
            >,
        >,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// A stringified JSON policy document that applies to the execute-api service for this DomainName regardless of the caller and Method configuration. Supported only for private custom domain names.
        pub policy: pulumi_gestalt_rust::Output<String>,
        /// ARN for an AWS-managed certificate that is used for validating the regional domain name.
        pub regional_certificate_arn: pulumi_gestalt_rust::Output<String>,
        /// User-friendly name of the certificate that is used by regional endpoint for this domain name.
        pub regional_certificate_name: pulumi_gestalt_rust::Output<String>,
        /// Hostname for the custom domain's regional endpoint.
        pub regional_domain_name: pulumi_gestalt_rust::Output<String>,
        /// Hosted zone ID that can be used to create a Route53 alias record for the regional endpoint.
        pub regional_zone_id: pulumi_gestalt_rust::Output<String>,
        /// Security policy for the domain name.
        pub security_policy: pulumi_gestalt_rust::Output<String>,
        /// Key-value map of tags for the resource.
        pub tags: pulumi_gestalt_rust::Output<std::collections::HashMap<String, String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetDomainNameArgs,
    ) -> GetDomainNameResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let domain_name_binding = args.domain_name.get_output(context);
        let domain_name_id_binding = args.domain_name_id.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "aws:apigateway/getDomainName:getDomainName".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "domainName".into(),
                    value: domain_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "domainNameId".into(),
                    value: domain_name_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetDomainNameResult {
            arn: o.get_field("arn"),
            certificate_arn: o.get_field("certificateArn"),
            certificate_name: o.get_field("certificateName"),
            certificate_upload_date: o.get_field("certificateUploadDate"),
            cloudfront_domain_name: o.get_field("cloudfrontDomainName"),
            cloudfront_zone_id: o.get_field("cloudfrontZoneId"),
            domain_name: o.get_field("domainName"),
            domain_name_id: o.get_field("domainNameId"),
            endpoint_configurations: o.get_field("endpointConfigurations"),
            id: o.get_field("id"),
            policy: o.get_field("policy"),
            regional_certificate_arn: o.get_field("regionalCertificateArn"),
            regional_certificate_name: o.get_field("regionalCertificateName"),
            regional_domain_name: o.get_field("regionalDomainName"),
            regional_zone_id: o.get_field("regionalZoneId"),
            security_policy: o.get_field("securityPolicy"),
            tags: o.get_field("tags"),
        }
    }
}
