/// Provides a Cloudflare Origin CA certificate used to protect traffic to your origin without involving a third party Certificate Authority.
///
/// > Since v3.32.0
///    all authentication schemes are supported for managing Origin CA certificates.
///    Versions prior to v3.32.0 will still need to use `api_user_service_key`.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: tls:privateKey
///     properties:
///       algorithm: RSA
///   exampleCertRequest:
///     type: tls:certRequest
///     name: example
///     properties:
///       privateKeyPem: ${example.privateKeyPem}
///       subject:
///         - commonName:
///           organization: Terraform Test
///   exampleOriginCaCertificate:
///     type: cloudflare:OriginCaCertificate
///     name: example
///     properties:
///       csr: ${exampleCertRequest.certRequestPem}
///       hostnames:
///         - example.com
///       requestType: origin-rsa
///       requestedValidity: 7
/// ```
///
/// ## Import
///
/// ```sh
/// $ pulumi import cloudflare:index/originCaCertificate:OriginCaCertificate example <certificate_id>
/// ```
///
pub mod origin_ca_certificate {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct OriginCaCertificateArgs {
        /// The Certificate Signing Request. Must be newline-encoded. **Modifying this attribute will force creation of a new resource.**
        #[builder(into)]
        pub csr: pulumi_wasm_rust::Output<String>,
        /// A list of hostnames or wildcard names bound to the certificate. **Modifying this attribute will force creation of a new resource.**
        #[builder(into)]
        pub hostnames: pulumi_wasm_rust::Output<Vec<String>>,
        #[builder(into, default)]
        pub min_days_for_renewal: pulumi_wasm_rust::Output<Option<i32>>,
        /// The signature type desired on the certificate. Available values: `origin-rsa`, `origin-ecc`, `keyless-certificate`. **Modifying this attribute will force creation of a new resource.**
        #[builder(into)]
        pub request_type: pulumi_wasm_rust::Output<String>,
        /// The number of days for which the certificate should be valid. Available values: `7`, `30`, `90`, `365`, `730`, `1095`, `5475`. **Modifying this attribute will force creation of a new resource.**
        #[builder(into, default)]
        pub requested_validity: pulumi_wasm_rust::Output<Option<i32>>,
    }
    #[allow(dead_code)]
    pub struct OriginCaCertificateResult {
        /// The Origin CA certificate.
        pub certificate: pulumi_wasm_rust::Output<String>,
        /// The Certificate Signing Request. Must be newline-encoded. **Modifying this attribute will force creation of a new resource.**
        pub csr: pulumi_wasm_rust::Output<String>,
        /// The datetime when the certificate will expire.
        pub expires_on: pulumi_wasm_rust::Output<String>,
        /// A list of hostnames or wildcard names bound to the certificate. **Modifying this attribute will force creation of a new resource.**
        pub hostnames: pulumi_wasm_rust::Output<Vec<String>>,
        pub min_days_for_renewal: pulumi_wasm_rust::Output<Option<i32>>,
        /// The signature type desired on the certificate. Available values: `origin-rsa`, `origin-ecc`, `keyless-certificate`. **Modifying this attribute will force creation of a new resource.**
        pub request_type: pulumi_wasm_rust::Output<String>,
        /// The number of days for which the certificate should be valid. Available values: `7`, `30`, `90`, `365`, `730`, `1095`, `5475`. **Modifying this attribute will force creation of a new resource.**
        pub requested_validity: pulumi_wasm_rust::Output<i32>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: OriginCaCertificateArgs,
    ) -> OriginCaCertificateResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let csr_binding = args.csr.get_inner();
        let hostnames_binding = args.hostnames.get_inner();
        let min_days_for_renewal_binding = args.min_days_for_renewal.get_inner();
        let request_type_binding = args.request_type.get_inner();
        let requested_validity_binding = args.requested_validity.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "cloudflare:index/originCaCertificate:OriginCaCertificate".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "csr".into(),
                    value: &csr_binding,
                },
                register_interface::ObjectField {
                    name: "hostnames".into(),
                    value: &hostnames_binding,
                },
                register_interface::ObjectField {
                    name: "minDaysForRenewal".into(),
                    value: &min_days_for_renewal_binding,
                },
                register_interface::ObjectField {
                    name: "requestType".into(),
                    value: &request_type_binding,
                },
                register_interface::ObjectField {
                    name: "requestedValidity".into(),
                    value: &requested_validity_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "certificate".into(),
                },
                register_interface::ResultField {
                    name: "csr".into(),
                },
                register_interface::ResultField {
                    name: "expiresOn".into(),
                },
                register_interface::ResultField {
                    name: "hostnames".into(),
                },
                register_interface::ResultField {
                    name: "minDaysForRenewal".into(),
                },
                register_interface::ResultField {
                    name: "requestType".into(),
                },
                register_interface::ResultField {
                    name: "requestedValidity".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        OriginCaCertificateResult {
            certificate: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("certificate").unwrap(),
            ),
            csr: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("csr").unwrap(),
            ),
            expires_on: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("expiresOn").unwrap(),
            ),
            hostnames: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("hostnames").unwrap(),
            ),
            min_days_for_renewal: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("minDaysForRenewal").unwrap(),
            ),
            request_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("requestType").unwrap(),
            ),
            requested_validity: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("requestedValidity").unwrap(),
            ),
        }
    }
}
