/// Provides a Cloudflare Zone Lockdown resource. Zone Lockdown allows
/// you to define one or more URLs (with wildcard matching on the domain
/// or path) that will only permit access if the request originates
/// from an IP address that matches a safelist of one or more IP
/// addresses and/or IP ranges.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   # Restrict access to these endpoints to requests from a known IP address range.
///   example:
///     type: cloudflare:ZoneLockdown
///     properties:
///       zoneId: 0da42c8d2132a9ddaf714f9e7c920711
///       paused: 'false'
///       description: Restrict access to these endpoints to requests from a known IP address range
///       urls:
///         - api.mysite.com/some/endpoint*
///       configurations:
///         - target: ip_range
///           value: 192.0.2.0/24
/// ```
///
/// ## Import
///
/// ```sh
/// $ pulumi import cloudflare:index/zoneLockdown:ZoneLockdown example <zone_id>/<lockdown_id>
/// ```
///
pub mod zone_lockdown {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ZoneLockdownArgs {
        /// A list of IP addresses or IP ranges to match the request against specified in target, value pairs.
        #[builder(into)]
        pub configurations: pulumi_wasm_rust::Output<
            Vec<super::types::ZoneLockdownConfiguration>,
        >,
        /// A description about the lockdown entry. Typically used as a reminder or explanation for the lockdown.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// Boolean of whether this zone lockdown is currently paused. Defaults to `false`.
        #[builder(into, default)]
        pub paused: pulumi_wasm_rust::Output<Option<bool>>,
        #[builder(into, default)]
        pub priority: pulumi_wasm_rust::Output<Option<i32>>,
        /// A list of simple wildcard patterns to match requests against. The order of the urls is unimportant.
        #[builder(into)]
        pub urls: pulumi_wasm_rust::Output<Vec<String>>,
        /// The zone identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
        #[builder(into)]
        pub zone_id: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct ZoneLockdownResult {
        /// A list of IP addresses or IP ranges to match the request against specified in target, value pairs.
        pub configurations: pulumi_wasm_rust::Output<
            Vec<super::types::ZoneLockdownConfiguration>,
        >,
        /// A description about the lockdown entry. Typically used as a reminder or explanation for the lockdown.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// Boolean of whether this zone lockdown is currently paused. Defaults to `false`.
        pub paused: pulumi_wasm_rust::Output<Option<bool>>,
        pub priority: pulumi_wasm_rust::Output<Option<i32>>,
        /// A list of simple wildcard patterns to match requests against. The order of the urls is unimportant.
        pub urls: pulumi_wasm_rust::Output<Vec<String>>,
        /// The zone identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
        pub zone_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: ZoneLockdownArgs) -> ZoneLockdownResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let configurations_binding = args.configurations.get_inner();
        let description_binding = args.description.get_inner();
        let paused_binding = args.paused.get_inner();
        let priority_binding = args.priority.get_inner();
        let urls_binding = args.urls.get_inner();
        let zone_id_binding = args.zone_id.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "cloudflare:index/zoneLockdown:ZoneLockdown".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "configurations".into(),
                    value: &configurations_binding,
                },
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "paused".into(),
                    value: &paused_binding,
                },
                register_interface::ObjectField {
                    name: "priority".into(),
                    value: &priority_binding,
                },
                register_interface::ObjectField {
                    name: "urls".into(),
                    value: &urls_binding,
                },
                register_interface::ObjectField {
                    name: "zoneId".into(),
                    value: &zone_id_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "configurations".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "paused".into(),
                },
                register_interface::ResultField {
                    name: "priority".into(),
                },
                register_interface::ResultField {
                    name: "urls".into(),
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
        ZoneLockdownResult {
            configurations: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("configurations").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            paused: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("paused").unwrap(),
            ),
            priority: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("priority").unwrap(),
            ),
            urls: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("urls").unwrap(),
            ),
            zone_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("zoneId").unwrap(),
            ),
        }
    }
}
