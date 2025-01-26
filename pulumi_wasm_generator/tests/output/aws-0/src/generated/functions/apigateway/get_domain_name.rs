pub mod get_domain_name {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetDomainNameArgs {
        /// Fully-qualified domain name to look up. If no domain name is found, an error will be returned.
        #[builder(into)]
        pub domain_name: pulumi_wasm_rust::InputOrOutput<String>,
        /// The identifier for the domain name resource. Supported only for private custom domain names.
        #[builder(into, default)]
        pub domain_name_id: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Key-value map of tags for the resource.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct GetDomainNameResult {
        /// ARN of the found custom domain name.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// ARN for an AWS-managed certificate that is used by edge-optimized endpoint for this domain name.
        pub certificate_arn: pulumi_wasm_rust::Output<String>,
        /// Name of the certificate that is used by edge-optimized endpoint for this domain name.
        pub certificate_name: pulumi_wasm_rust::Output<String>,
        /// Upload date associated with the domain certificate.
        pub certificate_upload_date: pulumi_wasm_rust::Output<String>,
        /// Hostname created by Cloudfront to represent the distribution that implements this domain name mapping.
        pub cloudfront_domain_name: pulumi_wasm_rust::Output<String>,
        /// For convenience, the hosted zone ID (`Z2FDTNDATAQYW2`) that can be used to create a Route53 alias record for the distribution.
        pub cloudfront_zone_id: pulumi_wasm_rust::Output<String>,
        pub domain_name: pulumi_wasm_rust::Output<String>,
        pub domain_name_id: pulumi_wasm_rust::Output<String>,
        /// List of objects with the endpoint configuration of this domain name.
        pub endpoint_configurations: pulumi_wasm_rust::Output<
            Vec<
                super::super::super::types::apigateway::GetDomainNameEndpointConfiguration,
            >,
        >,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        /// A stringified JSON policy document that applies to the execute-api service for this DomainName regardless of the caller and Method configuration. Supported only for private custom domain names.
        pub policy: pulumi_wasm_rust::Output<String>,
        /// ARN for an AWS-managed certificate that is used for validating the regional domain name.
        pub regional_certificate_arn: pulumi_wasm_rust::Output<String>,
        /// User-friendly name of the certificate that is used by regional endpoint for this domain name.
        pub regional_certificate_name: pulumi_wasm_rust::Output<String>,
        /// Hostname for the custom domain's regional endpoint.
        pub regional_domain_name: pulumi_wasm_rust::Output<String>,
        /// Hosted zone ID that can be used to create a Route53 alias record for the regional endpoint.
        pub regional_zone_id: pulumi_wasm_rust::Output<String>,
        /// Security policy for the domain name.
        pub security_policy: pulumi_wasm_rust::Output<String>,
        /// Key-value map of tags for the resource.
        pub tags: pulumi_wasm_rust::Output<std::collections::HashMap<String, String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetDomainNameArgs,
    ) -> GetDomainNameResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let domain_name_binding = args.domain_name.get_output(context).get_inner();
        let domain_name_id_binding = args.domain_name_id.get_output(context).get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:apigateway/getDomainName:getDomainName".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "domainName".into(),
                    value: &domain_name_binding,
                },
                register_interface::ObjectField {
                    name: "domainNameId".into(),
                    value: &domain_name_id_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "certificateArn".into(),
                },
                register_interface::ResultField {
                    name: "certificateName".into(),
                },
                register_interface::ResultField {
                    name: "certificateUploadDate".into(),
                },
                register_interface::ResultField {
                    name: "cloudfrontDomainName".into(),
                },
                register_interface::ResultField {
                    name: "cloudfrontZoneId".into(),
                },
                register_interface::ResultField {
                    name: "domainName".into(),
                },
                register_interface::ResultField {
                    name: "domainNameId".into(),
                },
                register_interface::ResultField {
                    name: "endpointConfigurations".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "policy".into(),
                },
                register_interface::ResultField {
                    name: "regionalCertificateArn".into(),
                },
                register_interface::ResultField {
                    name: "regionalCertificateName".into(),
                },
                register_interface::ResultField {
                    name: "regionalDomainName".into(),
                },
                register_interface::ResultField {
                    name: "regionalZoneId".into(),
                },
                register_interface::ResultField {
                    name: "securityPolicy".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetDomainNameResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            certificate_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("certificateArn").unwrap(),
            ),
            certificate_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("certificateName").unwrap(),
            ),
            certificate_upload_date: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("certificateUploadDate").unwrap(),
            ),
            cloudfront_domain_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("cloudfrontDomainName").unwrap(),
            ),
            cloudfront_zone_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("cloudfrontZoneId").unwrap(),
            ),
            domain_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("domainName").unwrap(),
            ),
            domain_name_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("domainNameId").unwrap(),
            ),
            endpoint_configurations: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("endpointConfigurations").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            policy: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("policy").unwrap(),
            ),
            regional_certificate_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("regionalCertificateArn").unwrap(),
            ),
            regional_certificate_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("regionalCertificateName").unwrap(),
            ),
            regional_domain_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("regionalDomainName").unwrap(),
            ),
            regional_zone_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("regionalZoneId").unwrap(),
            ),
            security_policy: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("securityPolicy").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
        }
    }
}
