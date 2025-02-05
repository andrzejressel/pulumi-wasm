/// Creates and manages an AWS IoT CA Certificate.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   ca:
///     type: tls:SelfSignedCert
///     properties:
///       privateKeyPem: ${caPrivateKey.privateKeyPem}
///       subject:
///         commonName: example.com
///         organization: ACME Examples, Inc
///       validityPeriodHours: 12
///       allowedUses:
///         - key_encipherment
///         - digital_signature
///         - server_auth
///       isCaCertificate: true
///   caPrivateKey:
///     type: tls:PrivateKey
///     name: ca
///     properties:
///       algorithm: RSA
///   verification:
///     type: tls:CertRequest
///     properties:
///       privateKeyPem: ${verificationPrivateKey.privateKeyPem}
///       subject:
///         commonName: ${example.registrationCode}
///   verificationPrivateKey:
///     type: tls:PrivateKey
///     name: verification
///     properties:
///       algorithm: RSA
///   verificationLocallySignedCert:
///     type: tls:LocallySignedCert
///     name: verification
///     properties:
///       certRequestPem: ${verification.certRequestPem}
///       caPrivateKeyPem: ${caPrivateKey.privateKeyPem}
///       caCertPem: ${ca.certPem}
///       validityPeriodHours: 12
///       allowedUses:
///         - key_encipherment
///         - digital_signature
///         - server_auth
///   exampleCaCertificate:
///     type: aws:iot:CaCertificate
///     name: example
///     properties:
///       active: true
///       caCertificatePem: ${ca.certPem}
///       verificationCertificatePem: ${verificationLocallySignedCert.certPem}
///       allowAutoRegistration: true
/// variables:
///   example:
///     fn::invoke:
///       function: aws:iot:getRegistrationCode
///       arguments: {}
/// ```
pub mod ca_certificate {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct CaCertificateArgs {
        /// Boolean flag to indicate if the certificate should be active for device authentication.
        #[builder(into)]
        pub active: pulumi_wasm_rust::InputOrOutput<bool>,
        /// Boolean flag to indicate if the certificate should be active for device regisration.
        #[builder(into)]
        pub allow_auto_registration: pulumi_wasm_rust::InputOrOutput<bool>,
        /// PEM encoded CA certificate.
        #[builder(into)]
        pub ca_certificate_pem: pulumi_wasm_rust::InputOrOutput<String>,
        /// The certificate mode in which the CA will be registered. Valida values: `DEFAULT` and `SNI_ONLY`. Default: `DEFAULT`.
        #[builder(into, default)]
        pub certificate_mode: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Information about the registration configuration. See below.
        #[builder(into, default)]
        pub registration_config: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::iot::CaCertificateRegistrationConfig>,
        >,
        /// A map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// PEM encoded verification certificate containing the common name of a registration code. Review
        /// [CreateVerificationCSR](https://docs.aws.amazon.com/iot/latest/developerguide/register-CA-cert.html). Reuired if `certificate_mode` is `DEFAULT`.
        #[builder(into, default)]
        pub verification_certificate_pem: pulumi_wasm_rust::InputOrOutput<
            Option<String>,
        >,
    }
    #[allow(dead_code)]
    pub struct CaCertificateResult {
        /// Boolean flag to indicate if the certificate should be active for device authentication.
        pub active: pulumi_wasm_rust::Output<bool>,
        /// Boolean flag to indicate if the certificate should be active for device regisration.
        pub allow_auto_registration: pulumi_wasm_rust::Output<bool>,
        /// The ARN of the created CA certificate.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// PEM encoded CA certificate.
        pub ca_certificate_pem: pulumi_wasm_rust::Output<String>,
        /// The certificate mode in which the CA will be registered. Valida values: `DEFAULT` and `SNI_ONLY`. Default: `DEFAULT`.
        pub certificate_mode: pulumi_wasm_rust::Output<Option<String>>,
        /// The customer version of the CA certificate.
        pub customer_version: pulumi_wasm_rust::Output<i32>,
        /// The generation ID of the CA certificate.
        pub generation_id: pulumi_wasm_rust::Output<String>,
        /// Information about the registration configuration. See below.
        pub registration_config: pulumi_wasm_rust::Output<
            Option<super::super::types::iot::CaCertificateRegistrationConfig>,
        >,
        /// A map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// When the CA certificate is valid.
        pub validities: pulumi_wasm_rust::Output<
            Vec<super::super::types::iot::CaCertificateValidity>,
        >,
        /// PEM encoded verification certificate containing the common name of a registration code. Review
        /// [CreateVerificationCSR](https://docs.aws.amazon.com/iot/latest/developerguide/register-CA-cert.html). Reuired if `certificate_mode` is `DEFAULT`.
        pub verification_certificate_pem: pulumi_wasm_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: CaCertificateArgs,
    ) -> CaCertificateResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let active_binding = args.active.get_output(context).get_inner();
        let allow_auto_registration_binding = args
            .allow_auto_registration
            .get_output(context)
            .get_inner();
        let ca_certificate_pem_binding = args
            .ca_certificate_pem
            .get_output(context)
            .get_inner();
        let certificate_mode_binding = args
            .certificate_mode
            .get_output(context)
            .get_inner();
        let registration_config_binding = args
            .registration_config
            .get_output(context)
            .get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let verification_certificate_pem_binding = args
            .verification_certificate_pem
            .get_output(context)
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:iot/caCertificate:CaCertificate".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "active".into(),
                    value: &active_binding,
                },
                register_interface::ObjectField {
                    name: "allowAutoRegistration".into(),
                    value: &allow_auto_registration_binding,
                },
                register_interface::ObjectField {
                    name: "caCertificatePem".into(),
                    value: &ca_certificate_pem_binding,
                },
                register_interface::ObjectField {
                    name: "certificateMode".into(),
                    value: &certificate_mode_binding,
                },
                register_interface::ObjectField {
                    name: "registrationConfig".into(),
                    value: &registration_config_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "verificationCertificatePem".into(),
                    value: &verification_certificate_pem_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        CaCertificateResult {
            active: pulumi_wasm_rust::__private::into_domain(o.extract_field("active")),
            allow_auto_registration: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("allowAutoRegistration"),
            ),
            arn: pulumi_wasm_rust::__private::into_domain(o.extract_field("arn")),
            ca_certificate_pem: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("caCertificatePem"),
            ),
            certificate_mode: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("certificateMode"),
            ),
            customer_version: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("customerVersion"),
            ),
            generation_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("generationId"),
            ),
            registration_config: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("registrationConfig"),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(o.extract_field("tags")),
            tags_all: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("tagsAll"),
            ),
            validities: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("validities"),
            ),
            verification_certificate_pem: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("verificationCertificatePem"),
            ),
        }
    }
}
