/// Provides a Cloudflare worker route resource. A route will also require a `cloudflare.WorkerScript`.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let myRoute = worker_route::create(
///         "myRoute",
///         WorkerRouteArgs::builder()
///             .pattern("example.com/*")
///             .script_name("${myScript.name}")
///             .zone_id("0da42c8d2132a9ddaf714f9e7c920711")
///             .build_struct(),
///     );
///     let myScript = workers_script::create(
///         "myScript",
///         WorkersScriptArgs::builder().build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// ```sh
/// $ pulumi import cloudflare:index/workerRoute:WorkerRoute example <zone_id>/<route_id>
/// ```
///
pub mod worker_route {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct WorkerRouteArgs {
        /// The [route pattern](https://developers.cloudflare.com/workers/about/routes/) to associate the Worker with.
        #[builder(into)]
        pub pattern: pulumi_wasm_rust::Output<String>,
        /// Worker script name to invoke for requests that match the route pattern.
        #[builder(into, default)]
        pub script_name: pulumi_wasm_rust::Output<Option<String>>,
        /// The zone identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
        #[builder(into)]
        pub zone_id: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct WorkerRouteResult {
        /// The [route pattern](https://developers.cloudflare.com/workers/about/routes/) to associate the Worker with.
        pub pattern: pulumi_wasm_rust::Output<String>,
        /// Worker script name to invoke for requests that match the route pattern.
        pub script_name: pulumi_wasm_rust::Output<Option<String>>,
        /// The zone identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
        pub zone_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: WorkerRouteArgs) -> WorkerRouteResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let pattern_binding = args.pattern.get_inner();
        let script_name_binding = args.script_name.get_inner();
        let zone_id_binding = args.zone_id.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "cloudflare:index/workerRoute:WorkerRoute".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "pattern".into(),
                    value: &pattern_binding,
                },
                register_interface::ObjectField {
                    name: "scriptName".into(),
                    value: &script_name_binding,
                },
                register_interface::ObjectField {
                    name: "zoneId".into(),
                    value: &zone_id_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "pattern".into(),
                },
                register_interface::ResultField {
                    name: "scriptName".into(),
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
        WorkerRouteResult {
            pattern: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("pattern").unwrap(),
            ),
            script_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("scriptName").unwrap(),
            ),
            zone_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("zoneId").unwrap(),
            ),
        }
    }
}
