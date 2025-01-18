pub mod get_lb_outbound_rule {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetLbOutboundRuleArgs {
        /// The ID of the Load Balancer in which the Outbound Rule exists.
        #[builder(into)]
        pub loadbalancer_id: pulumi_wasm_rust::Output<String>,
        /// The name of this Load Balancer Outbound Rule.
        #[builder(into)]
        pub name: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct GetLbOutboundRuleResult {
        /// The number of outbound ports used for NAT.
        pub allocated_outbound_ports: pulumi_wasm_rust::Output<i32>,
        /// The ID of the Backend Address Pool. Outbound traffic is randomly load balanced across IPs in the backend IPs.
        pub backend_address_pool_id: pulumi_wasm_rust::Output<String>,
        /// A `frontend_ip_configuration` block as defined below.
        pub frontend_ip_configurations: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::lb::GetLbOutboundRuleFrontendIpConfiguration>,
        >,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        /// The timeout for the TCP idle connection.
        pub idle_timeout_in_minutes: pulumi_wasm_rust::Output<i32>,
        pub loadbalancer_id: pulumi_wasm_rust::Output<String>,
        /// The name of the Frontend IP Configuration.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The transport protocol for the external endpoint.
        pub protocol: pulumi_wasm_rust::Output<String>,
        /// Is the bidirectional TCP Reset on TCP flow idle timeout or unexpected connection termination enabled? This value is useful when the protocol is set to TCP.
        pub tcp_reset_enabled: pulumi_wasm_rust::Output<bool>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(args: GetLbOutboundRuleArgs) -> GetLbOutboundRuleResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let loadbalancer_id_binding = args.loadbalancer_id.get_inner();
        let name_binding = args.name.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "azure:lb/getLBOutboundRule:getLBOutboundRule".into(),
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
                    name: "allocatedOutboundPorts".into(),
                },
                register_interface::ResultField {
                    name: "backendAddressPoolId".into(),
                },
                register_interface::ResultField {
                    name: "frontendIpConfigurations".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "idleTimeoutInMinutes".into(),
                },
                register_interface::ResultField {
                    name: "loadbalancerId".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "protocol".into(),
                },
                register_interface::ResultField {
                    name: "tcpResetEnabled".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetLbOutboundRuleResult {
            allocated_outbound_ports: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("allocatedOutboundPorts").unwrap(),
            ),
            backend_address_pool_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("backendAddressPoolId").unwrap(),
            ),
            frontend_ip_configurations: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("frontendIpConfigurations").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            idle_timeout_in_minutes: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("idleTimeoutInMinutes").unwrap(),
            ),
            loadbalancer_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("loadbalancerId").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            protocol: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("protocol").unwrap(),
            ),
            tcp_reset_enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tcpResetEnabled").unwrap(),
            ),
        }
    }
}
