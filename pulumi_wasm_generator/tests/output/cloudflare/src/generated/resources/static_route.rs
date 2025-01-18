/// Provides a resource, that manages Cloudflare static routes for Magic
/// Transit or Magic WAN. Static routes are used to route traffic
/// through GRE tunnels.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = static_route::create(
///         "example",
///         StaticRouteArgs::builder()
///             .account_id("f037e56e89293a057740de681ac9abbe")
///             .colo_names(vec!["den01",])
///             .colo_regions(vec!["APAC",])
///             .description("New route for new prefix 192.0.2.0/24")
///             .nexthop("10.0.0.0")
///             .prefix("192.0.2.0/24")
///             .priority(100)
///             .weight(10)
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// ```sh
/// $ pulumi import cloudflare:index/staticRoute:StaticRoute example <account_id>/<static_route_id>
/// ```
///
pub mod static_route {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct StaticRouteArgs {
        /// The account identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
        #[builder(into, default)]
        pub account_id: pulumi_wasm_rust::Output<Option<String>>,
        /// List of Cloudflare colocation regions for this static route.
        #[builder(into, default)]
        pub colo_names: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// List of Cloudflare colocation names for this static route.
        #[builder(into, default)]
        pub colo_regions: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// Description of the static route.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// The nexthop IP address where traffic will be routed to.
        #[builder(into)]
        pub nexthop: pulumi_wasm_rust::Output<String>,
        /// Your network prefix using CIDR notation.
        #[builder(into)]
        pub prefix: pulumi_wasm_rust::Output<String>,
        /// The priority for the static route.
        #[builder(into)]
        pub priority: pulumi_wasm_rust::Output<i32>,
        /// The optional weight for ECMP routes. **Modifying this attribute will force creation of a new resource.**
        #[builder(into, default)]
        pub weight: pulumi_wasm_rust::Output<Option<i32>>,
    }
    #[allow(dead_code)]
    pub struct StaticRouteResult {
        /// The account identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
        pub account_id: pulumi_wasm_rust::Output<Option<String>>,
        /// List of Cloudflare colocation regions for this static route.
        pub colo_names: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// List of Cloudflare colocation names for this static route.
        pub colo_regions: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// Description of the static route.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// The nexthop IP address where traffic will be routed to.
        pub nexthop: pulumi_wasm_rust::Output<String>,
        /// Your network prefix using CIDR notation.
        pub prefix: pulumi_wasm_rust::Output<String>,
        /// The priority for the static route.
        pub priority: pulumi_wasm_rust::Output<i32>,
        /// The optional weight for ECMP routes. **Modifying this attribute will force creation of a new resource.**
        pub weight: pulumi_wasm_rust::Output<Option<i32>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: StaticRouteArgs) -> StaticRouteResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let account_id_binding = args.account_id.get_inner();
        let colo_names_binding = args.colo_names.get_inner();
        let colo_regions_binding = args.colo_regions.get_inner();
        let description_binding = args.description.get_inner();
        let nexthop_binding = args.nexthop.get_inner();
        let prefix_binding = args.prefix.get_inner();
        let priority_binding = args.priority.get_inner();
        let weight_binding = args.weight.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "cloudflare:index/staticRoute:StaticRoute".into(),
            name: name.to_string(),
            version: super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "accountId".into(),
                    value: &account_id_binding,
                },
                register_interface::ObjectField {
                    name: "coloNames".into(),
                    value: &colo_names_binding,
                },
                register_interface::ObjectField {
                    name: "coloRegions".into(),
                    value: &colo_regions_binding,
                },
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "nexthop".into(),
                    value: &nexthop_binding,
                },
                register_interface::ObjectField {
                    name: "prefix".into(),
                    value: &prefix_binding,
                },
                register_interface::ObjectField {
                    name: "priority".into(),
                    value: &priority_binding,
                },
                register_interface::ObjectField {
                    name: "weight".into(),
                    value: &weight_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "accountId".into(),
                },
                register_interface::ResultField {
                    name: "coloNames".into(),
                },
                register_interface::ResultField {
                    name: "coloRegions".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "nexthop".into(),
                },
                register_interface::ResultField {
                    name: "prefix".into(),
                },
                register_interface::ResultField {
                    name: "priority".into(),
                },
                register_interface::ResultField {
                    name: "weight".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        StaticRouteResult {
            account_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("accountId").unwrap(),
            ),
            colo_names: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("coloNames").unwrap(),
            ),
            colo_regions: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("coloRegions").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            nexthop: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("nexthop").unwrap(),
            ),
            prefix: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("prefix").unwrap(),
            ),
            priority: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("priority").unwrap(),
            ),
            weight: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("weight").unwrap(),
            ),
        }
    }
}
