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
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod policy_based_route {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct PolicyBasedRouteArgs {
        /// An optional description of this resource.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The filter to match L4 traffic.
        /// Structure is documented below.
        #[builder(into)]
        pub filter: pulumi_gestalt_rust::InputOrOutput<
            super::super::types::networkconnectivity::PolicyBasedRouteFilter,
        >,
        /// The interconnect attachments that this policy-based route applies to.
        #[builder(into, default)]
        pub interconnect_attachment: pulumi_gestalt_rust::InputOrOutput<
            Option<
                super::super::types::networkconnectivity::PolicyBasedRouteInterconnectAttachment,
            >,
        >,
        /// User-defined labels. **Note**: This field is non-authoritative, and will only manage the labels present in your
        /// configuration. Please refer to the field 'effective_labels' for all of the labels present on the resource.
        #[builder(into, default)]
        pub labels: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The name of the policy based route.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Fully-qualified URL of the network that this route applies to, for example: projects/my-project/global/networks/my-network.
        #[builder(into)]
        pub network: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The IP address of a global-access-enabled L4 ILB that is the next hop for matching packets.
        #[builder(into, default)]
        pub next_hop_ilb_ip: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Other routes that will be referenced to determine the next hop of the packet. Possible values: ["DEFAULT_ROUTING"]
        #[builder(into, default)]
        pub next_hop_other_routes: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The priority of this policy-based route. Priority is used to break ties in cases where there are more than one matching
        /// policy-based routes found. In cases where multiple policy-based routes are matched, the one with the lowest-numbered
        /// priority value wins. The default value is 1000. The priority value must be from 1 to 65535, inclusive.
        #[builder(into, default)]
        pub priority: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// VM instances to which this policy-based route applies to.
        #[builder(into, default)]
        pub virtual_machine: pulumi_gestalt_rust::InputOrOutput<
            Option<
                super::super::types::networkconnectivity::PolicyBasedRouteVirtualMachine,
            >,
        >,
    }
    #[allow(dead_code)]
    pub struct PolicyBasedRouteResult {
        /// Time when the policy-based route was created.
        pub create_time: pulumi_gestalt_rust::Output<String>,
        /// An optional description of this resource.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// All of labels (key/value pairs) present on the resource in GCP, including the labels configured through Pulumi, other clients and services.
        pub effective_labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The filter to match L4 traffic.
        /// Structure is documented below.
        pub filter: pulumi_gestalt_rust::Output<
            super::super::types::networkconnectivity::PolicyBasedRouteFilter,
        >,
        /// The interconnect attachments that this policy-based route applies to.
        pub interconnect_attachment: pulumi_gestalt_rust::Output<
            Option<
                super::super::types::networkconnectivity::PolicyBasedRouteInterconnectAttachment,
            >,
        >,
        /// Type of this resource.
        pub kind: pulumi_gestalt_rust::Output<String>,
        /// User-defined labels. **Note**: This field is non-authoritative, and will only manage the labels present in your
        /// configuration. Please refer to the field 'effective_labels' for all of the labels present on the resource.
        pub labels: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The name of the policy based route.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Fully-qualified URL of the network that this route applies to, for example: projects/my-project/global/networks/my-network.
        pub network: pulumi_gestalt_rust::Output<String>,
        /// The IP address of a global-access-enabled L4 ILB that is the next hop for matching packets.
        pub next_hop_ilb_ip: pulumi_gestalt_rust::Output<Option<String>>,
        /// Other routes that will be referenced to determine the next hop of the packet. Possible values: ["DEFAULT_ROUTING"]
        pub next_hop_other_routes: pulumi_gestalt_rust::Output<Option<String>>,
        /// The priority of this policy-based route. Priority is used to break ties in cases where there are more than one matching
        /// policy-based routes found. In cases where multiple policy-based routes are matched, the one with the lowest-numbered
        /// priority value wins. The default value is 1000. The priority value must be from 1 to 65535, inclusive.
        pub priority: pulumi_gestalt_rust::Output<Option<i32>>,
        pub project: pulumi_gestalt_rust::Output<String>,
        /// The combination of labels configured directly on the resource
        /// and default labels configured on the provider.
        pub pulumi_labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Time when the policy-based route was created.
        pub update_time: pulumi_gestalt_rust::Output<String>,
        /// VM instances to which this policy-based route applies to.
        pub virtual_machine: pulumi_gestalt_rust::Output<
            Option<
                super::super::types::networkconnectivity::PolicyBasedRouteVirtualMachine,
            >,
        >,
        /// If potential misconfigurations are detected for this route, this field will be populated with warning messages.
        /// Structure is documented below.
        pub warnings: pulumi_gestalt_rust::Output<
            Vec<super::super::types::networkconnectivity::PolicyBasedRouteWarning>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: PolicyBasedRouteArgs,
    ) -> PolicyBasedRouteResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let description_binding = args.description.get_output(context);
        let filter_binding = args.filter.get_output(context);
        let interconnect_attachment_binding = args
            .interconnect_attachment
            .get_output(context);
        let labels_binding = args.labels.get_output(context);
        let name_binding = args.name.get_output(context);
        let network_binding = args.network.get_output(context);
        let next_hop_ilb_ip_binding = args.next_hop_ilb_ip.get_output(context);
        let next_hop_other_routes_binding = args
            .next_hop_other_routes
            .get_output(context);
        let priority_binding = args.priority.get_output(context);
        let project_binding = args.project.get_output(context);
        let virtual_machine_binding = args.virtual_machine.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:networkconnectivity/policyBasedRoute:PolicyBasedRoute".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: &description_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "filter".into(),
                    value: &filter_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "interconnectAttachment".into(),
                    value: &interconnect_attachment_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "labels".into(),
                    value: &labels_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "network".into(),
                    value: &network_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "nextHopIlbIp".into(),
                    value: &next_hop_ilb_ip_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "nextHopOtherRoutes".into(),
                    value: &next_hop_other_routes_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "priority".into(),
                    value: &priority_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "project".into(),
                    value: &project_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "virtualMachine".into(),
                    value: &virtual_machine_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        PolicyBasedRouteResult {
            create_time: o.get_field("createTime"),
            description: o.get_field("description"),
            effective_labels: o.get_field("effectiveLabels"),
            filter: o.get_field("filter"),
            interconnect_attachment: o.get_field("interconnectAttachment"),
            kind: o.get_field("kind"),
            labels: o.get_field("labels"),
            name: o.get_field("name"),
            network: o.get_field("network"),
            next_hop_ilb_ip: o.get_field("nextHopIlbIp"),
            next_hop_other_routes: o.get_field("nextHopOtherRoutes"),
            priority: o.get_field("priority"),
            project: o.get_field("project"),
            pulumi_labels: o.get_field("pulumiLabels"),
            update_time: o.get_field("updateTime"),
            virtual_machine: o.get_field("virtualMachine"),
            warnings: o.get_field("warnings"),
        }
    }
}
