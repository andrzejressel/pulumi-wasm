/// Provides a Cloudflare mTLS certificate resource. These certificates may be used with mTLS enabled Cloudflare services.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod mtls_certificate {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct MtlsCertificateArgs {
        /// The account identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
        #[builder(into)]
        pub account_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Whether this is a CA or leaf certificate. **Modifying this attribute will force creation of a new resource.**
        #[builder(into)]
        pub ca: pulumi_gestalt_rust::InputOrOutput<bool>,
        /// Certificate you intend to use with mTLS-enabled services. **Modifying this attribute will force creation of a new resource.**
        #[builder(into)]
        pub certificates: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Optional unique name for the certificate. **Modifying this attribute will force creation of a new resource.**
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The certificate's private key. **Modifying this attribute will force creation of a new resource.**
        #[builder(into, default)]
        pub private_key: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct MtlsCertificateResult {
        /// The account identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
        pub account_id: pulumi_gestalt_rust::Output<String>,
        /// Whether this is a CA or leaf certificate. **Modifying this attribute will force creation of a new resource.**
        pub ca: pulumi_gestalt_rust::Output<bool>,
        /// Certificate you intend to use with mTLS-enabled services. **Modifying this attribute will force creation of a new resource.**
        pub certificates: pulumi_gestalt_rust::Output<String>,
        /// **Modifying this attribute will force creation of a new resource.**
        pub expires_on: pulumi_gestalt_rust::Output<String>,
        /// **Modifying this attribute will force creation of a new resource.**
        pub issuer: pulumi_gestalt_rust::Output<String>,
        /// Optional unique name for the certificate. **Modifying this attribute will force creation of a new resource.**
        pub name: pulumi_gestalt_rust::Output<Option<String>>,
        /// The certificate's private key. **Modifying this attribute will force creation of a new resource.**
        pub private_key: pulumi_gestalt_rust::Output<Option<String>>,
        /// **Modifying this attribute will force creation of a new resource.**
        pub serial_number: pulumi_gestalt_rust::Output<String>,
        /// **Modifying this attribute will force creation of a new resource.**
        pub signature: pulumi_gestalt_rust::Output<String>,
        /// **Modifying this attribute will force creation of a new resource.**
        pub uploaded_on: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: MtlsCertificateArgs,
    ) -> MtlsCertificateResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let account_id_binding_1 = args.account_id.get_output(context);
        let account_id_binding = account_id_binding_1.get_inner();
        let ca_binding_1 = args.ca.get_output(context);
        let ca_binding = ca_binding_1.get_inner();
        let certificates_binding_1 = args.certificates.get_output(context);
        let certificates_binding = certificates_binding_1.get_inner();
        let name_binding_1 = args.name.get_output(context);
        let name_binding = name_binding_1.get_inner();
        let private_key_binding_1 = args.private_key.get_output(context);
        let private_key_binding = private_key_binding_1.get_inner();
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
        };
        let o = register_interface::register(context.get_inner(), &request);
        MtlsCertificateResult {
            account_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("accountId"),
            ),
            ca: pulumi_gestalt_rust::__private::into_domain(o.extract_field("ca")),
            certificates: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("certificates"),
            ),
            expires_on: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("expiresOn"),
            ),
            issuer: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("issuer"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            private_key: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("privateKey"),
            ),
            serial_number: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("serialNumber"),
            ),
            signature: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("signature"),
            ),
            uploaded_on: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("uploadedOn"),
            ),
        }
    }
}
