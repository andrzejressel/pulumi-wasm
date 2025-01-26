/// The ACM certificate resource allows requesting and management of certificates
/// from the Amazon Certificate Manager.
///
/// ACM certificates can be created in three ways:
/// Amazon-issued, where AWS provides the certificate authority and automatically manages renewal;
/// imported certificates, issued by another certificate authority;
/// and private certificates, issued using an ACM Private Certificate Authority.
///
/// ## Amazon-Issued Certificates
///
/// For Amazon-issued certificates, this resource deals with requesting certificates and managing their attributes and life-cycle.
/// This resource does not deal with validation of a certificate but can provide inputs
/// for other resources implementing the validation.
/// It does not wait for a certificate to be issued.
/// Use a `aws.acm.CertificateValidation` resource for this.
///
/// Most commonly, this resource is used together with `aws.route53.Record` and
/// `aws.acm.CertificateValidation` to request a DNS validated certificate,
/// deploy the required validation records and wait for validation to complete.
///
/// Domain validation through email is also supported but should be avoided as it requires a manual step outside of this provider.
///
///
/// ## Certificates Imported from Other Certificate Authority
///
/// Imported certificates can be used to make certificates created with an external certificate authority available for AWS services.
///
/// As they are not managed by AWS, imported certificates are not eligible for automatic renewal.
/// New certificate materials can be supplied to an existing imported certificate to update it in place.
///
/// ## Private Certificates
///
/// Private certificates are issued by an ACM Private Certificate Authority, which can be created using the resource type `aws.acmpca.CertificateAuthority`.
///
/// Private certificates created using this resource are eligible for managed renewal if they have been exported or associated with another AWS service.
/// See [managed renewal documentation](https://docs.aws.amazon.com/acm/latest/userguide/managed-renewal.html) for more information.
/// By default, a certificate is valid for 395 days and the managed renewal process will start 60 days before expiration.
/// To renew the certificate earlier than 60 days before expiration, configure `early_renewal_duration`.
///
/// ## Example Usage
///
/// ### Custom Domain Validation Options
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let cert = certificate::create(
///         "cert",
///         CertificateArgs::builder()
///             .domain_name("testing.example.com")
///             .validation_method("EMAIL")
///             .validation_options(
///                 vec![
///                     CertificateValidationOption::builder()
///                     .domainName("testing.example.com").validationDomain("example.com")
///                     .build_struct(),
///                 ],
///             )
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### Existing Certificate Body Import
///
/// ```yaml
/// resources:
///   example:
///     type: tls:PrivateKey
///     properties:
///       algorithm: RSA
///   exampleSelfSignedCert:
///     type: tls:SelfSignedCert
///     name: example
///     properties:
///       keyAlgorithm: RSA
///       privateKeyPem: ${example.privateKeyPem}
///       subject:
///         commonName: example.com
///         organization: ACME Examples, Inc
///       validityPeriodHours: 12
///       allowedUses:
///         - key_encipherment
///         - digital_signature
///         - server_auth
///   cert:
///     type: aws:acm:Certificate
///     properties:
///       privateKey: ${example.privateKeyPem}
///       certificateBody: ${exampleSelfSignedCert.certPem}
/// ```
///
/// ### Referencing domain_validation_options With for_each Based Resources
///
/// See the `aws.acm.CertificateValidation` resource for a full example of performing DNS validation.
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = record::create(
///         "example",
///         RecordArgs::builder()
///             .allow_overwrite(true)
///             .name("${range.value.name}")
///             .records(vec!["${range.value.record}",])
///             .ttl(60)
///             .type_("${range.value.type}")
///             .zone_id("${exampleAwsRoute53Zone.zoneId}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import certificates using their ARN. For example:
///
/// ```sh
/// $ pulumi import aws:acm/certificate:Certificate cert arn:aws:acm:eu-central-1:123456789012:certificate/7e7a28d2-163f-4b8f-b9cd-822f96c08d6a
/// ```
pub mod certificate {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct CertificateArgs {
        /// ARN of an ACM PCA
        #[builder(into, default)]
        pub certificate_authority_arn: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Certificate's PEM-formatted public key
        #[builder(into, default)]
        pub certificate_body: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Certificate's PEM-formatted chain
        /// * Creating a private CA issued certificate
        #[builder(into, default)]
        pub certificate_chain: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Fully qualified domain name (FQDN) in the certificate.
        #[builder(into, default)]
        pub domain_name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Amount of time to start automatic renewal process before expiration.
        /// Has no effect if less than 60 days.
        /// Represented by either
        /// a subset of [RFC 3339 duration](https://www.rfc-editor.org/rfc/rfc3339) supporting years, months, and days (e.g., `P90D`),
        /// or a string such as `2160h`.
        #[builder(into, default)]
        pub early_renewal_duration: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Specifies the algorithm of the public and private key pair that your Amazon issued certificate uses to encrypt data. See [ACM Certificate characteristics](https://docs.aws.amazon.com/acm/latest/userguide/acm-certificate.html#algorithms) for more details.
        #[builder(into, default)]
        pub key_algorithm: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Configuration block used to set certificate options. Detailed below.
        #[builder(into, default)]
        pub options: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::acm::CertificateOptions>,
        >,
        /// Certificate's PEM-formatted private key
        #[builder(into, default)]
        pub private_key: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Set of domains that should be SANs in the issued certificate.
        /// To remove all elements of a previously configured list, set this value equal to an empty list (`[]`)
        #[builder(into, default)]
        pub subject_alternative_names: pulumi_wasm_rust::InputOrOutput<
            Option<Vec<String>>,
        >,
        /// Map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Which method to use for validation. `DNS` or `EMAIL` are valid. This parameter must not be set for certificates that were imported into ACM and then into Pulumi.
        #[builder(into, default)]
        pub validation_method: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Configuration block used to specify information about the initial validation of each domain name. Detailed below.
        /// * Importing an existing certificate
        #[builder(into, default)]
        pub validation_options: pulumi_wasm_rust::InputOrOutput<
            Option<Vec<super::super::types::acm::CertificateValidationOption>>,
        >,
    }
    #[allow(dead_code)]
    pub struct CertificateResult {
        /// ARN of the certificate
        pub arn: pulumi_wasm_rust::Output<String>,
        /// ARN of an ACM PCA
        pub certificate_authority_arn: pulumi_wasm_rust::Output<Option<String>>,
        /// Certificate's PEM-formatted public key
        pub certificate_body: pulumi_wasm_rust::Output<Option<String>>,
        /// Certificate's PEM-formatted chain
        /// * Creating a private CA issued certificate
        pub certificate_chain: pulumi_wasm_rust::Output<Option<String>>,
        /// Fully qualified domain name (FQDN) in the certificate.
        pub domain_name: pulumi_wasm_rust::Output<String>,
        /// Set of domain validation objects which can be used to complete certificate validation.
        /// Can have more than one element, e.g., if SANs are defined.
        /// Only set if `DNS`-validation was used.
        pub domain_validation_options: pulumi_wasm_rust::Output<
            Vec<super::super::types::acm::CertificateDomainValidationOption>,
        >,
        /// Amount of time to start automatic renewal process before expiration.
        /// Has no effect if less than 60 days.
        /// Represented by either
        /// a subset of [RFC 3339 duration](https://www.rfc-editor.org/rfc/rfc3339) supporting years, months, and days (e.g., `P90D`),
        /// or a string such as `2160h`.
        pub early_renewal_duration: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies the algorithm of the public and private key pair that your Amazon issued certificate uses to encrypt data. See [ACM Certificate characteristics](https://docs.aws.amazon.com/acm/latest/userguide/acm-certificate.html#algorithms) for more details.
        pub key_algorithm: pulumi_wasm_rust::Output<String>,
        /// Expiration date and time of the certificate.
        pub not_after: pulumi_wasm_rust::Output<String>,
        /// Start of the validity period of the certificate.
        pub not_before: pulumi_wasm_rust::Output<String>,
        /// Configuration block used to set certificate options. Detailed below.
        pub options: pulumi_wasm_rust::Output<
            super::super::types::acm::CertificateOptions,
        >,
        /// `true` if a Private certificate eligible for managed renewal is within the `early_renewal_duration` period.
        pub pending_renewal: pulumi_wasm_rust::Output<bool>,
        /// Certificate's PEM-formatted private key
        pub private_key: pulumi_wasm_rust::Output<Option<String>>,
        /// Whether the certificate is eligible for managed renewal.
        pub renewal_eligibility: pulumi_wasm_rust::Output<String>,
        /// Contains information about the status of ACM's [managed renewal](https://docs.aws.amazon.com/acm/latest/userguide/acm-renewal.html) for the certificate.
        pub renewal_summaries: pulumi_wasm_rust::Output<
            Vec<super::super::types::acm::CertificateRenewalSummary>,
        >,
        /// Status of the certificate.
        pub status: pulumi_wasm_rust::Output<String>,
        /// Set of domains that should be SANs in the issued certificate.
        /// To remove all elements of a previously configured list, set this value equal to an empty list (`[]`)
        pub subject_alternative_names: pulumi_wasm_rust::Output<Vec<String>>,
        /// Map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Source of the certificate.
        pub type_: pulumi_wasm_rust::Output<String>,
        /// List of addresses that received a validation email. Only set if `EMAIL` validation was used.
        pub validation_emails: pulumi_wasm_rust::Output<Vec<String>>,
        /// Which method to use for validation. `DNS` or `EMAIL` are valid. This parameter must not be set for certificates that were imported into ACM and then into Pulumi.
        pub validation_method: pulumi_wasm_rust::Output<String>,
        /// Configuration block used to specify information about the initial validation of each domain name. Detailed below.
        /// * Importing an existing certificate
        pub validation_options: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::acm::CertificateValidationOption>>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: CertificateArgs,
    ) -> CertificateResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let certificate_authority_arn_binding = args
            .certificate_authority_arn
            .get_output(context)
            .get_inner();
        let certificate_body_binding = args
            .certificate_body
            .get_output(context)
            .get_inner();
        let certificate_chain_binding = args
            .certificate_chain
            .get_output(context)
            .get_inner();
        let domain_name_binding = args.domain_name.get_output(context).get_inner();
        let early_renewal_duration_binding = args
            .early_renewal_duration
            .get_output(context)
            .get_inner();
        let key_algorithm_binding = args.key_algorithm.get_output(context).get_inner();
        let options_binding = args.options.get_output(context).get_inner();
        let private_key_binding = args.private_key.get_output(context).get_inner();
        let subject_alternative_names_binding = args
            .subject_alternative_names
            .get_output(context)
            .get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let validation_method_binding = args
            .validation_method
            .get_output(context)
            .get_inner();
        let validation_options_binding = args
            .validation_options
            .get_output(context)
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:acm/certificate:Certificate".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "certificateAuthorityArn".into(),
                    value: &certificate_authority_arn_binding,
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
                    name: "domainName".into(),
                    value: &domain_name_binding,
                },
                register_interface::ObjectField {
                    name: "earlyRenewalDuration".into(),
                    value: &early_renewal_duration_binding,
                },
                register_interface::ObjectField {
                    name: "keyAlgorithm".into(),
                    value: &key_algorithm_binding,
                },
                register_interface::ObjectField {
                    name: "options".into(),
                    value: &options_binding,
                },
                register_interface::ObjectField {
                    name: "privateKey".into(),
                    value: &private_key_binding,
                },
                register_interface::ObjectField {
                    name: "subjectAlternativeNames".into(),
                    value: &subject_alternative_names_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "validationMethod".into(),
                    value: &validation_method_binding,
                },
                register_interface::ObjectField {
                    name: "validationOptions".into(),
                    value: &validation_options_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "certificateAuthorityArn".into(),
                },
                register_interface::ResultField {
                    name: "certificateBody".into(),
                },
                register_interface::ResultField {
                    name: "certificateChain".into(),
                },
                register_interface::ResultField {
                    name: "domainName".into(),
                },
                register_interface::ResultField {
                    name: "domainValidationOptions".into(),
                },
                register_interface::ResultField {
                    name: "earlyRenewalDuration".into(),
                },
                register_interface::ResultField {
                    name: "keyAlgorithm".into(),
                },
                register_interface::ResultField {
                    name: "notAfter".into(),
                },
                register_interface::ResultField {
                    name: "notBefore".into(),
                },
                register_interface::ResultField {
                    name: "options".into(),
                },
                register_interface::ResultField {
                    name: "pendingRenewal".into(),
                },
                register_interface::ResultField {
                    name: "privateKey".into(),
                },
                register_interface::ResultField {
                    name: "renewalEligibility".into(),
                },
                register_interface::ResultField {
                    name: "renewalSummaries".into(),
                },
                register_interface::ResultField {
                    name: "status".into(),
                },
                register_interface::ResultField {
                    name: "subjectAlternativeNames".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "tagsAll".into(),
                },
                register_interface::ResultField {
                    name: "type".into(),
                },
                register_interface::ResultField {
                    name: "validationEmails".into(),
                },
                register_interface::ResultField {
                    name: "validationMethod".into(),
                },
                register_interface::ResultField {
                    name: "validationOptions".into(),
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        CertificateResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            certificate_authority_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("certificateAuthorityArn").unwrap(),
            ),
            certificate_body: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("certificateBody").unwrap(),
            ),
            certificate_chain: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("certificateChain").unwrap(),
            ),
            domain_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("domainName").unwrap(),
            ),
            domain_validation_options: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("domainValidationOptions").unwrap(),
            ),
            early_renewal_duration: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("earlyRenewalDuration").unwrap(),
            ),
            key_algorithm: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("keyAlgorithm").unwrap(),
            ),
            not_after: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("notAfter").unwrap(),
            ),
            not_before: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("notBefore").unwrap(),
            ),
            options: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("options").unwrap(),
            ),
            pending_renewal: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("pendingRenewal").unwrap(),
            ),
            private_key: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("privateKey").unwrap(),
            ),
            renewal_eligibility: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("renewalEligibility").unwrap(),
            ),
            renewal_summaries: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("renewalSummaries").unwrap(),
            ),
            status: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("status").unwrap(),
            ),
            subject_alternative_names: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("subjectAlternativeNames").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            tags_all: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tagsAll").unwrap(),
            ),
            type_: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("type").unwrap(),
            ),
            validation_emails: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("validationEmails").unwrap(),
            ),
            validation_method: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("validationMethod").unwrap(),
            ),
            validation_options: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("validationOptions").unwrap(),
            ),
        }
    }
}
