/// Registers a custom domain name for use with AWS API Gateway. Additional information about this functionality
/// can be found in the [API Gateway Developer Guide](https://docs.aws.amazon.com/apigateway/latest/developerguide/how-to-custom-domains.html).
///
/// This resource just establishes ownership of and the TLS settings for
/// a particular domain name. An API can be attached to a particular path
/// under the registered domain name using
/// the `aws.apigateway.BasePathMapping` resource.
///
/// API Gateway domains can be defined as either 'edge-optimized' or 'regional'.  In an edge-optimized configuration,
/// API Gateway internally creates and manages a CloudFront distribution to route requests on the given hostname. In
/// addition to this resource it's necessary to create a DNS record corresponding to the given domain name which is an alias
/// (either Route53 alias or traditional CNAME) to the Cloudfront domain name exported in the `cloudfront_domain_name`
/// attribute.
///
/// In a regional configuration, API Gateway does not create a CloudFront distribution to route requests to the API, though
/// a distribution can be created if needed. In either case, it is necessary to create a DNS record corresponding to the
/// given domain name which is an alias (either Route53 alias or traditional CNAME) to the regional domain name exported in
/// the `regional_domain_name` attribute.
///
/// > **Note:** API Gateway requires the use of AWS Certificate Manager (ACM) certificates instead of Identity and Access Management (IAM) certificates in regions that support ACM. Regions that support ACM can be found in the [Regions and Endpoints Documentation](https://docs.aws.amazon.com/general/latest/gr/rande.html#acm_region). To import an existing private key and certificate into ACM or request an ACM certificate, see the `aws.acm.Certificate` resource.
///
/// > **Note:** The `aws.apigateway.DomainName` resource expects dependency on the `aws.acm.CertificateValidation` as
/// only verified certificates can be used. This can be made either explicitly by adding the
/// `depends_on = [aws_acm_certificate_validation.cert]` attribute. Or implicitly by referring certificate ARN
/// from the validation resource where it will be available after the resource creation:
/// `regional_certificate_arn = aws_acm_certificate_validation.cert.certificate_arn`.
///
/// ## Example Usage
///
/// ### Edge Optimized (ACM Certificate)
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = domain_name::create(
///         "example",
///         DomainNameArgs::builder()
///             .certificate_arn("${exampleAwsAcmCertificateValidation.certificateArn}")
///             .domain_name("api.example.com")
///             .build_struct(),
///     );
///     let exampleRecord = record::create(
///         "exampleRecord",
///         RecordArgs::builder()
///             .aliases(
///                 vec![
///                     RecordAlias::builder().evaluateTargetHealth(true)
///                     .name("${example.cloudfrontDomainName}")
///                     .zoneId("${example.cloudfrontZoneId}").build_struct(),
///                 ],
///             )
///             .name("${example.domainName}")
///             .type_("A")
///             .zone_id("${exampleAwsRoute53Zone.id}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### Regional (ACM Certificate)
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = domain_name::create(
///         "example",
///         DomainNameArgs::builder()
///             .domain_name("api.example.com")
///             .endpoint_configuration(
///                 DomainNameEndpointConfiguration::builder()
///                     .types("REGIONAL")
///                     .build_struct(),
///             )
///             .regional_certificate_arn(
///                 "${exampleAwsAcmCertificateValidation.certificateArn}",
///             )
///             .build_struct(),
///     );
///     let exampleRecord = record::create(
///         "exampleRecord",
///         RecordArgs::builder()
///             .aliases(
///                 vec![
///                     RecordAlias::builder().evaluateTargetHealth(true)
///                     .name("${example.regionalDomainName}")
///                     .zoneId("${example.regionalZoneId}").build_struct(),
///                 ],
///             )
///             .name("${example.domainName}")
///             .type_("A")
///             .zone_id("${exampleAwsRoute53Zone.id}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import API Gateway domain names using their `name`. For example:
///
/// ```sh
/// $ pulumi import aws:apigateway/domainName:DomainName example dev.example.com
/// ```
pub mod domain_name {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct DomainNameArgs {
        /// ARN for an AWS-managed certificate. AWS Certificate Manager is the only supported source. Used when an edge-optimized domain name is desired. Conflicts with `certificate_name`, `certificate_body`, `certificate_chain`, `certificate_private_key`, `regional_certificate_arn`, and `regional_certificate_name`.
        #[builder(into, default)]
        pub certificate_arn: pulumi_wasm_rust::Output<Option<String>>,
        /// Certificate issued for the domain name being registered, in PEM format. Only valid for `EDGE` endpoint configuration type. Conflicts with `certificate_arn`, `regional_certificate_arn`, and `regional_certificate_name`.
        #[builder(into, default)]
        pub certificate_body: pulumi_wasm_rust::Output<Option<String>>,
        /// Certificate for the CA that issued the certificate, along with any intermediate CA certificates required to create an unbroken chain to a certificate trusted by the intended API clients. Only valid for `EDGE` endpoint configuration type. Conflicts with `certificate_arn`, `regional_certificate_arn`, and `regional_certificate_name`.
        #[builder(into, default)]
        pub certificate_chain: pulumi_wasm_rust::Output<Option<String>>,
        /// Unique name to use when registering this certificate as an IAM server certificate. Conflicts with `certificate_arn`, `regional_certificate_arn`, and `regional_certificate_name`. Required if `certificate_arn` is not set.
        #[builder(into, default)]
        pub certificate_name: pulumi_wasm_rust::Output<Option<String>>,
        /// Private key associated with the domain certificate given in `certificate_body`. Only valid for `EDGE` endpoint configuration type. Conflicts with `certificate_arn`, `regional_certificate_arn`, and `regional_certificate_name`.
        #[builder(into, default)]
        pub certificate_private_key: pulumi_wasm_rust::Output<Option<String>>,
        /// Fully-qualified domain name to register.
        #[builder(into)]
        pub domain_name: pulumi_wasm_rust::Output<String>,
        /// Configuration block defining API endpoint information including type. See below.
        #[builder(into, default)]
        pub endpoint_configuration: pulumi_wasm_rust::Output<
            Option<super::super::types::apigateway::DomainNameEndpointConfiguration>,
        >,
        /// Mutual TLS authentication configuration for the domain name. See below.
        #[builder(into, default)]
        pub mutual_tls_authentication: pulumi_wasm_rust::Output<
            Option<super::super::types::apigateway::DomainNameMutualTlsAuthentication>,
        >,
        /// ARN of the AWS-issued certificate used to validate custom domain ownership (when `certificate_arn` is issued via an ACM Private CA or `mutual_tls_authentication` is configured with an ACM-imported certificate.)
        #[builder(into, default)]
        pub ownership_verification_certificate_arn: pulumi_wasm_rust::Output<
            Option<String>,
        >,
        /// ARN for an AWS-managed certificate. AWS Certificate Manager is the only supported source. Used when a regional domain name is desired. Conflicts with `certificate_arn`, `certificate_name`, `certificate_body`, `certificate_chain`, and `certificate_private_key`.
        ///
        /// When uploading a certificate, the following arguments are supported:
        #[builder(into, default)]
        pub regional_certificate_arn: pulumi_wasm_rust::Output<Option<String>>,
        /// User-friendly name of the certificate that will be used by regional endpoint for this domain name. Conflicts with `certificate_arn`, `certificate_name`, `certificate_body`, `certificate_chain`, and `certificate_private_key`.
        #[builder(into, default)]
        pub regional_certificate_name: pulumi_wasm_rust::Output<Option<String>>,
        /// Transport Layer Security (TLS) version + cipher suite for this DomainName. Valid values are `TLS_1_0` and `TLS_1_2`. Must be configured to perform drift detection.
        #[builder(into, default)]
        pub security_policy: pulumi_wasm_rust::Output<Option<String>>,
        /// Key-value map of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        ///
        /// When referencing an AWS-managed certificate, the following arguments are supported:
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct DomainNameResult {
        /// ARN of domain name.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// ARN for an AWS-managed certificate. AWS Certificate Manager is the only supported source. Used when an edge-optimized domain name is desired. Conflicts with `certificate_name`, `certificate_body`, `certificate_chain`, `certificate_private_key`, `regional_certificate_arn`, and `regional_certificate_name`.
        pub certificate_arn: pulumi_wasm_rust::Output<Option<String>>,
        /// Certificate issued for the domain name being registered, in PEM format. Only valid for `EDGE` endpoint configuration type. Conflicts with `certificate_arn`, `regional_certificate_arn`, and `regional_certificate_name`.
        pub certificate_body: pulumi_wasm_rust::Output<Option<String>>,
        /// Certificate for the CA that issued the certificate, along with any intermediate CA certificates required to create an unbroken chain to a certificate trusted by the intended API clients. Only valid for `EDGE` endpoint configuration type. Conflicts with `certificate_arn`, `regional_certificate_arn`, and `regional_certificate_name`.
        pub certificate_chain: pulumi_wasm_rust::Output<Option<String>>,
        /// Unique name to use when registering this certificate as an IAM server certificate. Conflicts with `certificate_arn`, `regional_certificate_arn`, and `regional_certificate_name`. Required if `certificate_arn` is not set.
        pub certificate_name: pulumi_wasm_rust::Output<Option<String>>,
        /// Private key associated with the domain certificate given in `certificate_body`. Only valid for `EDGE` endpoint configuration type. Conflicts with `certificate_arn`, `regional_certificate_arn`, and `regional_certificate_name`.
        pub certificate_private_key: pulumi_wasm_rust::Output<Option<String>>,
        /// Upload date associated with the domain certificate.
        pub certificate_upload_date: pulumi_wasm_rust::Output<String>,
        /// Hostname created by Cloudfront to represent the distribution that implements this domain name mapping.
        pub cloudfront_domain_name: pulumi_wasm_rust::Output<String>,
        /// For convenience, the hosted zone ID (`Z2FDTNDATAQYW2`) that can be used to create a Route53 alias record for the distribution.
        pub cloudfront_zone_id: pulumi_wasm_rust::Output<String>,
        /// Fully-qualified domain name to register.
        pub domain_name: pulumi_wasm_rust::Output<String>,
        /// Configuration block defining API endpoint information including type. See below.
        pub endpoint_configuration: pulumi_wasm_rust::Output<
            super::super::types::apigateway::DomainNameEndpointConfiguration,
        >,
        /// Mutual TLS authentication configuration for the domain name. See below.
        pub mutual_tls_authentication: pulumi_wasm_rust::Output<
            Option<super::super::types::apigateway::DomainNameMutualTlsAuthentication>,
        >,
        /// ARN of the AWS-issued certificate used to validate custom domain ownership (when `certificate_arn` is issued via an ACM Private CA or `mutual_tls_authentication` is configured with an ACM-imported certificate.)
        pub ownership_verification_certificate_arn: pulumi_wasm_rust::Output<String>,
        /// ARN for an AWS-managed certificate. AWS Certificate Manager is the only supported source. Used when a regional domain name is desired. Conflicts with `certificate_arn`, `certificate_name`, `certificate_body`, `certificate_chain`, and `certificate_private_key`.
        ///
        /// When uploading a certificate, the following arguments are supported:
        pub regional_certificate_arn: pulumi_wasm_rust::Output<Option<String>>,
        /// User-friendly name of the certificate that will be used by regional endpoint for this domain name. Conflicts with `certificate_arn`, `certificate_name`, `certificate_body`, `certificate_chain`, and `certificate_private_key`.
        pub regional_certificate_name: pulumi_wasm_rust::Output<Option<String>>,
        /// Hostname for the custom domain's regional endpoint.
        pub regional_domain_name: pulumi_wasm_rust::Output<String>,
        /// Hosted zone ID that can be used to create a Route53 alias record for the regional endpoint.
        pub regional_zone_id: pulumi_wasm_rust::Output<String>,
        /// Transport Layer Security (TLS) version + cipher suite for this DomainName. Valid values are `TLS_1_0` and `TLS_1_2`. Must be configured to perform drift detection.
        pub security_policy: pulumi_wasm_rust::Output<String>,
        /// Key-value map of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        ///
        /// When referencing an AWS-managed certificate, the following arguments are supported:
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: DomainNameArgs) -> DomainNameResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let certificate_arn_binding = args.certificate_arn.get_inner();
        let certificate_body_binding = args.certificate_body.get_inner();
        let certificate_chain_binding = args.certificate_chain.get_inner();
        let certificate_name_binding = args.certificate_name.get_inner();
        let certificate_private_key_binding = args.certificate_private_key.get_inner();
        let domain_name_binding = args.domain_name.get_inner();
        let endpoint_configuration_binding = args.endpoint_configuration.get_inner();
        let mutual_tls_authentication_binding = args
            .mutual_tls_authentication
            .get_inner();
        let ownership_verification_certificate_arn_binding = args
            .ownership_verification_certificate_arn
            .get_inner();
        let regional_certificate_arn_binding = args.regional_certificate_arn.get_inner();
        let regional_certificate_name_binding = args
            .regional_certificate_name
            .get_inner();
        let security_policy_binding = args.security_policy.get_inner();
        let tags_binding = args.tags.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:apigateway/domainName:DomainName".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "certificateArn".into(),
                    value: &certificate_arn_binding,
                },
                register_interface::ObjectField {
                    name: "certificateBody".into(),
                    value: &certificate_body_binding,
                },
                register_interface::ObjectField {
                    name: "certificateChain".into(),
                    value: &certificate_chain_binding,
                },
                register_interface::ObjectField {
                    name: "certificateName".into(),
                    value: &certificate_name_binding,
                },
                register_interface::ObjectField {
                    name: "certificatePrivateKey".into(),
                    value: &certificate_private_key_binding,
                },
                register_interface::ObjectField {
                    name: "domainName".into(),
                    value: &domain_name_binding,
                },
                register_interface::ObjectField {
                    name: "endpointConfiguration".into(),
                    value: &endpoint_configuration_binding,
                },
                register_interface::ObjectField {
                    name: "mutualTlsAuthentication".into(),
                    value: &mutual_tls_authentication_binding,
                },
                register_interface::ObjectField {
                    name: "ownershipVerificationCertificateArn".into(),
                    value: &ownership_verification_certificate_arn_binding,
                },
                register_interface::ObjectField {
                    name: "regionalCertificateArn".into(),
                    value: &regional_certificate_arn_binding,
                },
                register_interface::ObjectField {
                    name: "regionalCertificateName".into(),
                    value: &regional_certificate_name_binding,
                },
                register_interface::ObjectField {
                    name: "securityPolicy".into(),
                    value: &security_policy_binding,
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
                    name: "certificateBody".into(),
                },
                register_interface::ResultField {
                    name: "certificateChain".into(),
                },
                register_interface::ResultField {
                    name: "certificateName".into(),
                },
                register_interface::ResultField {
                    name: "certificatePrivateKey".into(),
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
                    name: "endpointConfiguration".into(),
                },
                register_interface::ResultField {
                    name: "mutualTlsAuthentication".into(),
                },
                register_interface::ResultField {
                    name: "ownershipVerificationCertificateArn".into(),
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
        DomainNameResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            certificate_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("certificateArn").unwrap(),
            ),
            certificate_body: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("certificateBody").unwrap(),
            ),
            certificate_chain: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("certificateChain").unwrap(),
            ),
            certificate_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("certificateName").unwrap(),
            ),
            certificate_private_key: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("certificatePrivateKey").unwrap(),
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
            endpoint_configuration: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("endpointConfiguration").unwrap(),
            ),
            mutual_tls_authentication: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("mutualTlsAuthentication").unwrap(),
            ),
            ownership_verification_certificate_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("ownershipVerificationCertificateArn").unwrap(),
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
            tags_all: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tagsAll").unwrap(),
            ),
        }
    }
}
