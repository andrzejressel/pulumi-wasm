/// Associates a certificate with an AWS Certificate Manager Private Certificate Authority (ACM PCA Certificate Authority). An ACM PCA Certificate Authority is unable to issue certificates until it has a certificate associated with it. A root level ACM PCA Certificate Authority is able to self-sign its own root certificate.
///
/// ## Example Usage
///
/// ### Self-Signed Root Certificate Authority Certificate
///
/// ```yaml
/// resources:
///   example:
///     type: aws:acmpca:CertificateAuthorityCertificate
///     properties:
///       certificateAuthorityArn: ${exampleCertificateAuthority.arn}
///       certificate: ${exampleCertificate.certificate}
///       certificateChain: ${exampleCertificate.certificateChain}
///   exampleCertificate:
///     type: aws:acmpca:Certificate
///     name: example
///     properties:
///       certificateAuthorityArn: ${exampleCertificateAuthority.arn}
///       certificateSigningRequest: ${exampleCertificateAuthority.certificateSigningRequest}
///       signingAlgorithm: SHA512WITHRSA
///       templateArn: arn:${current.partition}:acm-pca:::template/RootCACertificate/V1
///       validity:
///         type: YEARS
///         value: 1
///   exampleCertificateAuthority:
///     type: aws:acmpca:CertificateAuthority
///     name: example
///     properties:
///       type: ROOT
///       certificateAuthorityConfiguration:
///         keyAlgorithm: RSA_4096
///         signingAlgorithm: SHA512WITHRSA
///         subject:
///           commonName: example.com
/// variables:
///   current:
///     fn::invoke:
///       function: aws:getPartition
///       arguments: {}
/// ```
///
/// ### Certificate for Subordinate Certificate Authority
///
/// Note that the certificate for the subordinate certificate authority must be issued by the root certificate authority using a signing request from the subordinate certificate authority.
///
/// ```yaml
/// resources:
///   subordinate:
///     type: aws:acmpca:CertificateAuthorityCertificate
///     properties:
///       certificateAuthorityArn: ${subordinateCertificateAuthority.arn}
///       certificate: ${subordinateCertificate.certificate}
///       certificateChain: ${subordinateCertificate.certificateChain}
///   subordinateCertificate:
///     type: aws:acmpca:Certificate
///     name: subordinate
///     properties:
///       certificateAuthorityArn: ${root.arn}
///       certificateSigningRequest: ${subordinateCertificateAuthority.certificateSigningRequest}
///       signingAlgorithm: SHA512WITHRSA
///       templateArn: arn:${current.partition}:acm-pca:::template/SubordinateCACertificate_PathLen0/V1
///       validity:
///         type: YEARS
///         value: 1
///   subordinateCertificateAuthority:
///     type: aws:acmpca:CertificateAuthority
///     name: subordinate
///     properties:
///       type: SUBORDINATE
///       certificateAuthorityConfiguration:
///         keyAlgorithm: RSA_2048
///         signingAlgorithm: SHA512WITHRSA
///         subject:
///           commonName: sub.example.com
///   root:
///     type: aws:acmpca:CertificateAuthority
///   rootCertificateAuthorityCertificate:
///     type: aws:acmpca:CertificateAuthorityCertificate
///     name: root
///   rootCertificate:
///     type: aws:acmpca:Certificate
///     name: root
/// variables:
///   current:
///     fn::invoke:
///       function: aws:getPartition
///       arguments: {}
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod certificate_authority_certificate {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct CertificateAuthorityCertificateArgs {
        /// PEM-encoded certificate for the Certificate Authority.
        #[builder(into)]
        pub certificate: pulumi_gestalt_rust::InputOrOutput<String>,
        /// ARN of the Certificate Authority.
        #[builder(into)]
        pub certificate_authority_arn: pulumi_gestalt_rust::InputOrOutput<String>,
        /// PEM-encoded certificate chain that includes any intermediate certificates and chains up to root CA. Required for subordinate Certificate Authorities. Not allowed for root Certificate Authorities.
        #[builder(into, default)]
        pub certificate_chain: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct CertificateAuthorityCertificateResult {
        /// PEM-encoded certificate for the Certificate Authority.
        pub certificate: pulumi_gestalt_rust::Output<String>,
        /// ARN of the Certificate Authority.
        pub certificate_authority_arn: pulumi_gestalt_rust::Output<String>,
        /// PEM-encoded certificate chain that includes any intermediate certificates and chains up to root CA. Required for subordinate Certificate Authorities. Not allowed for root Certificate Authorities.
        pub certificate_chain: pulumi_gestalt_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: CertificateAuthorityCertificateArgs,
    ) -> CertificateAuthorityCertificateResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let certificate_binding = args.certificate.get_output(context);
        let certificate_authority_arn_binding = args
            .certificate_authority_arn
            .get_output(context);
        let certificate_chain_binding = args.certificate_chain.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:acmpca/certificateAuthorityCertificate:CertificateAuthorityCertificate"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "certificate".into(),
                    value: certificate_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "certificateAuthorityArn".into(),
                    value: certificate_authority_arn_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "certificateChain".into(),
                    value: certificate_chain_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        CertificateAuthorityCertificateResult {
            certificate: o.get_field("certificate"),
            certificate_authority_arn: o.get_field("certificateAuthorityArn"),
            certificate_chain: o.get_field("certificateChain"),
        }
    }
}
