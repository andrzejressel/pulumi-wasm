/// Manages a LoadBalancer Probe Resource.
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
///             .location("${example.location}")
///             .name("TestLoadBalancer")
///             .resource_group_name("${example.name}")
///             .build_struct(),
///     );
///     let exampleProbe = probe::create(
///         "exampleProbe",
///         ProbeArgs::builder()
///             .loadbalancer_id("${exampleLoadBalancer.id}")
///             .name("ssh-running-probe")
///             .port(22)
///             .build_struct(),
///     );
///     let examplePublicIp = public_ip::create(
///         "examplePublicIp",
///         PublicIpArgs::builder()
///             .allocation_method("Static")
///             .location("${example.location}")
///             .name("PublicIPForLB")
///             .resource_group_name("${example.name}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Load Balancer Probes can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:lb/probe:Probe example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.Network/loadBalancers/lb1/probes/probe1
/// ```
///
pub mod probe {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ProbeArgs {
        /// The interval, in seconds between probes to the backend endpoint for health status. The default value is 15, the minimum value is 5.
        #[builder(into, default)]
        pub interval_in_seconds: pulumi_wasm_rust::Output<Option<i32>>,
        /// The ID of the LoadBalancer in which to create the Probe. Changing this forces a new resource to be created.
        #[builder(into)]
        pub loadbalancer_id: pulumi_wasm_rust::Output<String>,
        /// Specifies the name of the Probe. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// The number of failed probe attempts after which the backend endpoint is removed from rotation. Default to `2`. NumberOfProbes multiplied by intervalInSeconds value must be greater or equal to 10.Endpoints are returned to rotation when at least one probe is successful.
        #[builder(into, default)]
        pub number_of_probes: pulumi_wasm_rust::Output<Option<i32>>,
        /// Port on which the Probe queries the backend endpoint. Possible values range from 1 to 65535, inclusive.
        #[builder(into)]
        pub port: pulumi_wasm_rust::Output<i32>,
        /// The number of consecutive successful or failed probes that allow or deny traffic to this endpoint. Possible values range from `1` to `100`. The default value is `1`.
        #[builder(into, default)]
        pub probe_threshold: pulumi_wasm_rust::Output<Option<i32>>,
        /// Specifies the protocol of the end point. Possible values are `Http`, `Https` or `Tcp`. If TCP is specified, a received ACK is required for the probe to be successful. If HTTP is specified, a 200 OK response from the specified URI is required for the probe to be successful. Defaults to `Tcp`.
        #[builder(into, default)]
        pub protocol: pulumi_wasm_rust::Output<Option<String>>,
        /// The URI used for requesting health status from the backend endpoint. Required if protocol is set to `Http` or `Https`. Otherwise, it is not allowed.
        #[builder(into, default)]
        pub request_path: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct ProbeResult {
        /// The interval, in seconds between probes to the backend endpoint for health status. The default value is 15, the minimum value is 5.
        pub interval_in_seconds: pulumi_wasm_rust::Output<Option<i32>>,
        pub load_balancer_rules: pulumi_wasm_rust::Output<Vec<String>>,
        /// The ID of the LoadBalancer in which to create the Probe. Changing this forces a new resource to be created.
        pub loadbalancer_id: pulumi_wasm_rust::Output<String>,
        /// Specifies the name of the Probe. Changing this forces a new resource to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The number of failed probe attempts after which the backend endpoint is removed from rotation. Default to `2`. NumberOfProbes multiplied by intervalInSeconds value must be greater or equal to 10.Endpoints are returned to rotation when at least one probe is successful.
        pub number_of_probes: pulumi_wasm_rust::Output<Option<i32>>,
        /// Port on which the Probe queries the backend endpoint. Possible values range from 1 to 65535, inclusive.
        pub port: pulumi_wasm_rust::Output<i32>,
        /// The number of consecutive successful or failed probes that allow or deny traffic to this endpoint. Possible values range from `1` to `100`. The default value is `1`.
        pub probe_threshold: pulumi_wasm_rust::Output<Option<i32>>,
        /// Specifies the protocol of the end point. Possible values are `Http`, `Https` or `Tcp`. If TCP is specified, a received ACK is required for the probe to be successful. If HTTP is specified, a 200 OK response from the specified URI is required for the probe to be successful. Defaults to `Tcp`.
        pub protocol: pulumi_wasm_rust::Output<Option<String>>,
        /// The URI used for requesting health status from the backend endpoint. Required if protocol is set to `Http` or `Https`. Otherwise, it is not allowed.
        pub request_path: pulumi_wasm_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: ProbeArgs) -> ProbeResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let interval_in_seconds_binding = args.interval_in_seconds.get_inner();
        let loadbalancer_id_binding = args.loadbalancer_id.get_inner();
        let name_binding = args.name.get_inner();
        let number_of_probes_binding = args.number_of_probes.get_inner();
        let port_binding = args.port.get_inner();
        let probe_threshold_binding = args.probe_threshold.get_inner();
        let protocol_binding = args.protocol.get_inner();
        let request_path_binding = args.request_path.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:lb/probe:Probe".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "intervalInSeconds".into(),
                    value: &interval_in_seconds_binding,
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
                    name: "numberOfProbes".into(),
                    value: &number_of_probes_binding,
                },
                register_interface::ObjectField {
                    name: "port".into(),
                    value: &port_binding,
                },
                register_interface::ObjectField {
                    name: "probeThreshold".into(),
                    value: &probe_threshold_binding,
                },
                register_interface::ObjectField {
                    name: "protocol".into(),
                    value: &protocol_binding,
                },
                register_interface::ObjectField {
                    name: "requestPath".into(),
                    value: &request_path_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "intervalInSeconds".into(),
                },
                register_interface::ResultField {
                    name: "loadBalancerRules".into(),
                },
                register_interface::ResultField {
                    name: "loadbalancerId".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "numberOfProbes".into(),
                },
                register_interface::ResultField {
                    name: "port".into(),
                },
                register_interface::ResultField {
                    name: "probeThreshold".into(),
                },
                register_interface::ResultField {
                    name: "protocol".into(),
                },
                register_interface::ResultField {
                    name: "requestPath".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        ProbeResult {
            interval_in_seconds: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("intervalInSeconds").unwrap(),
            ),
            load_balancer_rules: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("loadBalancerRules").unwrap(),
            ),
            loadbalancer_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("loadbalancerId").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            number_of_probes: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("numberOfProbes").unwrap(),
            ),
            port: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("port").unwrap(),
            ),
            probe_threshold: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("probeThreshold").unwrap(),
            ),
            protocol: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("protocol").unwrap(),
            ),
            request_path: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("requestPath").unwrap(),
            ),
        }
    }
}