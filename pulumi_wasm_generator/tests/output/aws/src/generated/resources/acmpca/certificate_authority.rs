/// Provides a resource to manage AWS Certificate Manager Private Certificate Authorities (ACM PCA Certificate Authorities).
///
/// > **NOTE:** Creating this resource will leave the certificate authority in a `PENDING_CERTIFICATE` status, which means it cannot yet issue certificates. To complete this setup, you must fully sign the certificate authority CSR available in the `certificate_signing_request` attribute. The `aws.acmpca.CertificateAuthorityCertificate` resource can be used for this purpose.
///
/// ## Example Usage
///
/// ### Basic
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = certificate_authority::create(
///         "example",
///         CertificateAuthorityArgs::builder()
///             .certificate_authority_configuration(
///                 CertificateAuthorityCertificateAuthorityConfiguration::builder()
///                     .keyAlgorithm("RSA_4096")
///                     .signingAlgorithm("SHA512WITHRSA")
///                     .subject(
///                         CertificateAuthorityCertificateAuthorityConfigurationSubject::builder()
///                             .commonName("example.com")
///                             .build_struct(),
///                     )
///                     .build_struct(),
///             )
///             .permanent_deletion_time_in_days(7)
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### Short-lived certificate
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = certificate_authority::create(
///         "example",
///         CertificateAuthorityArgs::builder()
///             .certificate_authority_configuration(
///                 CertificateAuthorityCertificateAuthorityConfiguration::builder()
///                     .keyAlgorithm("RSA_4096")
///                     .signingAlgorithm("SHA512WITHRSA")
///                     .subject(
///                         CertificateAuthorityCertificateAuthorityConfigurationSubject::builder()
///                             .commonName("example.com")
///                             .build_struct(),
///                     )
///                     .build_struct(),
///             )
///             .usage_mode("SHORT_LIVED_CERTIFICATE")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### Enable Certificate Revocation List
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let acmpcaBucketAccess = get_policy_document::invoke(
///         GetPolicyDocumentArgs::builder()
///             .statements(
///                 vec![
///                     GetPolicyDocumentStatement::builder().actions(vec!["s3:GetBucketAcl",
///                     "s3:GetBucketLocation", "s3:PutObject", "s3:PutObjectAcl",])
///                     .principals(vec![GetPolicyDocumentStatementPrincipal::builder()
///                     .identifiers(vec!["acm-pca.amazonaws.com",]). type ("Service")
///                     .build_struct(),]).resources(vec!["${example.arn}",
///                     "${example.arn}/*",]).build_struct(),
///                 ],
///             )
///             .build_struct(),
///     );
///     let example = bucket_v_2::create(
///         "example",
///         BucketV2Args::builder().bucket("example").force_destroy(true).build_struct(),
///     );
///     let exampleBucketPolicy = bucket_policy::create(
///         "exampleBucketPolicy",
///         BucketPolicyArgs::builder()
///             .bucket("${example.id}")
///             .policy("${acmpcaBucketAccess.json}")
///             .build_struct(),
///     );
///     let exampleCertificateAuthority = certificate_authority::create(
///         "exampleCertificateAuthority",
///         CertificateAuthorityArgs::builder()
///             .certificate_authority_configuration(
///                 CertificateAuthorityCertificateAuthorityConfiguration::builder()
///                     .keyAlgorithm("RSA_4096")
///                     .signingAlgorithm("SHA512WITHRSA")
///                     .subject(
///                         CertificateAuthorityCertificateAuthorityConfigurationSubject::builder()
///                             .commonName("example.com")
///                             .build_struct(),
///                     )
///                     .build_struct(),
///             )
///             .revocation_configuration(
///                 CertificateAuthorityRevocationConfiguration::builder()
///                     .crlConfiguration(
///                         CertificateAuthorityRevocationConfigurationCrlConfiguration::builder()
///                             .customCname("crl.example.com")
///                             .enabled(true)
///                             .expirationInDays(7)
///                             .s3BucketName("${example.id}")
///                             .s3ObjectAcl("BUCKET_OWNER_FULL_CONTROL")
///                             .build_struct(),
///                     )
///                     .build_struct(),
///             )
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import `aws_acmpca_certificate_authority` using the certificate authority ARN. For example:
///
/// ```sh
/// $ pulumi import aws:acmpca/certificateAuthority:CertificateAuthority example arn:aws:acm-pca:us-east-1:123456789012:certificate-authority/12345678-1234-1234-1234-123456789012
/// ```
pub mod certificate_authority {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct CertificateAuthorityArgs {
        /// Nested argument containing algorithms and certificate subject information. Defined below.
        #[builder(into)]
        pub certificate_authority_configuration: pulumi_wasm_rust::Output<
            super::super::types::acmpca::CertificateAuthorityCertificateAuthorityConfiguration,
        >,
        /// Whether the certificate authority is enabled or disabled. Defaults to `true`. Can only be disabled if the CA is in an `ACTIVE` state.
        #[builder(into, default)]
        pub enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// Cryptographic key management compliance standard used for handling CA keys. Defaults to `FIPS_140_2_LEVEL_3_OR_HIGHER`. Valid values: `FIPS_140_2_LEVEL_3_OR_HIGHER` and `FIPS_140_2_LEVEL_2_OR_HIGHER`. Supported standard for each region can be found in the [Storage and security compliance of AWS Private CA private keys Documentation](https://docs.aws.amazon.com/privateca/latest/userguide/data-protection.html#private-keys).
        #[builder(into, default)]
        pub key_storage_security_standard: pulumi_wasm_rust::Output<Option<String>>,
        /// Number of days to make a CA restorable after it has been deleted, must be between 7 to 30 days, with default to 30 days.
        #[builder(into, default)]
        pub permanent_deletion_time_in_days: pulumi_wasm_rust::Output<Option<i32>>,
        /// Nested argument containing revocation configuration. Defined below.
        #[builder(into, default)]
        pub revocation_configuration: pulumi_wasm_rust::Output<
            Option<
                super::super::types::acmpca::CertificateAuthorityRevocationConfiguration,
            >,
        >,
        /// Key-value map of user-defined tags that are attached to the certificate authority. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Type of the certificate authority. Defaults to `SUBORDINATE`. Valid values: `ROOT` and `SUBORDINATE`.
        #[builder(into, default)]
        pub type_: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies whether the CA issues general-purpose certificates that typically require a revocation mechanism, or short-lived certificates that may optionally omit revocation because they expire quickly. Short-lived certificate validity is limited to seven days. Defaults to `GENERAL_PURPOSE`. Valid values: `GENERAL_PURPOSE` and `SHORT_LIVED_CERTIFICATE`.
        #[builder(into, default)]
        pub usage_mode: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct CertificateAuthorityResult {
        /// ARN of the certificate authority.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// Base64-encoded certificate authority (CA) certificate. Only available after the certificate authority certificate has been imported.
        pub certificate: pulumi_wasm_rust::Output<String>,
        /// Nested argument containing algorithms and certificate subject information. Defined below.
        pub certificate_authority_configuration: pulumi_wasm_rust::Output<
            super::super::types::acmpca::CertificateAuthorityCertificateAuthorityConfiguration,
        >,
        /// Base64-encoded certificate chain that includes any intermediate certificates and chains up to root on-premises certificate that you used to sign your private CA certificate. The chain does not include your private CA certificate. Only available after the certificate authority certificate has been imported.
        pub certificate_chain: pulumi_wasm_rust::Output<String>,
        /// The base64 PEM-encoded certificate signing request (CSR) for your private CA certificate.
        pub certificate_signing_request: pulumi_wasm_rust::Output<String>,
        /// Whether the certificate authority is enabled or disabled. Defaults to `true`. Can only be disabled if the CA is in an `ACTIVE` state.
        pub enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// Cryptographic key management compliance standard used for handling CA keys. Defaults to `FIPS_140_2_LEVEL_3_OR_HIGHER`. Valid values: `FIPS_140_2_LEVEL_3_OR_HIGHER` and `FIPS_140_2_LEVEL_2_OR_HIGHER`. Supported standard for each region can be found in the [Storage and security compliance of AWS Private CA private keys Documentation](https://docs.aws.amazon.com/privateca/latest/userguide/data-protection.html#private-keys).
        pub key_storage_security_standard: pulumi_wasm_rust::Output<String>,
        /// Date and time after which the certificate authority is not valid. Only available after the certificate authority certificate has been imported.
        pub not_after: pulumi_wasm_rust::Output<String>,
        /// Date and time before which the certificate authority is not valid. Only available after the certificate authority certificate has been imported.
        pub not_before: pulumi_wasm_rust::Output<String>,
        /// Number of days to make a CA restorable after it has been deleted, must be between 7 to 30 days, with default to 30 days.
        pub permanent_deletion_time_in_days: pulumi_wasm_rust::Output<Option<i32>>,
        /// Nested argument containing revocation configuration. Defined below.
        pub revocation_configuration: pulumi_wasm_rust::Output<
            Option<
                super::super::types::acmpca::CertificateAuthorityRevocationConfiguration,
            >,
        >,
        /// Serial number of the certificate authority. Only available after the certificate authority certificate has been imported.
        pub serial: pulumi_wasm_rust::Output<String>,
        /// Key-value map of user-defined tags that are attached to the certificate authority. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Type of the certificate authority. Defaults to `SUBORDINATE`. Valid values: `ROOT` and `SUBORDINATE`.
        pub type_: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies whether the CA issues general-purpose certificates that typically require a revocation mechanism, or short-lived certificates that may optionally omit revocation because they expire quickly. Short-lived certificate validity is limited to seven days. Defaults to `GENERAL_PURPOSE`. Valid values: `GENERAL_PURPOSE` and `SHORT_LIVED_CERTIFICATE`.
        pub usage_mode: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: CertificateAuthorityArgs,
    ) -> CertificateAuthorityResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let certificate_authority_configuration_binding = args
            .certificate_authority_configuration
            .get_inner();
        let enabled_binding = args.enabled.get_inner();
        let key_storage_security_standard_binding = args
            .key_storage_security_standard
            .get_inner();
        let permanent_deletion_time_in_days_binding = args
            .permanent_deletion_time_in_days
            .get_inner();
        let revocation_configuration_binding = args.revocation_configuration.get_inner();
        let tags_binding = args.tags.get_inner();
        let type__binding = args.type_.get_inner();
        let usage_mode_binding = args.usage_mode.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:acmpca/certificateAuthority:CertificateAuthority".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "certificateAuthorityConfiguration".into(),
                    value: &certificate_authority_configuration_binding,
                },
                register_interface::ObjectField {
                    name: "enabled".into(),
                    value: &enabled_binding,
                },
                register_interface::ObjectField {
                    name: "keyStorageSecurityStandard".into(),
                    value: &key_storage_security_standard_binding,
                },
                register_interface::ObjectField {
                    name: "permanentDeletionTimeInDays".into(),
                    value: &permanent_deletion_time_in_days_binding,
                },
                register_interface::ObjectField {
                    name: "revocationConfiguration".into(),
                    value: &revocation_configuration_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "type".into(),
                    value: &type__binding,
                },
                register_interface::ObjectField {
                    name: "usageMode".into(),
                    value: &usage_mode_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "certificate".into(),
                },
                register_interface::ResultField {
                    name: "certificateAuthorityConfiguration".into(),
                },
                register_interface::ResultField {
                    name: "certificateChain".into(),
                },
                register_interface::ResultField {
                    name: "certificateSigningRequest".into(),
                },
                register_interface::ResultField {
                    name: "enabled".into(),
                },
                register_interface::ResultField {
                    name: "keyStorageSecurityStandard".into(),
                },
                register_interface::ResultField {
                    name: "notAfter".into(),
                },
                register_interface::ResultField {
                    name: "notBefore".into(),
                },
                register_interface::ResultField {
                    name: "permanentDeletionTimeInDays".into(),
                },
                register_interface::ResultField {
                    name: "revocationConfiguration".into(),
                },
                register_interface::ResultField {
                    name: "serial".into(),
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
                    name: "usageMode".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        CertificateAuthorityResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            certificate: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("certificate").unwrap(),
            ),
            certificate_authority_configuration: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("certificateAuthorityConfiguration").unwrap(),
            ),
            certificate_chain: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("certificateChain").unwrap(),
            ),
            certificate_signing_request: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("certificateSigningRequest").unwrap(),
            ),
            enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("enabled").unwrap(),
            ),
            key_storage_security_standard: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("keyStorageSecurityStandard").unwrap(),
            ),
            not_after: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("notAfter").unwrap(),
            ),
            not_before: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("notBefore").unwrap(),
            ),
            permanent_deletion_time_in_days: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("permanentDeletionTimeInDays").unwrap(),
            ),
            revocation_configuration: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("revocationConfiguration").unwrap(),
            ),
            serial: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("serial").unwrap(),
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
            usage_mode: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("usageMode").unwrap(),
            ),
        }
    }
}