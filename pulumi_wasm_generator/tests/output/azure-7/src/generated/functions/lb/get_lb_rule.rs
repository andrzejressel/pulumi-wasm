pub mod get_lb_rule {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetLbRuleArgs {
        /// The ID of the Load Balancer Rule.
        #[builder(into)]
        pub loadbalancer_id: pulumi_wasm_rust::Output<String>,
        /// The name of this Load Balancer Rule.
        #[builder(into)]
        pub name: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct GetLbRuleResult {
        /// A reference to a Backend Address Pool over which this Load Balancing Rule operates.
        pub backend_address_pool_id: pulumi_wasm_rust::Output<String>,
        /// The port used for internal connections on the endpoint.
        pub backend_port: pulumi_wasm_rust::Output<i32>,
        /// If outbound SNAT is enabled for this Load Balancer Rule.
        pub disable_outbound_snat: pulumi_wasm_rust::Output<bool>,
        /// If Floating IPs are enabled for this Load Balancer Rule
        pub enable_floating_ip: pulumi_wasm_rust::Output<bool>,
        /// If TCP Reset is enabled for this Load Balancer Rule.
        pub enable_tcp_reset: pulumi_wasm_rust::Output<bool>,
        /// The name of the frontend IP configuration to which the rule is associated.
        pub frontend_ip_configuration_name: pulumi_wasm_rust::Output<String>,
        /// The port for the external endpoint.
        pub frontend_port: pulumi_wasm_rust::Output<i32>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        /// Specifies the idle timeout in minutes for TCP connections.
        pub idle_timeout_in_minutes: pulumi_wasm_rust::Output<i32>,
        /// Specifies the load balancing distribution type used by the Load Balancer.
        pub load_distribution: pulumi_wasm_rust::Output<String>,
        pub loadbalancer_id: pulumi_wasm_rust::Output<String>,
        pub name: pulumi_wasm_rust::Output<String>,
        /// A reference to a Probe used by this Load Balancing Rule.
        pub probe_id: pulumi_wasm_rust::Output<String>,
        /// The transport protocol for the external endpoint.
        pub protocol: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(args: GetLbRuleArgs) -> GetLbRuleResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let loadbalancer_id_binding = args.loadbalancer_id.get_inner();
        let name_binding = args.name.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "azure:lb/getLBRule:getLBRule".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "loadbalancerId".into(),
                    value: &loadbalancer_id_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "backendAddressPoolId".into(),
                },
                register_interface::ResultField {
                    name: "backendPort".into(),
                },
                register_interface::ResultField {
                    name: "disableOutboundSnat".into(),
                },
                register_interface::ResultField {
                    name: "enableFloatingIp".into(),
                },
                register_interface::ResultField {
                    name: "enableTcpReset".into(),
                },
                register_interface::ResultField {
                    name: "frontendIpConfigurationName".into(),
                },
                register_interface::ResultField {
                    name: "frontendPort".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "idleTimeoutInMinutes".into(),
                },
                register_interface::ResultField {
                    name: "loadDistribution".into(),
                },
                register_interface::ResultField {
                    name: "loadbalancerId".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "probeId".into(),
                },
                register_interface::ResultField {
                    name: "protocol".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetLbRuleResult {
            backend_address_pool_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("backendAddressPoolId").unwrap(),
            ),
            backend_port: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("backendPort").unwrap(),
            ),
            disable_outbound_snat: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("disableOutboundSnat").unwrap(),
            ),
            enable_floating_ip: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("enableFloatingIp").unwrap(),
            ),
            enable_tcp_reset: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("enableTcpReset").unwrap(),
            ),
            frontend_ip_configuration_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("frontendIpConfigurationName").unwrap(),
            ),
            frontend_port: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("frontendPort").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            idle_timeout_in_minutes: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("idleTimeoutInMinutes").unwrap(),
            ),
            load_distribution: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("loadDistribution").unwrap(),
            ),
            loadbalancer_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("loadbalancerId").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            probe_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("probeId").unwrap(),
            ),
            protocol: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("protocol").unwrap(),
            ),
        }
    }
}
