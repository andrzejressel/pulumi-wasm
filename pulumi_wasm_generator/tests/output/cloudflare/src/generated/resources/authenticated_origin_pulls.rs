/// Provides a Cloudflare Authenticated Origin Pulls resource. A `cloudflare.AuthenticatedOriginPulls`
/// resource is required to use Per-Zone or Per-Hostname Authenticated
/// Origin Pulls.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let myAop = authenticated_origin_pulls::create(
///         "myAop",
///         AuthenticatedOriginPullsArgs::builder()
///             .enabled(true)
///             .zone_id("0da42c8d2132a9ddaf714f9e7c920711")
///             .build_struct(),
///     );
///     let myPerHostnameAop = authenticated_origin_pulls::create(
///         "myPerHostnameAop",
///         AuthenticatedOriginPullsArgs::builder()
///             .authenticated_origin_pulls_certificate("${myPerHostnameAopCert.id}")
///             .enabled(true)
///             .hostname("aop.example.com")
///             .zone_id("0da42c8d2132a9ddaf714f9e7c920711")
///             .build_struct(),
///     );
///     let myPerHostnameAopCert = authenticated_origin_pulls_certificate::create(
///         "myPerHostnameAopCert",
///         AuthenticatedOriginPullsCertificateArgs::builder()
///             .certificate("-----INSERT CERTIFICATE-----")
///             .private_key("-----INSERT PRIVATE KEY-----")
///             .type_("per-hostname")
///             .zone_id("0da42c8d2132a9ddaf714f9e7c920711")
///             .build_struct(),
///     );
///     let myPerZoneAop = authenticated_origin_pulls::create(
///         "myPerZoneAop",
///         AuthenticatedOriginPullsArgs::builder()
///             .authenticated_origin_pulls_certificate("${myPerZoneAopCert.id}")
///             .enabled(true)
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
/// global
///
/// ```sh
/// $ pulumi import cloudflare:index/authenticatedOriginPulls:AuthenticatedOriginPulls example <zone_id>
/// ```
///
/// per zone
///
/// ```sh
/// $ pulumi import cloudflare:index/authenticatedOriginPulls:AuthenticatedOriginPulls example <zone_id>/<certificate_id>
/// ```
///
/// per hostname
///
/// ```sh
/// $ pulumi import cloudflare:index/authenticatedOriginPulls:AuthenticatedOriginPulls example <zone_id>/<certificate_id>/<hostname>
/// ```
///
pub mod authenticated_origin_pulls {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct AuthenticatedOriginPullsArgs {
        /// The ID of an uploaded Authenticated Origin Pulls certificate. If no hostname is provided, this certificate will be used zone wide as Per-Zone Authenticated Origin Pulls.
        #[builder(into, default)]
        pub authenticated_origin_pulls_certificate: pulumi_wasm_rust::Output<
            Option<String>,
        >,
        /// Whether to enable Authenticated Origin Pulls on the given zone or hostname.
        #[builder(into)]
        pub enabled: pulumi_wasm_rust::Output<bool>,
        /// Specify a hostname to enable Per-Hostname Authenticated Origin Pulls on, using the provided certificate.
        #[builder(into, default)]
        pub hostname: pulumi_wasm_rust::Output<Option<String>>,
        /// The zone identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
        #[builder(into)]
        pub zone_id: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct AuthenticatedOriginPullsResult {
        /// The ID of an uploaded Authenticated Origin Pulls certificate. If no hostname is provided, this certificate will be used zone wide as Per-Zone Authenticated Origin Pulls.
        pub authenticated_origin_pulls_certificate: pulumi_wasm_rust::Output<
            Option<String>,
        >,
        /// Whether to enable Authenticated Origin Pulls on the given zone or hostname.
        pub enabled: pulumi_wasm_rust::Output<bool>,
        /// Specify a hostname to enable Per-Hostname Authenticated Origin Pulls on, using the provided certificate.
        pub hostname: pulumi_wasm_rust::Output<Option<String>>,
        /// The zone identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
        pub zone_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: AuthenticatedOriginPullsArgs,
    ) -> AuthenticatedOriginPullsResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let authenticated_origin_pulls_certificate_binding = args
            .authenticated_origin_pulls_certificate
            .get_inner();
        let enabled_binding = args.enabled.get_inner();
        let hostname_binding = args.hostname.get_inner();
        let zone_id_binding = args.zone_id.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "cloudflare:index/authenticatedOriginPulls:AuthenticatedOriginPulls"
                .into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "authenticatedOriginPullsCertificate".into(),
                    value: &authenticated_origin_pulls_certificate_binding,
                },
                register_interface::ObjectField {
                    name: "enabled".into(),
                    value: &enabled_binding,
                },
                register_interface::ObjectField {
                    name: "hostname".into(),
                    value: &hostname_binding,
                },
                register_interface::ObjectField {
                    name: "zoneId".into(),
                    value: &zone_id_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "authenticatedOriginPullsCertificate".into(),
                },
                register_interface::ResultField {
                    name: "enabled".into(),
                },
                register_interface::ResultField {
                    name: "hostname".into(),
                },
                register_interface::ResultField {
                    name: "zoneId".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        AuthenticatedOriginPullsResult {
            authenticated_origin_pulls_certificate: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("authenticatedOriginPullsCertificate").unwrap(),
            ),
            enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("enabled").unwrap(),
            ),
            hostname: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("hostname").unwrap(),
            ),
            zone_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("zoneId").unwrap(),
            ),
        }
    }
}
