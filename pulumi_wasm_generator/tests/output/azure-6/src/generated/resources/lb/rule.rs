/// Manages a Load Balancer Rule.
///
/// > **NOTE** When using this resource, the Load Balancer needs to have a FrontEnd IP Configuration Attached
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = resource_group::create(
///         "example",
///         ResourceGroupArgs::builder()
///             .location("West Europe")
///             .name("LoadBalancerRG")
///             .build_struct(),
///     );
///     let exampleLoadBalancer = load_balancer::create(
///         "exampleLoadBalancer",
///         LoadBalancerArgs::builder()
///             .frontend_ip_configurations(
///                 vec![
///                     LoadBalancerFrontendIpConfiguration::builder()
///                     .name("PublicIPAddress").publicIpAddressId("${examplePublicIp.id}")
///                     .build_struct(),
///                 ],
///             )
///             .location("West US")
///             .name("TestLoadBalancer")
///             .resource_group_name("${example.name}")
///             .build_struct(),
///     );
///     let examplePublicIp = public_ip::create(
///         "examplePublicIp",
///         PublicIpArgs::builder()
///             .allocation_method("Static")
///             .location("West US")
///             .name("PublicIPForLB")
///             .resource_group_name("${example.name}")
///             .build_struct(),
///     );
///     let exampleRule = rule::create(
///         "exampleRule",
///         RuleArgs::builder()
///             .backend_port(3389)
///             .frontend_ip_configuration_name("PublicIPAddress")
///             .frontend_port(3389)
///             .loadbalancer_id("${exampleLoadBalancer.id}")
///             .name("LBRule")
///             .protocol("Tcp")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Load Balancer Rules can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:lb/rule:Rule example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.Network/loadBalancers/lb1/loadBalancingRules/rule1
/// ```
///
pub mod rule {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct RuleArgs {
        /// A list of reference to a Backend Address Pool over which this Load Balancing Rule operates.
        ///
        /// > **NOTE:** In most cases users can only set one Backend Address Pool ID in the `backend_address_pool_ids`. Especially, when the sku of the LB is `Gateway`, users can set up to two IDs in the `backend_address_pool_ids`.
        #[builder(into, default)]
        pub backend_address_pool_ids: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// The port used for internal connections on the endpoint. Possible values range between 0 and 65535, inclusive. A port of `0` means "Any Port".
        #[builder(into)]
        pub backend_port: pulumi_wasm_rust::Output<i32>,
        /// Is snat enabled for this Load Balancer Rule? Default `false`.
        #[builder(into, default)]
        pub disable_outbound_snat: pulumi_wasm_rust::Output<Option<bool>>,
        /// Are the Floating IPs enabled for this Load Balancer Rule? A "floating” IP is reassigned to a secondary server in case the primary server fails. Required to configure a SQL AlwaysOn Availability Group. Defaults to `false`.
        #[builder(into, default)]
        pub enable_floating_ip: pulumi_wasm_rust::Output<Option<bool>>,
        /// Is TCP Reset enabled for this Load Balancer Rule?
        #[builder(into, default)]
        pub enable_tcp_reset: pulumi_wasm_rust::Output<Option<bool>>,
        /// The name of the frontend IP configuration to which the rule is associated.
        #[builder(into)]
        pub frontend_ip_configuration_name: pulumi_wasm_rust::Output<String>,
        /// The port for the external endpoint. Port numbers for each Rule must be unique within the Load Balancer. Possible values range between 0 and 65534, inclusive. A port of `0` means "Any Port".
        #[builder(into)]
        pub frontend_port: pulumi_wasm_rust::Output<i32>,
        /// Specifies the idle timeout in minutes for TCP connections. Valid values are between `4` and `100` minutes. Defaults to `4` minutes.
        #[builder(into, default)]
        pub idle_timeout_in_minutes: pulumi_wasm_rust::Output<Option<i32>>,
        /// Specifies the load balancing distribution type to be used by the Load Balancer. Possible values are: `Default` – The load balancer is configured to use a 5 tuple hash to map traffic to available servers. `SourceIP` – The load balancer is configured to use a 2 tuple hash to map traffic to available servers. `SourceIPProtocol` – The load balancer is configured to use a 3 tuple hash to map traffic to available servers. Also known as Session Persistence, where in the Azure portal the options are called `None`, `Client IP` and `Client IP and Protocol` respectively. Defaults to `Default`.
        #[builder(into, default)]
        pub load_distribution: pulumi_wasm_rust::Output<Option<String>>,
        /// The ID of the Load Balancer in which to create the Rule. Changing this forces a new resource to be created.
        #[builder(into)]
        pub loadbalancer_id: pulumi_wasm_rust::Output<String>,
        /// Specifies the name of the LB Rule. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// A reference to a Probe used by this Load Balancing Rule.
        #[builder(into, default)]
        pub probe_id: pulumi_wasm_rust::Output<Option<String>>,
        /// The transport protocol for the external endpoint. Possible values are `Tcp`, `Udp` or `All`.
        #[builder(into)]
        pub protocol: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct RuleResult {
        /// A list of reference to a Backend Address Pool over which this Load Balancing Rule operates.
        ///
        /// > **NOTE:** In most cases users can only set one Backend Address Pool ID in the `backend_address_pool_ids`. Especially, when the sku of the LB is `Gateway`, users can set up to two IDs in the `backend_address_pool_ids`.
        pub backend_address_pool_ids: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// The port used for internal connections on the endpoint. Possible values range between 0 and 65535, inclusive. A port of `0` means "Any Port".
        pub backend_port: pulumi_wasm_rust::Output<i32>,
        /// Is snat enabled for this Load Balancer Rule? Default `false`.
        pub disable_outbound_snat: pulumi_wasm_rust::Output<Option<bool>>,
        /// Are the Floating IPs enabled for this Load Balancer Rule? A "floating” IP is reassigned to a secondary server in case the primary server fails. Required to configure a SQL AlwaysOn Availability Group. Defaults to `false`.
        pub enable_floating_ip: pulumi_wasm_rust::Output<Option<bool>>,
        /// Is TCP Reset enabled for this Load Balancer Rule?
        pub enable_tcp_reset: pulumi_wasm_rust::Output<Option<bool>>,
        pub frontend_ip_configuration_id: pulumi_wasm_rust::Output<String>,
        /// The name of the frontend IP configuration to which the rule is associated.
        pub frontend_ip_configuration_name: pulumi_wasm_rust::Output<String>,
        /// The port for the external endpoint. Port numbers for each Rule must be unique within the Load Balancer. Possible values range between 0 and 65534, inclusive. A port of `0` means "Any Port".
        pub frontend_port: pulumi_wasm_rust::Output<i32>,
        /// Specifies the idle timeout in minutes for TCP connections. Valid values are between `4` and `100` minutes. Defaults to `4` minutes.
        pub idle_timeout_in_minutes: pulumi_wasm_rust::Output<Option<i32>>,
        /// Specifies the load balancing distribution type to be used by the Load Balancer. Possible values are: `Default` – The load balancer is configured to use a 5 tuple hash to map traffic to available servers. `SourceIP` – The load balancer is configured to use a 2 tuple hash to map traffic to available servers. `SourceIPProtocol` – The load balancer is configured to use a 3 tuple hash to map traffic to available servers. Also known as Session Persistence, where in the Azure portal the options are called `None`, `Client IP` and `Client IP and Protocol` respectively. Defaults to `Default`.
        pub load_distribution: pulumi_wasm_rust::Output<Option<String>>,
        /// The ID of the Load Balancer in which to create the Rule. Changing this forces a new resource to be created.
        pub loadbalancer_id: pulumi_wasm_rust::Output<String>,
        /// Specifies the name of the LB Rule. Changing this forces a new resource to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// A reference to a Probe used by this Load Balancing Rule.
        pub probe_id: pulumi_wasm_rust::Output<Option<String>>,
        /// The transport protocol for the external endpoint. Possible values are `Tcp`, `Udp` or `All`.
        pub protocol: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: RuleArgs) -> RuleResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let backend_address_pool_ids_binding = args.backend_address_pool_ids.get_inner();
        let backend_port_binding = args.backend_port.get_inner();
        let disable_outbound_snat_binding = args.disable_outbound_snat.get_inner();
        let enable_floating_ip_binding = args.enable_floating_ip.get_inner();
        let enable_tcp_reset_binding = args.enable_tcp_reset.get_inner();
        let frontend_ip_configuration_name_binding = args
            .frontend_ip_configuration_name
            .get_inner();
        let frontend_port_binding = args.frontend_port.get_inner();
        let idle_timeout_in_minutes_binding = args.idle_timeout_in_minutes.get_inner();
        let load_distribution_binding = args.load_distribution.get_inner();
        let loadbalancer_id_binding = args.loadbalancer_id.get_inner();
        let name_binding = args.name.get_inner();
        let probe_id_binding = args.probe_id.get_inner();
        let protocol_binding = args.protocol.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:lb/rule:Rule".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "backendAddressPoolIds".into(),
                    value: &backend_address_pool_ids_binding,
                },
                register_interface::ObjectField {
                    name: "backendPort".into(),
                    value: &backend_port_binding,
                },
                register_interface::ObjectField {
                    name: "disableOutboundSnat".into(),
                    value: &disable_outbound_snat_binding,
                },
                register_interface::ObjectField {
                    name: "enableFloatingIp".into(),
                    value: &enable_floating_ip_binding,
                },
                register_interface::ObjectField {
                    name: "enableTcpReset".into(),
                    value: &enable_tcp_reset_binding,
                },
                register_interface::ObjectField {
                    name: "frontendIpConfigurationName".into(),
                    value: &frontend_ip_configuration_name_binding,
                },
                register_interface::ObjectField {
                    name: "frontendPort".into(),
                    value: &frontend_port_binding,
                },
                register_interface::ObjectField {
                    name: "idleTimeoutInMinutes".into(),
                    value: &idle_timeout_in_minutes_binding,
                },
                register_interface::ObjectField {
                    name: "loadDistribution".into(),
                    value: &load_distribution_binding,
                },
                register_interface::ObjectField {
                    name: "loadbalancerId".into(),
                    value: &loadbalancer_id_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "probeId".into(),
                    value: &probe_id_binding,
                },
                register_interface::ObjectField {
                    name: "protocol".into(),
                    value: &protocol_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "backendAddressPoolIds".into(),
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
                    name: "frontendIpConfigurationId".into(),
                },
                register_interface::ResultField {
                    name: "frontendIpConfigurationName".into(),
                },
                register_interface::ResultField {
                    name: "frontendPort".into(),
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
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        RuleResult {
            backend_address_pool_ids: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("backendAddressPoolIds").unwrap(),
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
            frontend_ip_configuration_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("frontendIpConfigurationId").unwrap(),
            ),
            frontend_ip_configuration_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("frontendIpConfigurationName").unwrap(),
            ),
            frontend_port: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("frontendPort").unwrap(),
            ),
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
