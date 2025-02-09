#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_router_nat {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetRouterNatArgs {
        /// Name of the NAT service. The name must be 1-63 characters long and
        /// comply with RFC1035.
        #[builder(into)]
        pub name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Region where the router and NAT reside.
        #[builder(into, default)]
        pub region: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of the Cloud Router in which this NAT will be configured.
        ///
        /// - - -
        #[builder(into)]
        pub router: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetRouterNatResult {
        pub auto_network_tier: pulumi_gestalt_rust::Output<String>,
        pub drain_nat_ips: pulumi_gestalt_rust::Output<Vec<String>>,
        pub enable_dynamic_port_allocation: pulumi_gestalt_rust::Output<bool>,
        pub enable_endpoint_independent_mapping: pulumi_gestalt_rust::Output<bool>,
        pub endpoint_types: pulumi_gestalt_rust::Output<Vec<String>>,
        pub icmp_idle_timeout_sec: pulumi_gestalt_rust::Output<i32>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        pub initial_nat_ips: pulumi_gestalt_rust::Output<Vec<String>>,
        pub log_configs: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::compute::GetRouterNatLogConfig>,
        >,
        pub max_ports_per_vm: pulumi_gestalt_rust::Output<i32>,
        pub min_ports_per_vm: pulumi_gestalt_rust::Output<i32>,
        pub name: pulumi_gestalt_rust::Output<String>,
        pub nat_ip_allocate_option: pulumi_gestalt_rust::Output<String>,
        pub nat_ips: pulumi_gestalt_rust::Output<Vec<String>>,
        pub project: pulumi_gestalt_rust::Output<Option<String>>,
        pub region: pulumi_gestalt_rust::Output<Option<String>>,
        pub router: pulumi_gestalt_rust::Output<String>,
        pub rules: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::compute::GetRouterNatRule>,
        >,
        pub source_subnetwork_ip_ranges_to_nat: pulumi_gestalt_rust::Output<String>,
        pub subnetworks: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::compute::GetRouterNatSubnetwork>,
        >,
        pub tcp_established_idle_timeout_sec: pulumi_gestalt_rust::Output<i32>,
        pub tcp_time_wait_timeout_sec: pulumi_gestalt_rust::Output<i32>,
        pub tcp_transitory_idle_timeout_sec: pulumi_gestalt_rust::Output<i32>,
        pub type_: pulumi_gestalt_rust::Output<String>,
        pub udp_idle_timeout_sec: pulumi_gestalt_rust::Output<i32>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::PulumiContext,
        args: GetRouterNatArgs,
    ) -> GetRouterNatResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let name_binding_1 = args.name.get_output(context);
        let name_binding = name_binding_1.get_inner();
        let project_binding_1 = args.project.get_output(context);
        let project_binding = project_binding_1.get_inner();
        let region_binding_1 = args.region.get_output(context);
        let region_binding = region_binding_1.get_inner();
        let router_binding_1 = args.router.get_output(context);
        let router_binding = router_binding_1.get_inner();
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
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetRouterNatResult {
            auto_network_tier: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("autoNetworkTier"),
            ),
            drain_nat_ips: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("drainNatIps"),
            ),
            enable_dynamic_port_allocation: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("enableDynamicPortAllocation"),
            ),
            enable_endpoint_independent_mapping: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("enableEndpointIndependentMapping"),
            ),
            endpoint_types: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("endpointTypes"),
            ),
            icmp_idle_timeout_sec: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("icmpIdleTimeoutSec"),
            ),
            id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("id")),
            initial_nat_ips: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("initialNatIps"),
            ),
            log_configs: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("logConfigs"),
            ),
            max_ports_per_vm: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("maxPortsPerVm"),
            ),
            min_ports_per_vm: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("minPortsPerVm"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            nat_ip_allocate_option: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("natIpAllocateOption"),
            ),
            nat_ips: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("natIps"),
            ),
            project: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("project"),
            ),
            region: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("region"),
            ),
            router: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("router"),
            ),
            rules: pulumi_gestalt_rust::__private::into_domain(o.extract_field("rules")),
            source_subnetwork_ip_ranges_to_nat: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("sourceSubnetworkIpRangesToNat"),
            ),
            subnetworks: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("subnetworks"),
            ),
            tcp_established_idle_timeout_sec: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("tcpEstablishedIdleTimeoutSec"),
            ),
            tcp_time_wait_timeout_sec: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("tcpTimeWaitTimeoutSec"),
            ),
            tcp_transitory_idle_timeout_sec: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("tcpTransitoryIdleTimeoutSec"),
            ),
            type_: pulumi_gestalt_rust::__private::into_domain(o.extract_field("type")),
            udp_idle_timeout_sec: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("udpIdleTimeoutSec"),
            ),
        }
    }
}
