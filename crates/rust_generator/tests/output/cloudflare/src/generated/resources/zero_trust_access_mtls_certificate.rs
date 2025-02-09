/// Provides a Cloudflare Access Mutual TLS Certificate resource.
/// Mutual TLS authentication ensures that the traffic is secure and
/// trusted in both directions between a client and server and can be
///  used with Access to only allows requests from devices with a
///  corresponding client certificate.
///
/// > It's required that an `account_id` or `zone_id` is provided and in
///    most cases using either is fine. However, if you're using a scoped
///    access token, you must provide the argument that matches the token's
///    scope. For example, an access token that is scoped to the "example.com"
///    zone needs to use the `zone_id` argument.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let myCert = zero_trust_access_mtls_certificate::create(
///         "myCert",
///         ZeroTrustAccessMtlsCertificateArgs::builder()
///             .associated_hostnames(vec!["staging.example.com",])
///             .certificate("${caPem}")
///             .name("My Root Cert")
///             .zone_id("0da42c8d2132a9ddaf714f9e7c920711")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Account level import.
///
/// ```sh
/// $ pulumi import cloudflare:index/zeroTrustAccessMtlsCertificate:ZeroTrustAccessMtlsCertificate cloudflare_zero_sd -t_access_mtls_certificate.example account/<account_id>/<mutual_tls_certificate_id>
/// ```
///
/// Zone level import.
///
/// ```sh
/// $ pulumi import cloudflare:index/zeroTrustAccessMtlsCertificate:ZeroTrustAccessMtlsCertificate cloudflare_zero_sd -t_access_mtls_certificate.example zone/<zone_id>/<mutual_tls_certificate_id>
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod zero_trust_access_mtls_certificate {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ZeroTrustAccessMtlsCertificateArgs {
        /// The account identifier to target for the resource. Conflicts with `zone_id`.
        #[builder(into, default)]
        pub account_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The hostnames that will be prompted for this certificate.
        #[builder(into, default)]
        pub associated_hostnames: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<String>>,
        >,
        /// The Root CA for your certificates.
        #[builder(into, default)]
        pub certificate: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of the certificate.
        #[builder(into)]
        pub name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The zone identifier to target for the resource. Conflicts with `account_id`.
        #[builder(into, default)]
        pub zone_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct ZeroTrustAccessMtlsCertificateResult {
        /// The account identifier to target for the resource. Conflicts with `zone_id`.
        pub account_id: pulumi_gestalt_rust::Output<String>,
        /// The hostnames that will be prompted for this certificate.
        pub associated_hostnames: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// The Root CA for your certificates.
        pub certificate: pulumi_gestalt_rust::Output<Option<String>>,
        pub fingerprint: pulumi_gestalt_rust::Output<String>,
        /// The name of the certificate.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The zone identifier to target for the resource. Conflicts with `account_id`.
        pub zone_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ZeroTrustAccessMtlsCertificateArgs,
    ) -> ZeroTrustAccessMtlsCertificateResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let account_id_binding = args.account_id.get_output(context);
        let associated_hostnames_binding = args.associated_hostnames.get_output(context);
        let certificate_binding = args.certificate.get_output(context);
        let name_binding = args.name.get_output(context);
        let zone_id_binding = args.zone_id.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "cloudflare:index/zeroTrustAccessMtlsCertificate:ZeroTrustAccessMtlsCertificate"
                .into(),
            name: name.to_string(),
            version: super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "accountId".into(),
                    value: account_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "associatedHostnames".into(),
                    value: associated_hostnames_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "certificate".into(),
                    value: certificate_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "zoneId".into(),
                    value: zone_id_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        ZeroTrustAccessMtlsCertificateResult {
            account_id: o.get_field("accountId"),
            associated_hostnames: o.get_field("associatedHostnames"),
            certificate: o.get_field("certificate"),
            fingerprint: o.get_field("fingerprint"),
            name: o.get_field("name"),
            zone_id: o.get_field("zoneId"),
        }
    }
}
