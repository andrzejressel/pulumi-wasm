/// Provides a proxy protocol policy, which allows an ELB to carry a client connection information to a backend.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let lb = load_balancer::create(
///         "lb",
///         LoadBalancerArgs::builder()
///             .availability_zones(vec!["us-east-1a",])
///             .listeners(
///                 vec![
///                     LoadBalancerListener::builder().instancePort(25)
///                     .instanceProtocol("tcp").lbPort(25).lbProtocol("tcp").build_struct(),
///                     LoadBalancerListener::builder().instancePort(587)
///                     .instanceProtocol("tcp").lbPort(587).lbProtocol("tcp")
///                     .build_struct(),
///                 ],
///             )
///             .name("test-lb")
///             .build_struct(),
///     );
///     let smtp = proxy_protocol_policy::create(
///         "smtp",
///         ProxyProtocolPolicyArgs::builder()
///             .instance_ports(vec!["25", "587",])
///             .load_balancer("${lb.name}")
///             .build_struct(),
///     );
/// }
/// ```
pub mod proxy_protocol_policy {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ProxyProtocolPolicyArgs {
        /// List of instance ports to which the policy
        /// should be applied. This can be specified if the protocol is SSL or TCP.
        #[builder(into)]
        pub instance_ports: pulumi_wasm_rust::Output<Vec<String>>,
        /// The load balancer to which the policy
        /// should be attached.
        #[builder(into)]
        pub load_balancer: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct ProxyProtocolPolicyResult {
        /// List of instance ports to which the policy
        /// should be applied. This can be specified if the protocol is SSL or TCP.
        pub instance_ports: pulumi_wasm_rust::Output<Vec<String>>,
        /// The load balancer to which the policy
        /// should be attached.
        pub load_balancer: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: ProxyProtocolPolicyArgs,
    ) -> ProxyProtocolPolicyResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let instance_ports_binding = args.instance_ports.get_inner();
        let load_balancer_binding = args.load_balancer.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:ec2/proxyProtocolPolicy:ProxyProtocolPolicy".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "instancePorts".into(),
                    value: &instance_ports_binding,
                },
                register_interface::ObjectField {
                    name: "loadBalancer".into(),
                    value: &load_balancer_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "instancePorts".into(),
                },
                register_interface::ResultField {
                    name: "loadBalancer".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        ProxyProtocolPolicyResult {
            instance_ports: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("instancePorts").unwrap(),
            ),
            load_balancer: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("loadBalancer").unwrap(),
            ),
        }
    }
}
