/// A resource used to set the list of IP addresses to be used in a NAT service and manage the draining of destroyed IPs.
///
/// > **Note:** This resource is to be used alongside a `gcp.compute.RouterNat` resource,
/// the router nat resource must have no defined `nat_ips` or `drain_nat_ips` parameters,
/// instead using the `initial_nat_ips` parameter to set at least one IP for the creation of the resource.
///
///
/// To get more information about RouterNatAddress, see:
///
/// * [API documentation](https://cloud.google.com/compute/docs/reference/rest/v1/routers)
/// * How-to Guides
///     * [Google Cloud Router](https://cloud.google.com/router/docs/)
///
/// ## Example Usage
///
/// ## Import
///
/// RouterNatAddress can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/regions/{{region}}/routers/{{router}}/{{router_nat}}`
///
/// * `{{project}}/{{region}}/{{router}}/{{router_nat}}`
///
/// * `{{region}}/{{router}}/{{router_nat}}`
///
/// * `{{router}}/{{router_nat}}`
///
/// When using the `pulumi import` command, RouterNatAddress can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:compute/routerNatAddress:RouterNatAddress default projects/{{project}}/regions/{{region}}/routers/{{router}}/{{router_nat}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:compute/routerNatAddress:RouterNatAddress default {{project}}/{{region}}/{{router}}/{{router_nat}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:compute/routerNatAddress:RouterNatAddress default {{region}}/{{router}}/{{router_nat}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:compute/routerNatAddress:RouterNatAddress default {{router}}/{{router_nat}}
/// ```
///
pub mod router_nat_address {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct RouterNatAddressArgs {
        /// A list of URLs of the IP resources to be drained. These IPs must be
        /// valid static external IPs that have been assigned to the NAT.
        #[builder(into, default)]
        pub drain_nat_ips: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// Self-links of NAT IPs to be used in a Nat service. Only valid if the referenced RouterNat
        /// natIpAllocateOption is set to MANUAL_ONLY.
        #[builder(into)]
        pub nat_ips: pulumi_wasm_rust::Output<Vec<String>>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_wasm_rust::Output<Option<String>>,
        /// Region where the NAT service reside.
        #[builder(into, default)]
        pub region: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of the Cloud Router in which the referenced NAT service is configured.
        #[builder(into)]
        pub router: pulumi_wasm_rust::Output<String>,
        /// The name of the Nat service in which this address will be configured.
        ///
        ///
        /// - - -
        #[builder(into)]
        pub router_nat: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct RouterNatAddressResult {
        /// A list of URLs of the IP resources to be drained. These IPs must be
        /// valid static external IPs that have been assigned to the NAT.
        pub drain_nat_ips: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// Self-links of NAT IPs to be used in a Nat service. Only valid if the referenced RouterNat
        /// natIpAllocateOption is set to MANUAL_ONLY.
        pub nat_ips: pulumi_wasm_rust::Output<Vec<String>>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_wasm_rust::Output<String>,
        /// Region where the NAT service reside.
        pub region: pulumi_wasm_rust::Output<String>,
        /// The name of the Cloud Router in which the referenced NAT service is configured.
        pub router: pulumi_wasm_rust::Output<String>,
        /// The name of the Nat service in which this address will be configured.
        ///
        ///
        /// - - -
        pub router_nat: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: RouterNatAddressArgs) -> RouterNatAddressResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let drain_nat_ips_binding = args.drain_nat_ips.get_inner();
        let nat_ips_binding = args.nat_ips.get_inner();
        let project_binding = args.project.get_inner();
        let region_binding = args.region.get_inner();
        let router_binding = args.router.get_inner();
        let router_nat_binding = args.router_nat.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:compute/routerNatAddress:RouterNatAddress".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "drainNatIps".into(),
                    value: &drain_nat_ips_binding,
                },
                register_interface::ObjectField {
                    name: "natIps".into(),
                    value: &nat_ips_binding,
                },
                register_interface::ObjectField {
                    name: "project".into(),
                    value: &project_binding,
                },
                register_interface::ObjectField {
                    name: "region".into(),
                    value: &region_binding,
                },
                register_interface::ObjectField {
                    name: "router".into(),
                    value: &router_binding,
                },
                register_interface::ObjectField {
                    name: "routerNat".into(),
                    value: &router_nat_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "drainNatIps".into(),
                },
                register_interface::ResultField {
                    name: "natIps".into(),
                },
                register_interface::ResultField {
                    name: "project".into(),
                },
                register_interface::ResultField {
                    name: "region".into(),
                },
                register_interface::ResultField {
                    name: "router".into(),
                },
                register_interface::ResultField {
                    name: "routerNat".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        RouterNatAddressResult {
            drain_nat_ips: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("drainNatIps").unwrap(),
            ),
            nat_ips: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("natIps").unwrap(),
            ),
            project: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("project").unwrap(),
            ),
            region: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("region").unwrap(),
            ),
            router: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("router").unwrap(),
            ),
            router_nat: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("routerNat").unwrap(),
            ),
        }
    }
}
