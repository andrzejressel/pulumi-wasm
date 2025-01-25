/// Provides a resource, that manages Keyless certificates.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = keyless_certificate::create(
///         "example",
///         KeylessCertificateArgs::builder()
///             .bundle_method("ubiquitous")
///             .certificate("-----INSERT CERTIFICATE-----")
///             .enabled(true)
///             .host("example.com")
///             .name("example.com Keyless SSL")
///             .port(24008)
///             .zone_id("0da42c8d2132a9ddaf714f9e7c920711")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// ```sh
/// $ pulumi import cloudflare:index/keylessCertificate:KeylessCertificate example <zone_id>/<keyless_certificate_id>
/// ```
///
pub mod keyless_certificate {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct KeylessCertificateArgs {
        /// A ubiquitous bundle has the highest probability of being verified everywhere, even by clients using outdated or unusual trust stores. An optimal bundle uses the shortest chain and newest intermediates. And the force bundle verifies the chain, but does not otherwise modify it. Available values: `ubiquitous`, `optimal`, `force`. Defaults to `ubiquitous`. **Modifying this attribute will force creation of a new resource.**
        #[builder(into, default)]
        pub bundle_method: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The zone's SSL certificate or SSL certificate and intermediate(s). **Modifying this attribute will force creation of a new resource.**
        #[builder(into)]
        pub certificate: pulumi_wasm_rust::InputOrOutput<String>,
        /// Whether the KeyLess SSL is on.
        #[builder(into, default)]
        pub enabled: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
        /// The KeyLess SSL host.
        #[builder(into)]
        pub host: pulumi_wasm_rust::InputOrOutput<String>,
        /// The KeyLess SSL name.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The KeyLess SSL port used to communicate between Cloudflare and the client's KeyLess SSL server. Defaults to `24008`.
        #[builder(into, default)]
        pub port: pulumi_wasm_rust::InputOrOutput<Option<i32>>,
        /// The zone identifier to target for the resource.
        #[builder(into)]
        pub zone_id: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct KeylessCertificateResult {
        /// A ubiquitous bundle has the highest probability of being verified everywhere, even by clients using outdated or unusual trust stores. An optimal bundle uses the shortest chain and newest intermediates. And the force bundle verifies the chain, but does not otherwise modify it. Available values: `ubiquitous`, `optimal`, `force`. Defaults to `ubiquitous`. **Modifying this attribute will force creation of a new resource.**
        pub bundle_method: pulumi_wasm_rust::Output<Option<String>>,
        /// The zone's SSL certificate or SSL certificate and intermediate(s). **Modifying this attribute will force creation of a new resource.**
        pub certificate: pulumi_wasm_rust::Output<String>,
        /// Whether the KeyLess SSL is on.
        pub enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// The KeyLess SSL host.
        pub host: pulumi_wasm_rust::Output<String>,
        /// The KeyLess SSL name.
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// The KeyLess SSL port used to communicate between Cloudflare and the client's KeyLess SSL server. Defaults to `24008`.
        pub port: pulumi_wasm_rust::Output<Option<i32>>,
        /// Status of the KeyLess SSL.
        pub status: pulumi_wasm_rust::Output<String>,
        /// The zone identifier to target for the resource.
        pub zone_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: KeylessCertificateArgs,
    ) -> KeylessCertificateResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let bundle_method_binding = args.bundle_method.get_output(context).get_inner();
        let certificate_binding = args.certificate.get_output(context).get_inner();
        let enabled_binding = args.enabled.get_output(context).get_inner();
        let host_binding = args.host.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let port_binding = args.port.get_output(context).get_inner();
        let zone_id_binding = args.zone_id.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "cloudflare:index/keylessCertificate:KeylessCertificate".into(),
            name: name.to_string(),
            version: super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "bundleMethod".into(),
                    value: &bundle_method_binding,
                },
                register_interface::ObjectField {
                    name: "certificate".into(),
                    value: &certificate_binding,
                },
                register_interface::ObjectField {
                    name: "enabled".into(),
                    value: &enabled_binding,
                },
                register_interface::ObjectField {
                    name: "host".into(),
                    value: &host_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "port".into(),
                    value: &port_binding,
                },
                register_interface::ObjectField {
                    name: "zoneId".into(),
                    value: &zone_id_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "bundleMethod".into(),
                },
                register_interface::ResultField {
                    name: "certificate".into(),
                },
                register_interface::ResultField {
                    name: "enabled".into(),
                },
                register_interface::ResultField {
                    name: "host".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "port".into(),
                },
                register_interface::ResultField {
                    name: "status".into(),
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
        KeylessCertificateResult {
            bundle_method: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("bundleMethod").unwrap(),
            ),
            certificate: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("certificate").unwrap(),
            ),
            enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("enabled").unwrap(),
            ),
            host: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("host").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            port: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("port").unwrap(),
            ),
            status: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("status").unwrap(),
            ),
            zone_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("zoneId").unwrap(),
            ),
        }
    }
}
