/// Provides a Cloudflare Authenticated Origin Pulls certificate
/// resource. An uploaded client certificate is required to use Per-Zone
///  or Per-Hostname Authenticated Origin Pulls.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let myPerHostnameAopCert = authenticated_origin_pulls_certificate::create(
///         "myPerHostnameAopCert",
///         AuthenticatedOriginPullsCertificateArgs::builder()
///             .certificate("-----INSERT CERTIFICATE-----")
///             .private_key("-----INSERT PRIVATE KEY-----")
///             .type_("per-hostname")
///             .zone_id("0da42c8d2132a9ddaf714f9e7c920711")
///             .build_struct(),
///     );
///     let myPerZoneAopCert = authenticated_origin_pulls_certificate::create(
///         "myPerZoneAopCert",
///         AuthenticatedOriginPullsCertificateArgs::builder()
///             .certificate("-----INSERT CERTIFICATE-----")
///             .private_key("-----INSERT PRIVATE KEY-----")
///             .type_("per-zone")
///             .zone_id("0da42c8d2132a9ddaf714f9e7c920711")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// ```sh
/// $ pulumi import cloudflare:index/authenticatedOriginPullsCertificate:AuthenticatedOriginPullsCertificate example <zone_id>/<certificate_type>/<certificate_id>
/// ```
///
pub mod authenticated_origin_pulls_certificate {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct AuthenticatedOriginPullsCertificateArgs {
        /// The public client certificate. **Modifying this attribute will force creation of a new resource.**
        #[builder(into)]
        pub certificate: pulumi_wasm_rust::InputOrOutput<String>,
        /// The private key of the client certificate. **Modifying this attribute will force creation of a new resource.**
        #[builder(into)]
        pub private_key: pulumi_wasm_rust::InputOrOutput<String>,
        /// The form of Authenticated Origin Pulls to upload the certificate to. Available values: `per-zone`, `per-hostname`. **Modifying this attribute will force creation of a new resource.**
        #[builder(into)]
        pub type_: pulumi_wasm_rust::InputOrOutput<String>,
        /// The zone identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
        #[builder(into)]
        pub zone_id: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct AuthenticatedOriginPullsCertificateResult {
        /// The public client certificate. **Modifying this attribute will force creation of a new resource.**
        pub certificate: pulumi_wasm_rust::Output<String>,
        /// **Modifying this attribute will force creation of a new resource.**
        pub expires_on: pulumi_wasm_rust::Output<String>,
        /// **Modifying this attribute will force creation of a new resource.**
        pub issuer: pulumi_wasm_rust::Output<String>,
        /// The private key of the client certificate. **Modifying this attribute will force creation of a new resource.**
        pub private_key: pulumi_wasm_rust::Output<String>,
        /// **Modifying this attribute will force creation of a new resource.**
        pub serial_number: pulumi_wasm_rust::Output<String>,
        /// **Modifying this attribute will force creation of a new resource.**
        pub signature: pulumi_wasm_rust::Output<String>,
        /// **Modifying this attribute will force creation of a new resource.**
        pub status: pulumi_wasm_rust::Output<String>,
        /// The form of Authenticated Origin Pulls to upload the certificate to. Available values: `per-zone`, `per-hostname`. **Modifying this attribute will force creation of a new resource.**
        pub type_: pulumi_wasm_rust::Output<String>,
        /// **Modifying this attribute will force creation of a new resource.**
        pub uploaded_on: pulumi_wasm_rust::Output<String>,
        /// The zone identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
        pub zone_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: AuthenticatedOriginPullsCertificateArgs,
    ) -> AuthenticatedOriginPullsCertificateResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let certificate_binding = args.certificate.get_output(context).get_inner();
        let private_key_binding = args.private_key.get_output(context).get_inner();
        let type__binding = args.type_.get_output(context).get_inner();
        let zone_id_binding = args.zone_id.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "cloudflare:index/authenticatedOriginPullsCertificate:AuthenticatedOriginPullsCertificate"
                .into(),
            name: name.to_string(),
            version: super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "certificate".into(),
                    value: &certificate_binding,
                },
                register_interface::ObjectField {
                    name: "privateKey".into(),
                    value: &private_key_binding,
                },
                register_interface::ObjectField {
                    name: "type".into(),
                    value: &type__binding,
                },
                register_interface::ObjectField {
                    name: "zoneId".into(),
                    value: &zone_id_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "certificate".into(),
                },
                register_interface::ResultField {
                    name: "expiresOn".into(),
                },
                register_interface::ResultField {
                    name: "issuer".into(),
                },
                register_interface::ResultField {
                    name: "privateKey".into(),
                },
                register_interface::ResultField {
                    name: "serialNumber".into(),
                },
                register_interface::ResultField {
                    name: "signature".into(),
                },
                register_interface::ResultField {
                    name: "status".into(),
                },
                register_interface::ResultField {
                    name: "type".into(),
                },
                register_interface::ResultField {
                    name: "uploadedOn".into(),
                },
                register_interface::ResultField {
                    name: "zoneId".into(),
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        AuthenticatedOriginPullsCertificateResult {
            certificate: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("certificate").unwrap(),
            ),
            expires_on: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("expiresOn").unwrap(),
            ),
            issuer: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("issuer").unwrap(),
            ),
            private_key: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("privateKey").unwrap(),
            ),
            serial_number: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("serialNumber").unwrap(),
            ),
            signature: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("signature").unwrap(),
            ),
            status: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("status").unwrap(),
            ),
            type_: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("type").unwrap(),
            ),
            uploaded_on: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("uploadedOn").unwrap(),
            ),
            zone_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("zoneId").unwrap(),
            ),
        }
    }
}
