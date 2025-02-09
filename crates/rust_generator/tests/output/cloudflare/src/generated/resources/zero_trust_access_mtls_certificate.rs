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
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: ZeroTrustAccessMtlsCertificateArgs,
    ) -> ZeroTrustAccessMtlsCertificateResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let account_id_binding_1 = args.account_id.get_output(context);
        let account_id_binding = account_id_binding_1.get_inner();
        let associated_hostnames_binding_1 = args
            .associated_hostnames
            .get_output(context);
        let associated_hostnames_binding = associated_hostnames_binding_1.get_inner();
        let certificate_binding_1 = args.certificate.get_output(context);
        let certificate_binding = certificate_binding_1.get_inner();
        let name_binding_1 = args.name.get_output(context);
        let name_binding = name_binding_1.get_inner();
        let zone_id_binding_1 = args.zone_id.get_output(context);
        let zone_id_binding = zone_id_binding_1.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "cloudflare:index/zeroTrustAccessMtlsCertificate:ZeroTrustAccessMtlsCertificate"
                .into(),
            name: name.to_string(),
            version: super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "accountId".into(),
                    value: &account_id_binding,
                },
                register_interface::ObjectField {
                    name: "associatedHostnames".into(),
                    value: &associated_hostnames_binding,
                },
                register_interface::ObjectField {
                    name: "certificate".into(),
                    value: &certificate_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "zoneId".into(),
                    value: &zone_id_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        ZeroTrustAccessMtlsCertificateResult {
            account_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("accountId"),
            ),
            associated_hostnames: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("associatedHostnames"),
            ),
            certificate: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("certificate"),
            ),
            fingerprint: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("fingerprint"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            zone_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("zoneId"),
            ),
        }
    }
}
