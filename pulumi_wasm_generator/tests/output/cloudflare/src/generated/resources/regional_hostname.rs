/// Provides a Data Localization Suite Regional Hostname.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = record::create(
///         "example",
///         RecordArgs::builder()
///             .content("192.0.2.1")
///             .name("example.com")
///             .ttl(3600)
///             .type_("A")
///             .zone_id("0da42c8d2132a9ddaf714f9e7c920711")
///             .build_struct(),
///     );
///     let exampleRegionalHostname = regional_hostname::create(
///         "exampleRegionalHostname",
///         RegionalHostnameArgs::builder()
///             .hostname("example.com")
///             .region_key("eu")
///             .zone_id("0da42c8d2132a9ddaf714f9e7c920711")
///             .build_struct(),
///     );
/// }
/// ```
pub mod regional_hostname {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct RegionalHostnameArgs {
        /// The hostname to regionalize.
        #[builder(into)]
        pub hostname: pulumi_wasm_rust::Output<String>,
        /// The region key. See [the full region list](https://developers.cloudflare.com/data-localization/regional-services/get-started/).
        #[builder(into)]
        pub region_key: pulumi_wasm_rust::Output<String>,
        /// The zone identifier to target for the resource.
        #[builder(into)]
        pub zone_id: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct RegionalHostnameResult {
        /// The RFC3339 timestamp of when the hostname was created.
        pub created_on: pulumi_wasm_rust::Output<String>,
        /// The hostname to regionalize.
        pub hostname: pulumi_wasm_rust::Output<String>,
        /// The region key. See [the full region list](https://developers.cloudflare.com/data-localization/regional-services/get-started/).
        pub region_key: pulumi_wasm_rust::Output<String>,
        /// The zone identifier to target for the resource.
        pub zone_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: RegionalHostnameArgs) -> RegionalHostnameResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let hostname_binding = args.hostname.get_inner();
        let region_key_binding = args.region_key.get_inner();
        let zone_id_binding = args.zone_id.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "cloudflare:index/regionalHostname:RegionalHostname".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "hostname".into(),
                    value: &hostname_binding,
                },
                register_interface::ObjectField {
                    name: "regionKey".into(),
                    value: &region_key_binding,
                },
                register_interface::ObjectField {
                    name: "zoneId".into(),
                    value: &zone_id_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "createdOn".into(),
                },
                register_interface::ResultField {
                    name: "hostname".into(),
                },
                register_interface::ResultField {
                    name: "regionKey".into(),
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
        RegionalHostnameResult {
            created_on: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("createdOn").unwrap(),
            ),
            hostname: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("hostname").unwrap(),
            ),
            region_key: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("regionKey").unwrap(),
            ),
            zone_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("zoneId").unwrap(),
            ),
        }
    }
}
