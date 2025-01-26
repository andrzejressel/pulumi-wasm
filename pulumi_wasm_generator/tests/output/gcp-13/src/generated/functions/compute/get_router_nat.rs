pub mod get_router_nat {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetRouterNatArgs {
        /// Name of the NAT service. The name must be 1-63 characters long and
        /// comply with RFC1035.
        #[builder(into)]
        pub name: pulumi_wasm_rust::InputOrOutput<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Region where the router and NAT reside.
        #[builder(into, default)]
        pub region: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The name of the Cloud Router in which this NAT will be configured.
        ///
        /// - - -
        #[builder(into)]
        pub router: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetRouterNatResult {
        pub auto_network_tier: pulumi_wasm_rust::Output<String>,
        pub drain_nat_ips: pulumi_wasm_rust::Output<Vec<String>>,
        pub enable_dynamic_port_allocation: pulumi_wasm_rust::Output<bool>,
        pub enable_endpoint_independent_mapping: pulumi_wasm_rust::Output<bool>,
        pub endpoint_types: pulumi_wasm_rust::Output<Vec<String>>,
        pub icmp_idle_timeout_sec: pulumi_wasm_rust::Output<i32>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        pub initial_nat_ips: pulumi_wasm_rust::Output<Vec<String>>,
        pub log_configs: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::compute::GetRouterNatLogConfig>,
        >,
        pub max_ports_per_vm: pulumi_wasm_rust::Output<i32>,
        pub min_ports_per_vm: pulumi_wasm_rust::Output<i32>,
        pub name: pulumi_wasm_rust::Output<String>,
        pub nat_ip_allocate_option: pulumi_wasm_rust::Output<String>,
        pub nat_ips: pulumi_wasm_rust::Output<Vec<String>>,
        pub project: pulumi_wasm_rust::Output<Option<String>>,
        pub region: pulumi_wasm_rust::Output<Option<String>>,
        pub router: pulumi_wasm_rust::Output<String>,
        pub rules: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::compute::GetRouterNatRule>,
        >,
        pub source_subnetwork_ip_ranges_to_nat: pulumi_wasm_rust::Output<String>,
        pub subnetworks: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::compute::GetRouterNatSubnetwork>,
        >,
        pub tcp_established_idle_timeout_sec: pulumi_wasm_rust::Output<i32>,
        pub tcp_time_wait_timeout_sec: pulumi_wasm_rust::Output<i32>,
        pub tcp_transitory_idle_timeout_sec: pulumi_wasm_rust::Output<i32>,
        pub type_: pulumi_wasm_rust::Output<String>,
        pub udp_idle_timeout_sec: pulumi_wasm_rust::Output<i32>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetRouterNatArgs,
    ) -> GetRouterNatResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let name_binding = args.name.get_output(context).get_inner();
        let project_binding = args.project.get_output(context).get_inner();
        let region_binding = args.region.get_output(context).get_inner();
        let router_binding = args.router.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "gcp:compute/getRouterNat:getRouterNat".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
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
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "autoNetworkTier".into(),
                },
                register_interface::ResultField {
                    name: "drainNatIps".into(),
                },
                register_interface::ResultField {
                    name: "enableDynamicPortAllocation".into(),
                },
                register_interface::ResultField {
                    name: "enableEndpointIndependentMapping".into(),
                },
                register_interface::ResultField {
                    name: "endpointTypes".into(),
                },
                register_interface::ResultField {
                    name: "icmpIdleTimeoutSec".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "initialNatIps".into(),
                },
                register_interface::ResultField {
                    name: "logConfigs".into(),
                },
                register_interface::ResultField {
                    name: "maxPortsPerVm".into(),
                },
                register_interface::ResultField {
                    name: "minPortsPerVm".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "natIpAllocateOption".into(),
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
                    name: "rules".into(),
                },
                register_interface::ResultField {
                    name: "sourceSubnetworkIpRangesToNat".into(),
                },
                register_interface::ResultField {
                    name: "subnetworks".into(),
                },
                register_interface::ResultField {
                    name: "tcpEstablishedIdleTimeoutSec".into(),
                },
                register_interface::ResultField {
                    name: "tcpTimeWaitTimeoutSec".into(),
                },
                register_interface::ResultField {
                    name: "tcpTransitoryIdleTimeoutSec".into(),
                },
                register_interface::ResultField {
                    name: "type".into(),
                },
                register_interface::ResultField {
                    name: "udpIdleTimeoutSec".into(),
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetRouterNatResult {
            auto_network_tier: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("autoNetworkTier").unwrap(),
            ),
            drain_nat_ips: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("drainNatIps").unwrap(),
            ),
            enable_dynamic_port_allocation: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("enableDynamicPortAllocation").unwrap(),
            ),
            enable_endpoint_independent_mapping: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("enableEndpointIndependentMapping").unwrap(),
            ),
            endpoint_types: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("endpointTypes").unwrap(),
            ),
            icmp_idle_timeout_sec: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("icmpIdleTimeoutSec").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            initial_nat_ips: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("initialNatIps").unwrap(),
            ),
            log_configs: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("logConfigs").unwrap(),
            ),
            max_ports_per_vm: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("maxPortsPerVm").unwrap(),
            ),
            min_ports_per_vm: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("minPortsPerVm").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            nat_ip_allocate_option: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("natIpAllocateOption").unwrap(),
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
            rules: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("rules").unwrap(),
            ),
            source_subnetwork_ip_ranges_to_nat: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("sourceSubnetworkIpRangesToNat").unwrap(),
            ),
            subnetworks: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("subnetworks").unwrap(),
            ),
            tcp_established_idle_timeout_sec: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tcpEstablishedIdleTimeoutSec").unwrap(),
            ),
            tcp_time_wait_timeout_sec: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tcpTimeWaitTimeoutSec").unwrap(),
            ),
            tcp_transitory_idle_timeout_sec: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tcpTransitoryIdleTimeoutSec").unwrap(),
            ),
            type_: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("type").unwrap(),
            ),
            udp_idle_timeout_sec: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("udpIdleTimeoutSec").unwrap(),
            ),
        }
    }
}
