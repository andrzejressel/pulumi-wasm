/// Cloudflare Argo controls the routing to your origin and tiered
/// caching options to speed up your website browsing experience.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = argo::create(
///         "example",
///         ArgoArgs::builder()
///             .smart_routing("on")
///             .tiered_caching("on")
///             .zone_id("0da42c8d2132a9ddaf714f9e7c920711")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// ```sh
/// $ pulumi import cloudflare:index/argo:Argo example <zone_id>
/// ```
///
pub mod argo {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ArgoArgs {
        /// Whether smart routing is enabled. Available values: `on`, `off`.
        #[builder(into, default)]
        pub smart_routing: pulumi_wasm_rust::Output<Option<String>>,
        /// Whether tiered caching is enabled. Available values: `on`, `off`.
        #[builder(into, default)]
        pub tiered_caching: pulumi_wasm_rust::Output<Option<String>>,
        /// The zone identifier to target for the resource.
        #[builder(into)]
        pub zone_id: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct ArgoResult {
        /// Whether smart routing is enabled. Available values: `on`, `off`.
        pub smart_routing: pulumi_wasm_rust::Output<Option<String>>,
        /// Whether tiered caching is enabled. Available values: `on`, `off`.
        pub tiered_caching: pulumi_wasm_rust::Output<Option<String>>,
        /// The zone identifier to target for the resource.
        pub zone_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: ArgoArgs) -> ArgoResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let smart_routing_binding = args.smart_routing.get_inner();
        let tiered_caching_binding = args.tiered_caching.get_inner();
        let zone_id_binding = args.zone_id.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "cloudflare:index/argo:Argo".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "smartRouting".into(),
                    value: &smart_routing_binding,
                },
                register_interface::ObjectField {
                    name: "tieredCaching".into(),
                    value: &tiered_caching_binding,
                },
                register_interface::ObjectField {
                    name: "zoneId".into(),
                    value: &zone_id_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "smartRouting".into(),
                },
                register_interface::ResultField {
                    name: "tieredCaching".into(),
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
        ArgoResult {
            smart_routing: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("smartRouting").unwrap(),
            ),
            tiered_caching: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tieredCaching").unwrap(),
            ),
            zone_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("zoneId").unwrap(),
            ),
        }
    }
}
