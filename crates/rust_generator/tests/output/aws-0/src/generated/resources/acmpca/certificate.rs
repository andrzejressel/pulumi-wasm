/// Provides a resource to issue a certificate using AWS Certificate Manager Private Certificate Authority (ACM PCA).
///
/// Certificates created using `aws.acmpca.Certificate` are not eligible for automatic renewal,
/// and must be replaced instead.
/// To issue a renewable certificate using an ACM PCA, create a `aws.acm.Certificate`
/// with the parameter `certificate_authority_arn`.
///
/// ## Example Usage
///
/// ### Basic
///
/// ```yaml
/// resources:
///   example:
///     type: aws:acmpca:Certificate
///     properties:
///       certificateAuthorityArn: ${exampleCertificateAuthority.arn}
///       certificateSigningRequest: ${csr.certRequestPem}
///       signingAlgorithm: SHA256WITHRSA
///       validity:
///         type: YEARS
///         value: 1
///   exampleCertificateAuthority:
///     type: aws:acmpca:CertificateAuthority
///     name: example
///     properties:
///       certificateAuthorityConfiguration:
///         keyAlgorithm: RSA_4096
///         signingAlgorithm: SHA512WITHRSA
///         subject:
///           commonName: example.com
///       permanentDeletionTimeInDays: 7
///   key:
///     type: tls:PrivateKey
///     properties:
///       algorithm: RSA
///   csr:
///     type: tls:CertRequest
///     properties:
///       privateKeyPem: ${key.privateKeyPem}
///       subject:
///         commonName: example
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import ACM PCA Certificates using their ARN. For example:
///
/// ```sh
/// $ pulumi import aws:acmpca/certificate:Certificate cert arn:aws:acm-pca:eu-west-1:675225743824:certificate-authority/08319ede-83g9-1400-8f21-c7d12b2b6edb/certificate/a4e9c2aa4bcfab625g1b9136464cd3a
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod certificate {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct CertificateArgs {
        /// Specifies X.509 certificate information to be included in the issued certificate. To use with API Passthrough templates
        #[builder(into, default)]
        pub api_passthrough: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// ARN of the certificate authority.
        #[builder(into)]
        pub certificate_authority_arn: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Certificate Signing Request in PEM format.
        #[builder(into)]
        pub certificate_signing_request: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Algorithm to use to sign certificate requests. Valid values: `SHA256WITHRSA`, `SHA256WITHECDSA`, `SHA384WITHRSA`, `SHA384WITHECDSA`, `SHA512WITHRSA`, `SHA512WITHECDSA`.
        #[builder(into)]
        pub signing_algorithm: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Template to use when issuing a certificate.
        /// See [ACM PCA Documentation](https://docs.aws.amazon.com/privateca/latest/userguide/UsingTemplates.html) for more information.
        #[builder(into, default)]
        pub template_arn: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Configures end of the validity period for the certificate. See validity block below.
        #[builder(into)]
        pub validity: pulumi_gestalt_rust::InputOrOutput<
            super::super::types::acmpca::CertificateValidity,
        >,
    }
    #[allow(dead_code)]
    pub struct CertificateResult {
        /// Specifies X.509 certificate information to be included in the issued certificate. To use with API Passthrough templates
        pub api_passthrough: pulumi_gestalt_rust::Output<Option<String>>,
        /// ARN of the certificate.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// PEM-encoded certificate value.
        pub certificate: pulumi_gestalt_rust::Output<String>,
        /// ARN of the certificate authority.
        pub certificate_authority_arn: pulumi_gestalt_rust::Output<String>,
        /// PEM-encoded certificate chain that includes any intermediate certificates and chains up to root CA.
        pub certificate_chain: pulumi_gestalt_rust::Output<String>,
        /// Certificate Signing Request in PEM format.
        pub certificate_signing_request: pulumi_gestalt_rust::Output<String>,
        /// Algorithm to use to sign certificate requests. Valid values: `SHA256WITHRSA`, `SHA256WITHECDSA`, `SHA384WITHRSA`, `SHA384WITHECDSA`, `SHA512WITHRSA`, `SHA512WITHECDSA`.
        pub signing_algorithm: pulumi_gestalt_rust::Output<String>,
        /// Template to use when issuing a certificate.
        /// See [ACM PCA Documentation](https://docs.aws.amazon.com/privateca/latest/userguide/UsingTemplates.html) for more information.
        pub template_arn: pulumi_gestalt_rust::Output<Option<String>>,
        /// Configures end of the validity period for the certificate. See validity block below.
        pub validity: pulumi_gestalt_rust::Output<
            super::super::types::acmpca::CertificateValidity,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: CertificateArgs,
    ) -> CertificateResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let api_passthrough_binding = args.api_passthrough.get_output(context);
        let certificate_authority_arn_binding = args
            .certificate_authority_arn
            .get_output(context);
        let certificate_signing_request_binding = args
            .certificate_signing_request
            .get_output(context);
        let signing_algorithm_binding = args.signing_algorithm.get_output(context);
        let template_arn_binding = args.template_arn.get_output(context);
        let validity_binding = args.validity.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:acmpca/certificate:Certificate".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "apiPassthrough".into(),
                    value: api_passthrough_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "certificateAuthorityArn".into(),
                    value: certificate_authority_arn_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "certificateSigningRequest".into(),
                    value: certificate_signing_request_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "signingAlgorithm".into(),
                    value: signing_algorithm_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "templateArn".into(),
                    value: template_arn_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "validity".into(),
                    value: validity_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        CertificateResult {
            api_passthrough: o.get_field("apiPassthrough"),
            arn: o.get_field("arn"),
            certificate: o.get_field("certificate"),
            certificate_authority_arn: o.get_field("certificateAuthorityArn"),
            certificate_chain: o.get_field("certificateChain"),
            certificate_signing_request: o.get_field("certificateSigningRequest"),
            signing_algorithm: o.get_field("signingAlgorithm"),
            template_arn: o.get_field("templateArn"),
            validity: o.get_field("validity"),
        }
    }
}
