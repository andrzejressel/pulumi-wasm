/// Manages a VPC network or legacy network resource on GCP.
///
///
/// To get more information about Network, see:
///
/// * [API documentation](https://cloud.google.com/compute/docs/reference/rest/v1/networks)
/// * How-to Guides
///     * [Official Documentation](https://cloud.google.com/vpc/docs/vpc)
///
/// ## Example Usage
///
/// ### Network Basic
///
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let vpcNetwork = network::create(
///         "vpcNetwork",
///         NetworkArgs::builder().name("vpc-network").build_struct(),
///     );
/// }
/// ```
/// ### Network Custom Mtu
///
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let vpcNetwork = network::create(
///         "vpcNetwork",
///         NetworkArgs::builder()
///             .auto_create_subnetworks(true)
///             .mtu(1460)
///             .name("vpc-network")
///             .project("my-project-name")
///             .build_struct(),
///     );
/// }
/// ```
/// ### Network Custom Firewall Enforcement Order
///
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let vpcNetwork = network::create(
///         "vpcNetwork",
///         NetworkArgs::builder()
///             .auto_create_subnetworks(true)
///             .name("vpc-network")
///             .network_firewall_policy_enforcement_order("BEFORE_CLASSIC_FIREWALL")
///             .project("my-project-name")
///             .build_struct(),
///     );
/// }
/// ```
/// ### Network Bgp Best Path Selection Mode
///
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let vpcNetwork = network::create(
///         "vpcNetwork",
///         NetworkArgs::builder()
///             .name("vpc-network")
///             .project("my-project-name")
///             .routing_mode("GLOBAL")
///             .build_struct(),
///     );
/// }
/// ```
/// ### Network Bgp Best Path Selection Mode Standard
///
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let vpcNetwork = network::create(
///         "vpcNetwork",
///         NetworkArgs::builder()
///             .bgp_best_path_selection_mode("STANDARD")
///             .name("vpc-network")
///             .project("my-project-name")
///             .routing_mode("GLOBAL")
///             .build_struct(),
///     );
/// }
/// ```
/// ### Network Bgp Best Path Selection Mode Standard Custom Fields
///
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let vpcNetwork = network::create(
///         "vpcNetwork",
///         NetworkArgs::builder()
///             .bgp_always_compare_med(true)
///             .bgp_best_path_selection_mode("STANDARD")
///             .bgp_inter_region_cost("ADD_COST_TO_MED")
///             .name("vpc-network")
///             .project("my-project-name")
///             .routing_mode("GLOBAL")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Network can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/global/networks/{{name}}`
///
/// * `{{project}}/{{name}}`
///
/// * `{{name}}`
///
/// When using the `pulumi import` command, Network can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:compute/network:Network default projects/{{project}}/global/networks/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:compute/network:Network default {{project}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:compute/network:Network default {{name}}
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod network {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct NetworkArgs {
        /// When set to `true`, the network is created in "auto subnet mode" and
        /// it will create a subnet for each region automatically across the
        /// `10.128.0.0/9` address range.
        /// When set to `false`, the network is created in "custom subnet mode" so
        /// the user can explicitly connect subnetwork resources.
        #[builder(into, default)]
        pub auto_create_subnetworks: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Enables/disables the comparison of MED across routes with different Neighbor ASNs.
        /// This value can only be set if the --bgp-best-path-selection-mode is STANDARD
        #[builder(into, default)]
        pub bgp_always_compare_med: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// The BGP best selection algorithm to be employed. MODE can be LEGACY or STANDARD.
        /// Possible values are: `LEGACY`, `STANDARD`.
        #[builder(into, default)]
        pub bgp_best_path_selection_mode: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// Choice of the behavior of inter-regional cost and MED in the BPS algorithm.
        /// Possible values are: `DEFAULT`, `ADD_COST_TO_MED`.
        #[builder(into, default)]
        pub bgp_inter_region_cost: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// If set to `true`, default routes (`0.0.0.0/0`) will be deleted
        /// immediately after network creation. Defaults to `false`.
        #[builder(into, default)]
        pub delete_default_routes_on_create: pulumi_gestalt_rust::InputOrOutput<
            Option<bool>,
        >,
        /// An optional description of this resource. The resource must be
        /// recreated to modify this field.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Enable ULA internal ipv6 on this network. Enabling this feature will assign
        /// a /48 from google defined ULA prefix fd20::/20.
        #[builder(into, default)]
        pub enable_ula_internal_ipv6: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// When enabling ula internal ipv6, caller optionally can specify the /48 range
        /// they want from the google defined ULA prefix fd20::/20. The input must be a
        /// valid /48 ULA IPv6 address and must be within the fd20::/20. Operation will
        /// fail if the speficied /48 is already in used by another resource.
        /// If the field is not speficied, then a /48 range will be randomly allocated from fd20::/20 and returned via this field.
        #[builder(into, default)]
        pub internal_ipv6_range: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Maximum Transmission Unit in bytes. The default value is 1460 bytes.
        /// The minimum value for this field is 1300 and the maximum value is 8896 bytes (jumbo frames).
        /// Note that packets larger than 1500 bytes (standard Ethernet) can be subject to TCP-MSS clamping or dropped
        /// with an ICMP `Fragmentation-Needed` message if the packets are routed to the Internet or other VPCs
        /// with varying MTUs.
        #[builder(into, default)]
        pub mtu: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// Name of the resource. Provided by the client when the resource is
        /// created. The name must be 1-63 characters long, and comply with
        /// RFC1035. Specifically, the name must be 1-63 characters long and match
        /// the regular expression `a-z?` which means the
        /// first character must be a lowercase letter, and all following
        /// characters must be a dash, lowercase letter, or digit, except the last
        /// character, which cannot be a dash.
        ///
        ///
        /// - - -
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Set the order that Firewall Rules and Firewall Policies are evaluated.
        /// Default value is `AFTER_CLASSIC_FIREWALL`.
        /// Possible values are: `BEFORE_CLASSIC_FIREWALL`, `AFTER_CLASSIC_FIREWALL`.
        #[builder(into, default)]
        pub network_firewall_policy_enforcement_order: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// A full or partial URL of the network profile to apply to this network.
        /// This field can be set only at resource creation time. For example, the
        /// following are valid URLs:
        /// * https://www.googleapis.com/compute/beta/projects/{projectId}/global/networkProfiles/{network_profile_name}
        /// * projects/{projectId}/global/networkProfiles/{network_profile_name}
        #[builder(into, default)]
        pub network_profile: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The network-wide routing mode to use. If set to `REGIONAL`, this
        /// network's cloud routers will only advertise routes with subnetworks
        /// of this network in the same region as the router. If set to `GLOBAL`,
        /// this network's cloud routers will advertise routes with all
        /// subnetworks of this network, across regions.
        /// Possible values are: `REGIONAL`, `GLOBAL`.
        #[builder(into, default)]
        pub routing_mode: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct NetworkResult {
        /// When set to `true`, the network is created in "auto subnet mode" and
        /// it will create a subnet for each region automatically across the
        /// `10.128.0.0/9` address range.
        /// When set to `false`, the network is created in "custom subnet mode" so
        /// the user can explicitly connect subnetwork resources.
        pub auto_create_subnetworks: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Enables/disables the comparison of MED across routes with different Neighbor ASNs.
        /// This value can only be set if the --bgp-best-path-selection-mode is STANDARD
        pub bgp_always_compare_med: pulumi_gestalt_rust::Output<bool>,
        /// The BGP best selection algorithm to be employed. MODE can be LEGACY or STANDARD.
        /// Possible values are: `LEGACY`, `STANDARD`.
        pub bgp_best_path_selection_mode: pulumi_gestalt_rust::Output<String>,
        /// Choice of the behavior of inter-regional cost and MED in the BPS algorithm.
        /// Possible values are: `DEFAULT`, `ADD_COST_TO_MED`.
        pub bgp_inter_region_cost: pulumi_gestalt_rust::Output<String>,
        /// If set to `true`, default routes (`0.0.0.0/0`) will be deleted
        /// immediately after network creation. Defaults to `false`.
        pub delete_default_routes_on_create: pulumi_gestalt_rust::Output<Option<bool>>,
        /// An optional description of this resource. The resource must be
        /// recreated to modify this field.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// Enable ULA internal ipv6 on this network. Enabling this feature will assign
        /// a /48 from google defined ULA prefix fd20::/20.
        pub enable_ula_internal_ipv6: pulumi_gestalt_rust::Output<Option<bool>>,
        /// The gateway address for default routing out of the network. This value
        /// is selected by GCP.
        pub gateway_ipv4: pulumi_gestalt_rust::Output<String>,
        /// When enabling ula internal ipv6, caller optionally can specify the /48 range
        /// they want from the google defined ULA prefix fd20::/20. The input must be a
        /// valid /48 ULA IPv6 address and must be within the fd20::/20. Operation will
        /// fail if the speficied /48 is already in used by another resource.
        /// If the field is not speficied, then a /48 range will be randomly allocated from fd20::/20 and returned via this field.
        pub internal_ipv6_range: pulumi_gestalt_rust::Output<String>,
        /// Maximum Transmission Unit in bytes. The default value is 1460 bytes.
        /// The minimum value for this field is 1300 and the maximum value is 8896 bytes (jumbo frames).
        /// Note that packets larger than 1500 bytes (standard Ethernet) can be subject to TCP-MSS clamping or dropped
        /// with an ICMP `Fragmentation-Needed` message if the packets are routed to the Internet or other VPCs
        /// with varying MTUs.
        pub mtu: pulumi_gestalt_rust::Output<i32>,
        /// Name of the resource. Provided by the client when the resource is
        /// created. The name must be 1-63 characters long, and comply with
        /// RFC1035. Specifically, the name must be 1-63 characters long and match
        /// the regular expression `a-z?` which means the
        /// first character must be a lowercase letter, and all following
        /// characters must be a dash, lowercase letter, or digit, except the last
        /// character, which cannot be a dash.
        ///
        ///
        /// - - -
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Set the order that Firewall Rules and Firewall Policies are evaluated.
        /// Default value is `AFTER_CLASSIC_FIREWALL`.
        /// Possible values are: `BEFORE_CLASSIC_FIREWALL`, `AFTER_CLASSIC_FIREWALL`.
        pub network_firewall_policy_enforcement_order: pulumi_gestalt_rust::Output<
            Option<String>,
        >,
        /// A full or partial URL of the network profile to apply to this network.
        /// This field can be set only at resource creation time. For example, the
        /// following are valid URLs:
        /// * https://www.googleapis.com/compute/beta/projects/{projectId}/global/networkProfiles/{network_profile_name}
        /// * projects/{projectId}/global/networkProfiles/{network_profile_name}
        pub network_profile: pulumi_gestalt_rust::Output<Option<String>>,
        /// The unique identifier for the resource. This identifier is defined by the server.
        pub numeric_id: pulumi_gestalt_rust::Output<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_gestalt_rust::Output<String>,
        /// The network-wide routing mode to use. If set to `REGIONAL`, this
        /// network's cloud routers will only advertise routes with subnetworks
        /// of this network in the same region as the router. If set to `GLOBAL`,
        /// this network's cloud routers will advertise routes with all
        /// subnetworks of this network, across regions.
        /// Possible values are: `REGIONAL`, `GLOBAL`.
        pub routing_mode: pulumi_gestalt_rust::Output<String>,
        /// The URI of the created resource.
        pub self_link: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: NetworkArgs,
    ) -> NetworkResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let auto_create_subnetworks_binding = args
            .auto_create_subnetworks
            .get_output(context)
            .get_inner();
        let bgp_always_compare_med_binding = args
            .bgp_always_compare_med
            .get_output(context)
            .get_inner();
        let bgp_best_path_selection_mode_binding = args
            .bgp_best_path_selection_mode
            .get_output(context)
            .get_inner();
        let bgp_inter_region_cost_binding = args
            .bgp_inter_region_cost
            .get_output(context)
            .get_inner();
        let delete_default_routes_on_create_binding = args
            .delete_default_routes_on_create
            .get_output(context)
            .get_inner();
        let description_binding = args.description.get_output(context).get_inner();
        let enable_ula_internal_ipv6_binding = args
            .enable_ula_internal_ipv6
            .get_output(context)
            .get_inner();
        let internal_ipv6_range_binding = args
            .internal_ipv6_range
            .get_output(context)
            .get_inner();
        let mtu_binding = args.mtu.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let network_firewall_policy_enforcement_order_binding = args
            .network_firewall_policy_enforcement_order
            .get_output(context)
            .get_inner();
        let network_profile_binding = args
            .network_profile
            .get_output(context)
            .get_inner();
        let project_binding = args.project.get_output(context).get_inner();
        let routing_mode_binding = args.routing_mode.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:compute/network:Network".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "autoCreateSubnetworks".into(),
                    value: &auto_create_subnetworks_binding,
                },
                register_interface::ObjectField {
                    name: "bgpAlwaysCompareMed".into(),
                    value: &bgp_always_compare_med_binding,
                },
                register_interface::ObjectField {
                    name: "bgpBestPathSelectionMode".into(),
                    value: &bgp_best_path_selection_mode_binding,
                },
                register_interface::ObjectField {
                    name: "bgpInterRegionCost".into(),
                    value: &bgp_inter_region_cost_binding,
                },
                register_interface::ObjectField {
                    name: "deleteDefaultRoutesOnCreate".into(),
                    value: &delete_default_routes_on_create_binding,
                },
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "enableUlaInternalIpv6".into(),
                    value: &enable_ula_internal_ipv6_binding,
                },
                register_interface::ObjectField {
                    name: "internalIpv6Range".into(),
                    value: &internal_ipv6_range_binding,
                },
                register_interface::ObjectField {
                    name: "mtu".into(),
                    value: &mtu_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "networkFirewallPolicyEnforcementOrder".into(),
                    value: &network_firewall_policy_enforcement_order_binding,
                },
                register_interface::ObjectField {
                    name: "networkProfile".into(),
                    value: &network_profile_binding,
                },
                register_interface::ObjectField {
                    name: "project".into(),
                    value: &project_binding,
                },
                register_interface::ObjectField {
                    name: "routingMode".into(),
                    value: &routing_mode_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        NetworkResult {
            auto_create_subnetworks: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("autoCreateSubnetworks"),
            ),
            bgp_always_compare_med: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("bgpAlwaysCompareMed"),
            ),
            bgp_best_path_selection_mode: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("bgpBestPathSelectionMode"),
            ),
            bgp_inter_region_cost: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("bgpInterRegionCost"),
            ),
            delete_default_routes_on_create: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("deleteDefaultRoutesOnCreate"),
            ),
            description: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            enable_ula_internal_ipv6: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("enableUlaInternalIpv6"),
            ),
            gateway_ipv4: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("gatewayIpv4"),
            ),
            internal_ipv6_range: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("internalIpv6Range"),
            ),
            mtu: pulumi_gestalt_rust::__private::into_domain(o.extract_field("mtu")),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            network_firewall_policy_enforcement_order: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("networkFirewallPolicyEnforcementOrder"),
            ),
            network_profile: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("networkProfile"),
            ),
            numeric_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("numericId"),
            ),
            project: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("project"),
            ),
            routing_mode: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("routingMode"),
            ),
            self_link: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("selfLink"),
            ),
        }
    }
}
