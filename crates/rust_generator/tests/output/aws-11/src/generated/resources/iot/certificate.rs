/// Creates and manages an AWS IoT certificate.
///
/// ## Example Usage
///
/// ### With CSR
///
/// ```yaml
/// resources:
///   cert:
///     type: aws:iot:Certificate
///     properties:
///       csr:
///         fn::invoke:
///           function: std:file
///           arguments:
///             input: /my/csr.pem
///           return: result
///       active: true
/// ```
///
/// ### Without CSR
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let cert = certificate::create(
///         "cert",
///         CertificateArgs::builder().active(true).build_struct(),
///     );
/// }
/// ```
///
/// ### From existing certificate without a CA
///
/// ```yaml
/// resources:
///   cert:
///     type: aws:iot:Certificate
///     properties:
///       certificatePem:
///         fn::invoke:
///           function: std:file
///           arguments:
///             input: /my/cert.pem
///           return: result
///       active: true
/// ```
pub mod certificate {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct CertificateArgs {
        /// Boolean flag to indicate if the certificate should be active
        #[builder(into)]
        pub active: pulumi_wasm_rust::InputOrOutput<bool>,
        /// The CA certificate for the certificate to be registered. If this is set, the CA needs to be registered with AWS IoT beforehand.
        #[builder(into, default)]
        pub ca_pem: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The certificate to be registered. If `ca_pem` is unspecified, review
        /// [RegisterCertificateWithoutCA](https://docs.aws.amazon.com/iot/latest/apireference/API_RegisterCertificateWithoutCA.html).
        /// If `ca_pem` is specified, review
        /// [RegisterCertificate](https://docs.aws.amazon.com/iot/latest/apireference/API_RegisterCertificate.html)
        /// for more information on registering a certificate.
        #[builder(into, default)]
        pub certificate_pem: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The certificate signing request. Review
        /// [CreateCertificateFromCsr](https://docs.aws.amazon.com/iot/latest/apireference/API_CreateCertificateFromCsr.html)
        /// for more information on generating a certificate from a certificate signing request (CSR).
        /// If none is specified both the certificate and keys will be generated, review [CreateKeysAndCertificate](https://docs.aws.amazon.com/iot/latest/apireference/API_CreateKeysAndCertificate.html)
        /// for more information on generating keys and a certificate.
        #[builder(into, default)]
        pub csr: pulumi_wasm_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct CertificateResult {
        /// Boolean flag to indicate if the certificate should be active
        pub active: pulumi_wasm_rust::Output<bool>,
        /// The ARN of the created certificate.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// The certificate ID of the CA certificate used to sign the certificate.
        pub ca_certificate_id: pulumi_wasm_rust::Output<String>,
        /// The CA certificate for the certificate to be registered. If this is set, the CA needs to be registered with AWS IoT beforehand.
        pub ca_pem: pulumi_wasm_rust::Output<Option<String>>,
        /// The certificate to be registered. If `ca_pem` is unspecified, review
        /// [RegisterCertificateWithoutCA](https://docs.aws.amazon.com/iot/latest/apireference/API_RegisterCertificateWithoutCA.html).
        /// If `ca_pem` is specified, review
        /// [RegisterCertificate](https://docs.aws.amazon.com/iot/latest/apireference/API_RegisterCertificate.html)
        /// for more information on registering a certificate.
        pub certificate_pem: pulumi_wasm_rust::Output<String>,
        /// The certificate signing request. Review
        /// [CreateCertificateFromCsr](https://docs.aws.amazon.com/iot/latest/apireference/API_CreateCertificateFromCsr.html)
        /// for more information on generating a certificate from a certificate signing request (CSR).
        /// If none is specified both the certificate and keys will be generated, review [CreateKeysAndCertificate](https://docs.aws.amazon.com/iot/latest/apireference/API_CreateKeysAndCertificate.html)
        /// for more information on generating keys and a certificate.
        pub csr: pulumi_wasm_rust::Output<Option<String>>,
        /// When neither CSR nor certificate is provided, the private key.
        pub private_key: pulumi_wasm_rust::Output<String>,
        /// When neither CSR nor certificate is provided, the public key.
        pub public_key: pulumi_wasm_rust::Output<String>,
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
        let active_binding = args.active.get_output(context).get_inner();
        let ca_pem_binding = args.ca_pem.get_output(context).get_inner();
        let certificate_pem_binding = args
            .certificate_pem
            .get_output(context)
            .get_inner();
        let csr_binding = args.csr.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:iot/certificate:Certificate".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "active".into(),
                    value: &active_binding,
                },
                register_interface::ObjectField {
                    name: "caPem".into(),
                    value: &ca_pem_binding,
                },
                register_interface::ObjectField {
                    name: "certificatePem".into(),
                    value: &certificate_pem_binding,
                },
                register_interface::ObjectField {
                    name: "csr".into(),
                    value: &csr_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        CertificateResult {
            active: pulumi_wasm_rust::__private::into_domain(o.extract_field("active")),
            arn: pulumi_wasm_rust::__private::into_domain(o.extract_field("arn")),
            ca_certificate_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("caCertificateId"),
            ),
            ca_pem: pulumi_wasm_rust::__private::into_domain(o.extract_field("caPem")),
            certificate_pem: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("certificatePem"),
            ),
            csr: pulumi_wasm_rust::__private::into_domain(o.extract_field("csr")),
            private_key: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("privateKey"),
            ),
            public_key: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("publicKey"),
            ),
        }
    }
}
