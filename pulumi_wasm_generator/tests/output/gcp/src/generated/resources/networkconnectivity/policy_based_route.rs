/// Policy-based Routes are more powerful routes that route L4 network traffic based on not just destination IP, but also source IP, protocol and more. A Policy-based Route always take precedence when it conflicts with other types of routes.
///
///
/// To get more information about PolicyBasedRoute, see:
///
/// * [API documentation](https://cloud.google.com/network-connectivity/docs/reference/networkconnectivity/rest/v1/projects.locations.global.policyBasedRoutes)
/// * How-to Guides
///     * [Use policy-based routes](https://cloud.google.com/vpc/docs/use-policy-based-routes#api)
///
/// ## Example Usage
///
/// ### Network Connectivity Policy Based Route Basic
///
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let default = policy_based_route::create(
///         "default",
///         PolicyBasedRouteArgs::builder()
///             .filter(
///                 PolicyBasedRouteFilter::builder().protocolVersion("IPV4").build_struct(),
///             )
///             .name("my-pbr")
///             .network("${myNetwork.id}")
///             .next_hop_other_routes("DEFAULT_ROUTING")
///             .build_struct(),
///     );
///     let myNetwork = network::create(
///         "myNetwork",
///         NetworkArgs::builder()
///             .auto_create_subnetworks(false)
///             .name("my-network")
///             .build_struct(),
///     );
/// }
/// ```
/// ### Network Connectivity Policy Based Route Full
///
///
/// ```yaml
/// resources:
///   default:
///     type: gcp:networkconnectivity:PolicyBasedRoute
///     properties:
///       name: my-pbr
///       description: My routing policy
///       network: ${myNetwork.id}
///       priority: 2302
///       filter:
///         protocolVersion: IPV4
///         ipProtocol: UDP
///         srcRange: 10.0.0.0/24
///         destRange: 0.0.0.0/0
///       nextHopIlbIp: ${ilb.address}
///       virtualMachine:
///         tags:
///           - restricted
///       labels:
///         env: default
///   myNetwork:
///     type: gcp:compute:Network
///     name: my_network
///     properties:
///       name: my-network
///       autoCreateSubnetworks: false
///   # This example substitutes an arbitrary internal IP for an internal network
///   # load balancer for brevity. Consult https://cloud.google.com/load-balancing/docs/internal
///   # to set one up.
///   ilb:
///     type: gcp:compute:GlobalAddress
///     properties:
///       name: my-ilb
/// ```
///
/// ## Import
///
/// PolicyBasedRoute can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/locations/global/policyBasedRoutes/{{name}}`
///
/// * `{{project}}/{{name}}`
///
/// * `{{name}}`
///
/// When using the `pulumi import` command, PolicyBasedRoute can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:networkconnectivity/policyBasedRoute:PolicyBasedRoute default projects/{{project}}/locations/global/policyBasedRoutes/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:networkconnectivity/policyBasedRoute:PolicyBasedRoute default {{project}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:networkconnectivity/policyBasedRoute:PolicyBasedRoute default {{name}}
/// ```
///
pub mod policy_based_route {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct PolicyBasedRouteArgs {
        /// An optional description of this resource.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// The filter to match L4 traffic.
        /// Structure is documented below.
        #[builder(into)]
        pub filter: pulumi_wasm_rust::Output<
            super::super::types::networkconnectivity::PolicyBasedRouteFilter,
        >,
        /// The interconnect attachments that this policy-based route applies to.
        #[builder(into, default)]
        pub interconnect_attachment: pulumi_wasm_rust::Output<
            Option<
                super::super::types::networkconnectivity::PolicyBasedRouteInterconnectAttachment,
            >,
        >,
        /// User-defined labels. **Note**: This field is non-authoritative, and will only manage the labels present in your
        /// configuration. Please refer to the field 'effective_labels' for all of the labels present on the resource.
        #[builder(into, default)]
        pub labels: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The name of the policy based route.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// Fully-qualified URL of the network that this route applies to, for example: projects/my-project/global/networks/my-network.
        #[builder(into)]
        pub network: pulumi_wasm_rust::Output<String>,
        /// The IP address of a global-access-enabled L4 ILB that is the next hop for matching packets.
        #[builder(into, default)]
        pub next_hop_ilb_ip: pulumi_wasm_rust::Output<Option<String>>,
        /// Other routes that will be referenced to determine the next hop of the packet. Possible values: ["DEFAULT_ROUTING"]
        #[builder(into, default)]
        pub next_hop_other_routes: pulumi_wasm_rust::Output<Option<String>>,
        /// The priority of this policy-based route. Priority is used to break ties in cases where there are more than one matching
        /// policy-based routes found. In cases where multiple policy-based routes are matched, the one with the lowest-numbered
        /// priority value wins. The default value is 1000. The priority value must be from 1 to 65535, inclusive.
        #[builder(into, default)]
        pub priority: pulumi_wasm_rust::Output<Option<i32>>,
        #[builder(into, default)]
        pub project: pulumi_wasm_rust::Output<Option<String>>,
        /// VM instances to which this policy-based route applies to.
        #[builder(into, default)]
        pub virtual_machine: pulumi_wasm_rust::Output<
            Option<
                super::super::types::networkconnectivity::PolicyBasedRouteVirtualMachine,
            >,
        >,
    }
    #[allow(dead_code)]
    pub struct PolicyBasedRouteResult {
        /// Time when the policy-based route was created.
        pub create_time: pulumi_wasm_rust::Output<String>,
        /// An optional description of this resource.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// All of labels (key/value pairs) present on the resource in GCP, including the labels configured through Pulumi, other clients and services.
        pub effective_labels: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The filter to match L4 traffic.
        /// Structure is documented below.
        pub filter: pulumi_wasm_rust::Output<
            super::super::types::networkconnectivity::PolicyBasedRouteFilter,
        >,
        /// The interconnect attachments that this policy-based route applies to.
        pub interconnect_attachment: pulumi_wasm_rust::Output<
            Option<
                super::super::types::networkconnectivity::PolicyBasedRouteInterconnectAttachment,
            >,
        >,
        /// Type of this resource.
        pub kind: pulumi_wasm_rust::Output<String>,
        /// User-defined labels. **Note**: This field is non-authoritative, and will only manage the labels present in your
        /// configuration. Please refer to the field 'effective_labels' for all of the labels present on the resource.
        pub labels: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The name of the policy based route.
        pub name: pulumi_wasm_rust::Output<String>,
        /// Fully-qualified URL of the network that this route applies to, for example: projects/my-project/global/networks/my-network.
        pub network: pulumi_wasm_rust::Output<String>,
        /// The IP address of a global-access-enabled L4 ILB that is the next hop for matching packets.
        pub next_hop_ilb_ip: pulumi_wasm_rust::Output<Option<String>>,
        /// Other routes that will be referenced to determine the next hop of the packet. Possible values: ["DEFAULT_ROUTING"]
        pub next_hop_other_routes: pulumi_wasm_rust::Output<Option<String>>,
        /// The priority of this policy-based route. Priority is used to break ties in cases where there are more than one matching
        /// policy-based routes found. In cases where multiple policy-based routes are matched, the one with the lowest-numbered
        /// priority value wins. The default value is 1000. The priority value must be from 1 to 65535, inclusive.
        pub priority: pulumi_wasm_rust::Output<Option<i32>>,
        pub project: pulumi_wasm_rust::Output<String>,
        /// The combination of labels configured directly on the resource
        /// and default labels configured on the provider.
        pub pulumi_labels: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Time when the policy-based route was created.
        pub update_time: pulumi_wasm_rust::Output<String>,
        /// VM instances to which this policy-based route applies to.
        pub virtual_machine: pulumi_wasm_rust::Output<
            Option<
                super::super::types::networkconnectivity::PolicyBasedRouteVirtualMachine,
            >,
        >,
        /// If potential misconfigurations are detected for this route, this field will be populated with warning messages.
        /// Structure is documented below.
        pub warnings: pulumi_wasm_rust::Output<
            Vec<super::super::types::networkconnectivity::PolicyBasedRouteWarning>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: PolicyBasedRouteArgs) -> PolicyBasedRouteResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let description_binding = args.description.get_inner();
        let filter_binding = args.filter.get_inner();
        let interconnect_attachment_binding = args.interconnect_attachment.get_inner();
        let labels_binding = args.labels.get_inner();
        let name_binding = args.name.get_inner();
        let network_binding = args.network.get_inner();
        let next_hop_ilb_ip_binding = args.next_hop_ilb_ip.get_inner();
        let next_hop_other_routes_binding = args.next_hop_other_routes.get_inner();
        let priority_binding = args.priority.get_inner();
        let project_binding = args.project.get_inner();
        let virtual_machine_binding = args.virtual_machine.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:networkconnectivity/policyBasedRoute:PolicyBasedRoute".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "filter".into(),
                    value: &filter_binding,
                },
                register_interface::ObjectField {
                    name: "interconnectAttachment".into(),
                    value: &interconnect_attachment_binding,
                },
                register_interface::ObjectField {
                    name: "labels".into(),
                    value: &labels_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "network".into(),
                    value: &network_binding,
                },
                register_interface::ObjectField {
                    name: "nextHopIlbIp".into(),
                    value: &next_hop_ilb_ip_binding,
                },
                register_interface::ObjectField {
                    name: "nextHopOtherRoutes".into(),
                    value: &next_hop_other_routes_binding,
                },
                register_interface::ObjectField {
                    name: "priority".into(),
                    value: &priority_binding,
                },
                register_interface::ObjectField {
                    name: "project".into(),
                    value: &project_binding,
                },
                register_interface::ObjectField {
                    name: "virtualMachine".into(),
                    value: &virtual_machine_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "createTime".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "effectiveLabels".into(),
                },
                register_interface::ResultField {
                    name: "filter".into(),
                },
                register_interface::ResultField {
                    name: "interconnectAttachment".into(),
                },
                register_interface::ResultField {
                    name: "kind".into(),
                },
                register_interface::ResultField {
                    name: "labels".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "network".into(),
                },
                register_interface::ResultField {
                    name: "nextHopIlbIp".into(),
                },
                register_interface::ResultField {
                    name: "nextHopOtherRoutes".into(),
                },
                register_interface::ResultField {
                    name: "priority".into(),
                },
                register_interface::ResultField {
                    name: "project".into(),
                },
                register_interface::ResultField {
                    name: "pulumiLabels".into(),
                },
                register_interface::ResultField {
                    name: "updateTime".into(),
                },
                register_interface::ResultField {
                    name: "virtualMachine".into(),
                },
                register_interface::ResultField {
                    name: "warnings".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        PolicyBasedRouteResult {
            create_time: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("createTime").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            effective_labels: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("effectiveLabels").unwrap(),
            ),
            filter: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("filter").unwrap(),
            ),
            interconnect_attachment: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("interconnectAttachment").unwrap(),
            ),
            kind: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("kind").unwrap(),
            ),
            labels: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("labels").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            network: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("network").unwrap(),
            ),
            next_hop_ilb_ip: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("nextHopIlbIp").unwrap(),
            ),
            next_hop_other_routes: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("nextHopOtherRoutes").unwrap(),
            ),
            priority: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("priority").unwrap(),
            ),
            project: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("project").unwrap(),
            ),
            pulumi_labels: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("pulumiLabels").unwrap(),
            ),
            update_time: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("updateTime").unwrap(),
            ),
            virtual_machine: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("virtualMachine").unwrap(),
            ),
            warnings: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("warnings").unwrap(),
            ),
        }
    }
}
