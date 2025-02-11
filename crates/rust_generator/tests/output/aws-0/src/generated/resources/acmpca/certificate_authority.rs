/// Provides a resource to manage AWS Certificate Manager Private Certificate Authorities (ACM PCA Certificate Authorities).
///
/// > **NOTE:** Creating this resource will leave the certificate authority in a `PENDING_CERTIFICATE` status, which means it cannot yet issue certificates. To complete this setup, you must fully sign the certificate authority CSR available in the `certificate_signing_request` attribute. The `aws.acmpca.CertificateAuthorityCertificate` resource can be used for this purpose.
///
/// ## Example Usage
///
/// ### Basic
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
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
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
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
/// ```yaml
/// resources:
///   example:
///     type: aws:s3:BucketV2
///     properties:
///       bucket: example
///       forceDestroy: true
///   exampleBucketPolicy:
///     type: aws:s3:BucketPolicy
///     name: example
///     properties:
///       bucket: ${example.id}
///       policy: ${acmpcaBucketAccess.json}
///   exampleCertificateAuthority:
///     type: aws:acmpca:CertificateAuthority
///     name: example
///     properties:
///       certificateAuthorityConfiguration:
///         keyAlgorithm: RSA_4096
///         signingAlgorithm: SHA512WITHRSA
///         subject:
///           commonName: example.com
///       revocationConfiguration:
///         crlConfiguration:
///           customCname: crl.example.com
///           enabled: true
///           expirationInDays: 7
///           s3BucketName: ${example.id}
///           s3ObjectAcl: BUCKET_OWNER_FULL_CONTROL
///     options:
///       dependsOn:
///         - ${exampleBucketPolicy}
/// variables:
///   acmpcaBucketAccess:
///     fn::invoke:
///       function: aws:iam:getPolicyDocument
///       arguments:
///         statements:
///           - actions:
///               - s3:GetBucketAcl
///               - s3:GetBucketLocation
///               - s3:PutObject
///               - s3:PutObjectAcl
///             resources:
///               - ${example.arn}
///               - ${example.arn}/*
///             principals:
///               - identifiers:
///                   - acm-pca.amazonaws.com
///                 type: Service
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import `aws_acmpca_certificate_authority` using the certificate authority ARN. For example:
///
/// ```sh
/// $ pulumi import aws:acmpca/certificateAuthority:CertificateAuthority example arn:aws:acm-pca:us-east-1:123456789012:certificate-authority/12345678-1234-1234-1234-123456789012
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod certificate_authority {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct CertificateAuthorityArgs {
        /// Nested argument containing algorithms and certificate subject information. Defined below.
        #[builder(into)]
        pub certificate_authority_configuration: pulumi_gestalt_rust::InputOrOutput<
            super::super::types::acmpca::CertificateAuthorityCertificateAuthorityConfiguration,
        >,
        /// Whether the certificate authority is enabled or disabled. Defaults to `true`. Can only be disabled if the CA is in an `ACTIVE` state.
        #[builder(into, default)]
        pub enabled: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Cryptographic key management compliance standard used for handling CA keys. Defaults to `FIPS_140_2_LEVEL_3_OR_HIGHER`. Valid values: `FIPS_140_2_LEVEL_3_OR_HIGHER` and `FIPS_140_2_LEVEL_2_OR_HIGHER`. Supported standard for each region can be found in the [Storage and security compliance of AWS Private CA private keys Documentation](https://docs.aws.amazon.com/privateca/latest/userguide/data-protection.html#private-keys).
        #[builder(into, default)]
        pub key_storage_security_standard: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// Number of days to make a CA restorable after it has been deleted, must be between 7 to 30 days, with default to 30 days.
        #[builder(into, default)]
        pub permanent_deletion_time_in_days: pulumi_gestalt_rust::InputOrOutput<
            Option<i32>,
        >,
        /// Nested argument containing revocation configuration. Defined below.
        #[builder(into, default)]
        pub revocation_configuration: pulumi_gestalt_rust::InputOrOutput<
            Option<
                super::super::types::acmpca::CertificateAuthorityRevocationConfiguration,
            >,
        >,
        /// Key-value map of user-defined tags that are attached to the certificate authority. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Type of the certificate authority. Defaults to `SUBORDINATE`. Valid values: `ROOT` and `SUBORDINATE`.
        #[builder(into, default)]
        pub type_: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies whether the CA issues general-purpose certificates that typically require a revocation mechanism, or short-lived certificates that may optionally omit revocation because they expire quickly. Short-lived certificate validity is limited to seven days. Defaults to `GENERAL_PURPOSE`. Valid values: `GENERAL_PURPOSE` and `SHORT_LIVED_CERTIFICATE`.
        #[builder(into, default)]
        pub usage_mode: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct CertificateAuthorityResult {
        /// ARN of the certificate authority.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// Base64-encoded certificate authority (CA) certificate. Only available after the certificate authority certificate has been imported.
        pub certificate: pulumi_gestalt_rust::Output<String>,
        /// Nested argument containing algorithms and certificate subject information. Defined below.
        pub certificate_authority_configuration: pulumi_gestalt_rust::Output<
            super::super::types::acmpca::CertificateAuthorityCertificateAuthorityConfiguration,
        >,
        /// Base64-encoded certificate chain that includes any intermediate certificates and chains up to root on-premises certificate that you used to sign your private CA certificate. The chain does not include your private CA certificate. Only available after the certificate authority certificate has been imported.
        pub certificate_chain: pulumi_gestalt_rust::Output<String>,
        /// The base64 PEM-encoded certificate signing request (CSR) for your private CA certificate.
        pub certificate_signing_request: pulumi_gestalt_rust::Output<String>,
        /// Whether the certificate authority is enabled or disabled. Defaults to `true`. Can only be disabled if the CA is in an `ACTIVE` state.
        pub enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Cryptographic key management compliance standard used for handling CA keys. Defaults to `FIPS_140_2_LEVEL_3_OR_HIGHER`. Valid values: `FIPS_140_2_LEVEL_3_OR_HIGHER` and `FIPS_140_2_LEVEL_2_OR_HIGHER`. Supported standard for each region can be found in the [Storage and security compliance of AWS Private CA private keys Documentation](https://docs.aws.amazon.com/privateca/latest/userguide/data-protection.html#private-keys).
        pub key_storage_security_standard: pulumi_gestalt_rust::Output<String>,
        /// Date and time after which the certificate authority is not valid. Only available after the certificate authority certificate has been imported.
        pub not_after: pulumi_gestalt_rust::Output<String>,
        /// Date and time before which the certificate authority is not valid. Only available after the certificate authority certificate has been imported.
        pub not_before: pulumi_gestalt_rust::Output<String>,
        /// Number of days to make a CA restorable after it has been deleted, must be between 7 to 30 days, with default to 30 days.
        pub permanent_deletion_time_in_days: pulumi_gestalt_rust::Output<Option<i32>>,
        /// Nested argument containing revocation configuration. Defined below.
        pub revocation_configuration: pulumi_gestalt_rust::Output<
            Option<
                super::super::types::acmpca::CertificateAuthorityRevocationConfiguration,
            >,
        >,
        /// Serial number of the certificate authority. Only available after the certificate authority certificate has been imported.
        pub serial: pulumi_gestalt_rust::Output<String>,
        /// Key-value map of user-defined tags that are attached to the certificate authority. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Type of the certificate authority. Defaults to `SUBORDINATE`. Valid values: `ROOT` and `SUBORDINATE`.
        pub type_: pulumi_gestalt_rust::Output<Option<String>>,
        /// Specifies whether the CA issues general-purpose certificates that typically require a revocation mechanism, or short-lived certificates that may optionally omit revocation because they expire quickly. Short-lived certificate validity is limited to seven days. Defaults to `GENERAL_PURPOSE`. Valid values: `GENERAL_PURPOSE` and `SHORT_LIVED_CERTIFICATE`.
        pub usage_mode: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: CertificateAuthorityArgs,
    ) -> CertificateAuthorityResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let certificate_authority_configuration_binding = args
            .certificate_authority_configuration
            .get_output(context);
        let enabled_binding = args.enabled.get_output(context);
        let key_storage_security_standard_binding = args
            .key_storage_security_standard
            .get_output(context);
        let permanent_deletion_time_in_days_binding = args
            .permanent_deletion_time_in_days
            .get_output(context);
        let revocation_configuration_binding = args
            .revocation_configuration
            .get_output(context);
        let tags_binding = args.tags.get_output(context);
        let type__binding = args.type_.get_output(context);
        let usage_mode_binding = args.usage_mode.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:acmpca/certificateAuthority:CertificateAuthority".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "certificateAuthorityConfiguration".into(),
                    value: &certificate_authority_configuration_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "enabled".into(),
                    value: &enabled_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "keyStorageSecurityStandard".into(),
                    value: &key_storage_security_standard_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "permanentDeletionTimeInDays".into(),
                    value: &permanent_deletion_time_in_days_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "revocationConfiguration".into(),
                    value: &revocation_configuration_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: &tags_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "type".into(),
                    value: &type__binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "usageMode".into(),
                    value: &usage_mode_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        CertificateAuthorityResult {
            arn: o.get_field("arn"),
            certificate: o.get_field("certificate"),
            certificate_authority_configuration: o
                .get_field("certificateAuthorityConfiguration"),
            certificate_chain: o.get_field("certificateChain"),
            certificate_signing_request: o.get_field("certificateSigningRequest"),
            enabled: o.get_field("enabled"),
            key_storage_security_standard: o.get_field("keyStorageSecurityStandard"),
            not_after: o.get_field("notAfter"),
            not_before: o.get_field("notBefore"),
            permanent_deletion_time_in_days: o.get_field("permanentDeletionTimeInDays"),
            revocation_configuration: o.get_field("revocationConfiguration"),
            serial: o.get_field("serial"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
            type_: o.get_field("type"),
            usage_mode: o.get_field("usageMode"),
        }
    }
}
