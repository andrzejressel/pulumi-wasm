/// Provides a Cloudflare mTLS certificate resource. These certificates may be used with mTLS enabled Cloudflare services.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = mtls_certificate::create(
///         "example",
///         MtlsCertificateArgs::builder()
///             .account_id("f037e56e89293a057740de681ac9abbe")
///             .ca(true)
///             .certificates(
///                 "-----BEGIN CERTIFICATE-----\nMIIDmDCCAoCgAwIBAgIUKTOAZNj...i4JhqeoTewsxndhDDE\n-----END CERTIFICATE-----",
///             )
///             .name("example")
///             .private_key(
///                 "-----BEGIN PRIVATE KEY-----\nMIIEvQIBADANBgkqhkiG9w0BAQE...1IS3EnQRrz6WMYA=\n-----END PRIVATE KEY-----",
///             )
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// ```sh
/// $ pulumi import cloudflare:index/mtlsCertificate:MtlsCertificate example <account_id>/<mtls_certificate_id>
/// ```
///
pub mod mtls_certificate {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct MtlsCertificateArgs {
        /// The account identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
        #[builder(into)]
        pub account_id: pulumi_wasm_rust::InputOrOutput<String>,
        /// Whether this is a CA or leaf certificate. **Modifying this attribute will force creation of a new resource.**
        #[builder(into)]
        pub ca: pulumi_wasm_rust::InputOrOutput<bool>,
        /// Certificate you intend to use with mTLS-enabled services. **Modifying this attribute will force creation of a new resource.**
        #[builder(into)]
        pub certificates: pulumi_wasm_rust::InputOrOutput<String>,
        /// Optional unique name for the certificate. **Modifying this attribute will force creation of a new resource.**
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The certificate's private key. **Modifying this attribute will force creation of a new resource.**
        #[builder(into, default)]
        pub private_key: pulumi_wasm_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct MtlsCertificateResult {
        /// The account identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
        pub account_id: pulumi_wasm_rust::Output<String>,
        /// Whether this is a CA or leaf certificate. **Modifying this attribute will force creation of a new resource.**
        pub ca: pulumi_wasm_rust::Output<bool>,
        /// Certificate you intend to use with mTLS-enabled services. **Modifying this attribute will force creation of a new resource.**
        pub certificates: pulumi_wasm_rust::Output<String>,
        /// **Modifying this attribute will force creation of a new resource.**
        pub expires_on: pulumi_wasm_rust::Output<String>,
        /// **Modifying this attribute will force creation of a new resource.**
        pub issuer: pulumi_wasm_rust::Output<String>,
        /// Optional unique name for the certificate. **Modifying this attribute will force creation of a new resource.**
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// The certificate's private key. **Modifying this attribute will force creation of a new resource.**
        pub private_key: pulumi_wasm_rust::Output<Option<String>>,
        /// **Modifying this attribute will force creation of a new resource.**
        pub serial_number: pulumi_wasm_rust::Output<String>,
        /// **Modifying this attribute will force creation of a new resource.**
        pub signature: pulumi_wasm_rust::Output<String>,
        /// **Modifying this attribute will force creation of a new resource.**
        pub uploaded_on: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: MtlsCertificateArgs,
    ) -> MtlsCertificateResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let account_id_binding = args.account_id.get_output(context).get_inner();
        let ca_binding = args.ca.get_output(context).get_inner();
        let certificates_binding = args.certificates.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let private_key_binding = args.private_key.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "cloudflare:index/mtlsCertificate:MtlsCertificate".into(),
            name: name.to_string(),
            version: super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "accountId".into(),
                    value: &account_id_binding,
                },
                register_interface::ObjectField {
                    name: "ca".into(),
                    value: &ca_binding,
                },
                register_interface::ObjectField {
                    name: "certificates".into(),
                    value: &certificates_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "privateKey".into(),
                    value: &private_key_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "accountId".into(),
                },
                register_interface::ResultField {
                    name: "ca".into(),
                },
                register_interface::ResultField {
                    name: "certificates".into(),
                },
                register_interface::ResultField {
                    name: "expiresOn".into(),
                },
                register_interface::ResultField {
                    name: "issuer".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
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
                    name: "uploadedOn".into(),
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        MtlsCertificateResult {
            account_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("accountId").unwrap(),
            ),
            ca: pulumi_wasm_rust::__private::into_domain(hashmap.remove("ca").unwrap()),
            certificates: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("certificates").unwrap(),
            ),
            expires_on: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("expiresOn").unwrap(),
            ),
            issuer: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("issuer").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
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
            uploaded_on: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("uploadedOn").unwrap(),
            ),
        }
    }
}
