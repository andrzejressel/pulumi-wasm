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
pub mod certificate {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct CertificateArgs {
        /// Specifies X.509 certificate information to be included in the issued certificate. To use with API Passthrough templates
        #[builder(into, default)]
        pub api_passthrough: pulumi_wasm_rust::Output<Option<String>>,
        /// ARN of the certificate authority.
        #[builder(into)]
        pub certificate_authority_arn: pulumi_wasm_rust::Output<String>,
        /// Certificate Signing Request in PEM format.
        #[builder(into)]
        pub certificate_signing_request: pulumi_wasm_rust::Output<String>,
        /// Algorithm to use to sign certificate requests. Valid values: `SHA256WITHRSA`, `SHA256WITHECDSA`, `SHA384WITHRSA`, `SHA384WITHECDSA`, `SHA512WITHRSA`, `SHA512WITHECDSA`.
        #[builder(into)]
        pub signing_algorithm: pulumi_wasm_rust::Output<String>,
        /// Template to use when issuing a certificate.
        /// See [ACM PCA Documentation](https://docs.aws.amazon.com/privateca/latest/userguide/UsingTemplates.html) for more information.
        #[builder(into, default)]
        pub template_arn: pulumi_wasm_rust::Output<Option<String>>,
        /// Configures end of the validity period for the certificate. See validity block below.
        #[builder(into)]
        pub validity: pulumi_wasm_rust::Output<
            super::super::types::acmpca::CertificateValidity,
        >,
    }
    #[allow(dead_code)]
    pub struct CertificateResult {
        /// Specifies X.509 certificate information to be included in the issued certificate. To use with API Passthrough templates
        pub api_passthrough: pulumi_wasm_rust::Output<Option<String>>,
        /// ARN of the certificate.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// PEM-encoded certificate value.
        pub certificate: pulumi_wasm_rust::Output<String>,
        /// ARN of the certificate authority.
        pub certificate_authority_arn: pulumi_wasm_rust::Output<String>,
        /// PEM-encoded certificate chain that includes any intermediate certificates and chains up to root CA.
        pub certificate_chain: pulumi_wasm_rust::Output<String>,
        /// Certificate Signing Request in PEM format.
        pub certificate_signing_request: pulumi_wasm_rust::Output<String>,
        /// Algorithm to use to sign certificate requests. Valid values: `SHA256WITHRSA`, `SHA256WITHECDSA`, `SHA384WITHRSA`, `SHA384WITHECDSA`, `SHA512WITHRSA`, `SHA512WITHECDSA`.
        pub signing_algorithm: pulumi_wasm_rust::Output<String>,
        /// Template to use when issuing a certificate.
        /// See [ACM PCA Documentation](https://docs.aws.amazon.com/privateca/latest/userguide/UsingTemplates.html) for more information.
        pub template_arn: pulumi_wasm_rust::Output<Option<String>>,
        /// Configures end of the validity period for the certificate. See validity block below.
        pub validity: pulumi_wasm_rust::Output<
            super::super::types::acmpca::CertificateValidity,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: CertificateArgs) -> CertificateResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let api_passthrough_binding = args.api_passthrough.get_inner();
        let certificate_authority_arn_binding = args
            .certificate_authority_arn
            .get_inner();
        let certificate_signing_request_binding = args
            .certificate_signing_request
            .get_inner();
        let signing_algorithm_binding = args.signing_algorithm.get_inner();
        let template_arn_binding = args.template_arn.get_inner();
        let validity_binding = args.validity.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:acmpca/certificate:Certificate".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "apiPassthrough".into(),
                    value: &api_passthrough_binding,
                },
                register_interface::ObjectField {
                    name: "certificateAuthorityArn".into(),
                    value: &certificate_authority_arn_binding,
                },
                register_interface::ObjectField {
                    name: "certificateSigningRequest".into(),
                    value: &certificate_signing_request_binding,
                },
                register_interface::ObjectField {
                    name: "signingAlgorithm".into(),
                    value: &signing_algorithm_binding,
                },
                register_interface::ObjectField {
                    name: "templateArn".into(),
                    value: &template_arn_binding,
                },
                register_interface::ObjectField {
                    name: "validity".into(),
                    value: &validity_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "apiPassthrough".into(),
                },
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "certificate".into(),
                },
                register_interface::ResultField {
                    name: "certificateAuthorityArn".into(),
                },
                register_interface::ResultField {
                    name: "certificateChain".into(),
                },
                register_interface::ResultField {
                    name: "certificateSigningRequest".into(),
                },
                register_interface::ResultField {
                    name: "signingAlgorithm".into(),
                },
                register_interface::ResultField {
                    name: "templateArn".into(),
                },
                register_interface::ResultField {
                    name: "validity".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        CertificateResult {
            api_passthrough: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("apiPassthrough").unwrap(),
            ),
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            certificate: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("certificate").unwrap(),
            ),
            certificate_authority_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("certificateAuthorityArn").unwrap(),
            ),
            certificate_chain: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("certificateChain").unwrap(),
            ),
            certificate_signing_request: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("certificateSigningRequest").unwrap(),
            ),
            signing_algorithm: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("signingAlgorithm").unwrap(),
            ),
            template_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("templateArn").unwrap(),
            ),
            validity: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("validity").unwrap(),
            ),
        }
    }
}
