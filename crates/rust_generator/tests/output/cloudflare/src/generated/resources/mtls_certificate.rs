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
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: MtlsCertificateArgs,
    ) -> MtlsCertificateResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let account_id_binding = args.account_id.get_output(context);
        let ca_binding = args.ca.get_output(context);
        let certificates_binding = args.certificates.get_output(context);
        let name_binding = args.name.get_output(context);
        let private_key_binding = args.private_key.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "cloudflare:index/mtlsCertificate:MtlsCertificate".into(),
            name: name.to_string(),
            version: super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "accountId".into(),
                    value: &account_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "ca".into(),
                    value: &ca_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "certificates".into(),
                    value: &certificates_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "privateKey".into(),
                    value: &private_key_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        MtlsCertificateResult {
            account_id: o.get_field("accountId"),
            ca: o.get_field("ca"),
            certificates: o.get_field("certificates"),
            expires_on: o.get_field("expiresOn"),
            issuer: o.get_field("issuer"),
            name: o.get_field("name"),
            private_key: o.get_field("privateKey"),
            serial_number: o.get_field("serialNumber"),
            signature: o.get_field("signature"),
            uploaded_on: o.get_field("uploadedOn"),
        }
    }
}
