/// Allows management of the Logpull Retention settings used to control whether or not to retain HTTP request logs.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: cloudflare:LogpullRetention
///     properties:
///       zoneId: 0da42c8d2132a9ddaf714f9e7c920711
///       enabled: 'true'
/// ```
///
/// ## Import
///
/// ```sh
/// $ pulumi import cloudflare:index/logpullRetention:LogpullRetention example <zone_id>
/// ```
///
pub mod logpull_retention {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct LogpullRetentionArgs {
        /// Whether you wish to retain logs or not.
        #[builder(into)]
        pub enabled: pulumi_wasm_rust::Output<bool>,
        /// The zone identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
        #[builder(into)]
        pub zone_id: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct LogpullRetentionResult {
        /// Whether you wish to retain logs or not.
        pub enabled: pulumi_wasm_rust::Output<bool>,
        /// The zone identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
        pub zone_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: LogpullRetentionArgs) -> LogpullRetentionResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let enabled_binding = args.enabled.get_inner();
        let zone_id_binding = args.zone_id.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "cloudflare:index/logpullRetention:LogpullRetention".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "enabled".into(),
                    value: &enabled_binding,
                },
                register_interface::ObjectField {
                    name: "zoneId".into(),
                    value: &zone_id_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "enabled".into(),
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
        LogpullRetentionResult {
            enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("enabled").unwrap(),
            ),
            zone_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("zoneId").unwrap(),
            ),
        }
    }
}
