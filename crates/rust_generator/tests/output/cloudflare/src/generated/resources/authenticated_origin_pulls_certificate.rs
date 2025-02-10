/// Provides a Cloudflare Authenticated Origin Pulls certificate
/// resource. An uploaded client certificate is required to use Per-Zone
///  or Per-Hostname Authenticated Origin Pulls.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod authenticated_origin_pulls_certificate {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct AuthenticatedOriginPullsCertificateArgs {
        /// The public client certificate. **Modifying this attribute will force creation of a new resource.**
        #[builder(into)]
        pub certificate: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The private key of the client certificate. **Modifying this attribute will force creation of a new resource.**
        #[builder(into)]
        pub private_key: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The form of Authenticated Origin Pulls to upload the certificate to. Available values: `per-zone`, `per-hostname`. **Modifying this attribute will force creation of a new resource.**
        #[builder(into)]
        pub type_: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The zone identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
        #[builder(into)]
        pub zone_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct AuthenticatedOriginPullsCertificateResult {
        /// The public client certificate. **Modifying this attribute will force creation of a new resource.**
        pub certificate: pulumi_gestalt_rust::Output<String>,
        /// **Modifying this attribute will force creation of a new resource.**
        pub expires_on: pulumi_gestalt_rust::Output<String>,
        /// **Modifying this attribute will force creation of a new resource.**
        pub issuer: pulumi_gestalt_rust::Output<String>,
        /// The private key of the client certificate. **Modifying this attribute will force creation of a new resource.**
        pub private_key: pulumi_gestalt_rust::Output<String>,
        /// **Modifying this attribute will force creation of a new resource.**
        pub serial_number: pulumi_gestalt_rust::Output<String>,
        /// **Modifying this attribute will force creation of a new resource.**
        pub signature: pulumi_gestalt_rust::Output<String>,
        /// **Modifying this attribute will force creation of a new resource.**
        pub status: pulumi_gestalt_rust::Output<String>,
        /// The form of Authenticated Origin Pulls to upload the certificate to. Available values: `per-zone`, `per-hostname`. **Modifying this attribute will force creation of a new resource.**
        pub type_: pulumi_gestalt_rust::Output<String>,
        /// **Modifying this attribute will force creation of a new resource.**
        pub uploaded_on: pulumi_gestalt_rust::Output<String>,
        /// The zone identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
        pub zone_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: AuthenticatedOriginPullsCertificateArgs,
    ) -> AuthenticatedOriginPullsCertificateResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let certificate_binding = args.certificate.get_output(context);
        let private_key_binding = args.private_key.get_output(context);
        let type__binding = args.type_.get_output(context);
        let zone_id_binding = args.zone_id.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "cloudflare:index/authenticatedOriginPullsCertificate:AuthenticatedOriginPullsCertificate"
                .into(),
            name: name.to_string(),
            version: super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "certificate".into(),
                    value: certificate_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "privateKey".into(),
                    value: private_key_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "type".into(),
                    value: type__binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "zoneId".into(),
                    value: zone_id_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        AuthenticatedOriginPullsCertificateResult {
            certificate: o.get_field("certificate"),
            expires_on: o.get_field("expiresOn"),
            issuer: o.get_field("issuer"),
            private_key: o.get_field("privateKey"),
            serial_number: o.get_field("serialNumber"),
            signature: o.get_field("signature"),
            status: o.get_field("status"),
            type_: o.get_field("type"),
            uploaded_on: o.get_field("uploadedOn"),
            zone_id: o.get_field("zoneId"),
        }
    }
}
