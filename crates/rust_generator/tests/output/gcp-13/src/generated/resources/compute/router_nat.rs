/// A NAT service created in a router.
///
/// > **Note:** Recreating a `gcp.compute.Address` that is being used by `gcp.compute.RouterNat` will give a `resourceInUseByAnotherResource` error.
/// Use `lifecycle.create_before_destroy` on this address resource to avoid this type of error as shown in the Manual Ips example.
///
///
/// To get more information about RouterNat, see:
///
/// * [API documentation](https://cloud.google.com/compute/docs/reference/rest/v1/routers)
/// * How-to Guides
///     * [Google Cloud Router](https://cloud.google.com/router/docs/)
///
/// ## Example Usage
///
/// ### Router Nat Basic
///
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let nat = router_nat::create(
///         "nat",
///         RouterNatArgs::builder()
///             .log_config(
///                 RouterNatLogConfig::builder()
///                     .enable(true)
///                     .filter("ERRORS_ONLY")
///                     .build_struct(),
///             )
///             .name("my-router-nat")
///             .nat_ip_allocate_option("AUTO_ONLY")
///             .region("${router.region}")
///             .router("${router.name}")
///             .source_subnetwork_ip_ranges_to_nat("ALL_SUBNETWORKS_ALL_IP_RANGES")
///             .build_struct(),
///     );
///     let net = network::create(
///         "net",
///         NetworkArgs::builder().name("my-network").build_struct(),
///     );
///     let router = router::create(
///         "router",
///         RouterArgs::builder()
///             .bgp(RouterBgp::builder().asn(64514).build_struct())
///             .name("my-router")
///             .network("${net.id}")
///             .region("${subnet.region}")
///             .build_struct(),
///     );
///     let subnet = subnetwork::create(
///         "subnet",
///         SubnetworkArgs::builder()
///             .ip_cidr_range("10.0.0.0/16")
///             .name("my-subnetwork")
///             .network("${net.id}")
///             .region("us-central1")
///             .build_struct(),
///     );
/// }
/// ```
/// ### Router Nat Rules
///
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let addr1 = address::create(
///         "addr1",
///         AddressArgs::builder()
///             .name("nat-address1")
///             .region("${subnet.region}")
///             .build_struct(),
///     );
///     let addr2 = address::create(
///         "addr2",
///         AddressArgs::builder()
///             .name("nat-address2")
///             .region("${subnet.region}")
///             .build_struct(),
///     );
///     let addr3 = address::create(
///         "addr3",
///         AddressArgs::builder()
///             .name("nat-address3")
///             .region("${subnet.region}")
///             .build_struct(),
///     );
///     let natRules = router_nat::create(
///         "natRules",
///         RouterNatArgs::builder()
///             .enable_endpoint_independent_mapping(false)
///             .name("my-router-nat")
///             .nat_ip_allocate_option("MANUAL_ONLY")
///             .nat_ips(vec!["${addr1.selfLink}",])
///             .region("${router.region}")
///             .router("${router.name}")
///             .rules(
///                 vec![
///                     RouterNatRule::builder().action(RouterNatRuleAction::builder()
///                     .sourceNatActiveIps(vec!["${addr2.selfLink}", "${addr3.selfLink}",])
///                     .build_struct()).description("nat rules example"). match
///                     ("inIpRange(destination.ip, '1.1.0.0/16') || inIpRange(destination.ip, '2.2.0.0/16')")
///                     .ruleNumber(100).build_struct(),
///                 ],
///             )
///             .source_subnetwork_ip_ranges_to_nat("LIST_OF_SUBNETWORKS")
///             .subnetworks(
///                 vec![
///                     RouterNatSubnetwork::builder().name("${subnet.id}")
///                     .sourceIpRangesToNats(vec!["ALL_IP_RANGES",]).build_struct(),
///                 ],
///             )
///             .build_struct(),
///     );
///     let net = network::create(
///         "net",
///         NetworkArgs::builder()
///             .auto_create_subnetworks(false)
///             .name("my-network")
///             .build_struct(),
///     );
///     let router = router::create(
///         "router",
///         RouterArgs::builder()
///             .name("my-router")
///             .network("${net.id}")
///             .region("${subnet.region}")
///             .build_struct(),
///     );
///     let subnet = subnetwork::create(
///         "subnet",
///         SubnetworkArgs::builder()
///             .ip_cidr_range("10.0.0.0/16")
///             .name("my-subnetwork")
///             .network("${net.id}")
///             .region("us-central1")
///             .build_struct(),
///     );
/// }
/// ```
/// ### Router Nat Private
///
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let hub = hub::create(
///         "hub",
///         HubArgs::builder()
///             .description("vpc hub for inter vpc nat")
///             .name("my-hub")
///             .build_struct(),
///     );
///     let natType = router_nat::create(
///         "natType",
///         RouterNatArgs::builder()
///             .enable_dynamic_port_allocation(false)
///             .enable_endpoint_independent_mapping(false)
///             .min_ports_per_vm(32)
///             .name("my-router-nat")
///             .region("${router.region}")
///             .router("${router.name}")
///             .rules(
///                 vec![
///                     RouterNatRule::builder().action(RouterNatRuleAction::builder()
///                     .sourceNatActiveRanges(vec!["${subnet.selfLink}",]).build_struct())
///                     .description("rule for private nat"). match
///                     ("nexthop.hub == \"//networkconnectivity.googleapis.com/projects/acm-test-proj-123/locations/global/hubs/my-hub\"")
///                     .ruleNumber(100).build_struct(),
///                 ],
///             )
///             .source_subnetwork_ip_ranges_to_nat("LIST_OF_SUBNETWORKS")
///             .subnetworks(
///                 vec![
///                     RouterNatSubnetwork::builder().name("${subnet.id}")
///                     .sourceIpRangesToNats(vec!["ALL_IP_RANGES",]).build_struct(),
///                 ],
///             )
///             .type_("PRIVATE")
///             .build_struct(),
///     );
///     let net = network::create(
///         "net",
///         NetworkArgs::builder().name("my-network").build_struct(),
///     );
///     let router = router::create(
///         "router",
///         RouterArgs::builder()
///             .name("my-router")
///             .network("${net.id}")
///             .region("${subnet.region}")
///             .build_struct(),
///     );
///     let spoke = spoke::create(
///         "spoke",
///         SpokeArgs::builder()
///             .description("vpc spoke for inter vpc nat")
///             .hub("${hub.id}")
///             .linked_vpc_network(
///                 SpokeLinkedVpcNetwork::builder()
///                     .excludeExportRanges(vec!["198.51.100.0/24", "10.10.0.0/16",])
///                     .uri("${net.selfLink}")
///                     .build_struct(),
///             )
///             .location("global")
///             .name("my-spoke")
///             .build_struct(),
///     );
///     let subnet = subnetwork::create(
///         "subnet",
///         SubnetworkArgs::builder()
///             .ip_cidr_range("10.0.0.0/16")
///             .name("my-subnetwork")
///             .network("${net.id}")
///             .purpose("PRIVATE_NAT")
///             .region("us-central1")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// RouterNat can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/regions/{{region}}/routers/{{router}}/{{name}}`
///
/// * `{{project}}/{{region}}/{{router}}/{{name}}`
///
/// * `{{region}}/{{router}}/{{name}}`
///
/// * `{{router}}/{{name}}`
///
/// When using the `pulumi import` command, RouterNat can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:compute/routerNat:RouterNat default projects/{{project}}/regions/{{region}}/routers/{{router}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:compute/routerNat:RouterNat default {{project}}/{{region}}/{{router}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:compute/routerNat:RouterNat default {{region}}/{{router}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:compute/routerNat:RouterNat default {{router}}/{{name}}
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod router_nat {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct RouterNatArgs {
        /// The network tier to use when automatically reserving NAT IP addresses.
        /// Must be one of: PREMIUM, STANDARD. If not specified, then the current
        /// project-level default tier is used.
        /// Possible values are: `PREMIUM`, `STANDARD`.
        #[builder(into, default)]
        pub auto_network_tier: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A list of URLs of the IP resources to be drained. These IPs must be
        /// valid static external IPs that have been assigned to the NAT.
        #[builder(into, default)]
        pub drain_nat_ips: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// Enable Dynamic Port Allocation.
        /// If minPortsPerVm is set, minPortsPerVm must be set to a power of two greater than or equal to 32.
        /// If minPortsPerVm is not set, a minimum of 32 ports will be allocated to a VM from this NAT config.
        /// If maxPortsPerVm is set, maxPortsPerVm must be set to a power of two greater than minPortsPerVm.
        /// If maxPortsPerVm is not set, a maximum of 65536 ports will be allocated to a VM from this NAT config.
        /// Mutually exclusive with enableEndpointIndependentMapping.
        #[builder(into, default)]
        pub enable_dynamic_port_allocation: pulumi_gestalt_rust::InputOrOutput<
            Option<bool>,
        >,
        /// Enable endpoint independent mapping.
        /// For more information see the [official documentation](https://cloud.google.com/nat/docs/overview#specs-rfcs).
        #[builder(into, default)]
        pub enable_endpoint_independent_mapping: pulumi_gestalt_rust::InputOrOutput<
            Option<bool>,
        >,
        /// Specifies the endpoint Types supported by the NAT Gateway.
        /// Supported values include:
        /// `ENDPOINT_TYPE_VM`, `ENDPOINT_TYPE_SWG`,
        /// `ENDPOINT_TYPE_MANAGED_PROXY_LB`.
        #[builder(into, default)]
        pub endpoint_types: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// Timeout (in seconds) for ICMP connections. Defaults to 30s if not set.
        #[builder(into, default)]
        pub icmp_idle_timeout_sec: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// Self-links of NAT IPs to be used as initial value for creation alongside a RouterNatAddress resource.
        /// Conflicts with natIps and drainNatIps. Only valid if natIpAllocateOption is set to MANUAL_ONLY.
        #[builder(into, default)]
        pub initial_nat_ips: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// Configuration for logging on NAT
        /// Structure is documented below.
        #[builder(into, default)]
        pub log_config: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::compute::RouterNatLogConfig>,
        >,
        /// Maximum number of ports allocated to a VM from this NAT.
        /// This field can only be set when enableDynamicPortAllocation is enabled.
        #[builder(into, default)]
        pub max_ports_per_vm: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// Minimum number of ports allocated to a VM from this NAT. Defaults to 64 for static port allocation and 32 dynamic port allocation if not set.
        #[builder(into, default)]
        pub min_ports_per_vm: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// Name of the NAT service. The name must be 1-63 characters long and
        /// comply with RFC1035.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// How external IPs should be allocated for this NAT. Valid values are
        /// `AUTO_ONLY` for only allowing NAT IPs allocated by Google Cloud
        /// Platform, or `MANUAL_ONLY` for only user-allocated NAT IP addresses.
        /// Possible values are: `MANUAL_ONLY`, `AUTO_ONLY`.
        #[builder(into, default)]
        pub nat_ip_allocate_option: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Self-links of NAT IPs. Only valid if natIpAllocateOption
        /// is set to MANUAL_ONLY.
        /// If this field is used alongside with a count created list of address resources `google_compute_address.foobar.*.self_link`,
        /// the access level resource for the address resource must have a `lifecycle` block with `create_before_destroy = true` so
        /// the number of resources can be increased/decreased without triggering the `resourceInUseByAnotherResource` error.
        #[builder(into, default)]
        pub nat_ips: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Region where the router and NAT reside.
        #[builder(into, default)]
        pub region: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of the Cloud Router in which this NAT will be configured.
        ///
        ///
        /// - - -
        #[builder(into)]
        pub router: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A list of rules associated with this NAT.
        /// Structure is documented below.
        #[builder(into, default)]
        pub rules: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::compute::RouterNatRule>>,
        >,
        /// How NAT should be configured per Subnetwork.
        /// If `ALL_SUBNETWORKS_ALL_IP_RANGES`, all of the
        /// IP ranges in every Subnetwork are allowed to Nat.
        /// If `ALL_SUBNETWORKS_ALL_PRIMARY_IP_RANGES`, all of the primary IP
        /// ranges in every Subnetwork are allowed to Nat.
        /// `LIST_OF_SUBNETWORKS`: A list of Subnetworks are allowed to Nat
        /// (specified in the field subnetwork below). Note that if this field
        /// contains ALL_SUBNETWORKS_ALL_IP_RANGES or
        /// ALL_SUBNETWORKS_ALL_PRIMARY_IP_RANGES, then there should not be any
        /// other RouterNat section in any Router for this network in this region.
        /// Possible values are: `ALL_SUBNETWORKS_ALL_IP_RANGES`, `ALL_SUBNETWORKS_ALL_PRIMARY_IP_RANGES`, `LIST_OF_SUBNETWORKS`.
        #[builder(into)]
        pub source_subnetwork_ip_ranges_to_nat: pulumi_gestalt_rust::InputOrOutput<
            String,
        >,
        /// One or more subnetwork NAT configurations. Only used if
        /// `source_subnetwork_ip_ranges_to_nat` is set to `LIST_OF_SUBNETWORKS`
        /// Structure is documented below.
        #[builder(into, default)]
        pub subnetworks: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::compute::RouterNatSubnetwork>>,
        >,
        /// Timeout (in seconds) for TCP established connections.
        /// Defaults to 1200s if not set.
        #[builder(into, default)]
        pub tcp_established_idle_timeout_sec: pulumi_gestalt_rust::InputOrOutput<
            Option<i32>,
        >,
        /// Timeout (in seconds) for TCP connections that are in TIME_WAIT state.
        /// Defaults to 120s if not set.
        #[builder(into, default)]
        pub tcp_time_wait_timeout_sec: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// Timeout (in seconds) for TCP transitory connections.
        /// Defaults to 30s if not set.
        #[builder(into, default)]
        pub tcp_transitory_idle_timeout_sec: pulumi_gestalt_rust::InputOrOutput<
            Option<i32>,
        >,
        /// Indicates whether this NAT is used for public or private IP translation.
        /// If unspecified, it defaults to PUBLIC.
        /// If `PUBLIC` NAT used for public IP translation.
        /// If `PRIVATE` NAT used for private IP translation.
        /// Default value is `PUBLIC`.
        /// Possible values are: `PUBLIC`, `PRIVATE`.
        #[builder(into, default)]
        pub type_: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Timeout (in seconds) for UDP connections. Defaults to 30s if not set.
        #[builder(into, default)]
        pub udp_idle_timeout_sec: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
    }
    #[allow(dead_code)]
    pub struct RouterNatResult {
        /// The network tier to use when automatically reserving NAT IP addresses.
        /// Must be one of: PREMIUM, STANDARD. If not specified, then the current
        /// project-level default tier is used.
        /// Possible values are: `PREMIUM`, `STANDARD`.
        pub auto_network_tier: pulumi_gestalt_rust::Output<String>,
        /// A list of URLs of the IP resources to be drained. These IPs must be
        /// valid static external IPs that have been assigned to the NAT.
        pub drain_nat_ips: pulumi_gestalt_rust::Output<Vec<String>>,
        /// Enable Dynamic Port Allocation.
        /// If minPortsPerVm is set, minPortsPerVm must be set to a power of two greater than or equal to 32.
        /// If minPortsPerVm is not set, a minimum of 32 ports will be allocated to a VM from this NAT config.
        /// If maxPortsPerVm is set, maxPortsPerVm must be set to a power of two greater than minPortsPerVm.
        /// If maxPortsPerVm is not set, a maximum of 65536 ports will be allocated to a VM from this NAT config.
        /// Mutually exclusive with enableEndpointIndependentMapping.
        pub enable_dynamic_port_allocation: pulumi_gestalt_rust::Output<bool>,
        /// Enable endpoint independent mapping.
        /// For more information see the [official documentation](https://cloud.google.com/nat/docs/overview#specs-rfcs).
        pub enable_endpoint_independent_mapping: pulumi_gestalt_rust::Output<bool>,
        /// Specifies the endpoint Types supported by the NAT Gateway.
        /// Supported values include:
        /// `ENDPOINT_TYPE_VM`, `ENDPOINT_TYPE_SWG`,
        /// `ENDPOINT_TYPE_MANAGED_PROXY_LB`.
        pub endpoint_types: pulumi_gestalt_rust::Output<Vec<String>>,
        /// Timeout (in seconds) for ICMP connections. Defaults to 30s if not set.
        pub icmp_idle_timeout_sec: pulumi_gestalt_rust::Output<Option<i32>>,
        /// Self-links of NAT IPs to be used as initial value for creation alongside a RouterNatAddress resource.
        /// Conflicts with natIps and drainNatIps. Only valid if natIpAllocateOption is set to MANUAL_ONLY.
        pub initial_nat_ips: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// Configuration for logging on NAT
        /// Structure is documented below.
        pub log_config: pulumi_gestalt_rust::Output<
            Option<super::super::types::compute::RouterNatLogConfig>,
        >,
        /// Maximum number of ports allocated to a VM from this NAT.
        /// This field can only be set when enableDynamicPortAllocation is enabled.
        pub max_ports_per_vm: pulumi_gestalt_rust::Output<Option<i32>>,
        /// Minimum number of ports allocated to a VM from this NAT. Defaults to 64 for static port allocation and 32 dynamic port allocation if not set.
        pub min_ports_per_vm: pulumi_gestalt_rust::Output<i32>,
        /// Name of the NAT service. The name must be 1-63 characters long and
        /// comply with RFC1035.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// How external IPs should be allocated for this NAT. Valid values are
        /// `AUTO_ONLY` for only allowing NAT IPs allocated by Google Cloud
        /// Platform, or `MANUAL_ONLY` for only user-allocated NAT IP addresses.
        /// Possible values are: `MANUAL_ONLY`, `AUTO_ONLY`.
        pub nat_ip_allocate_option: pulumi_gestalt_rust::Output<Option<String>>,
        /// Self-links of NAT IPs. Only valid if natIpAllocateOption
        /// is set to MANUAL_ONLY.
        /// If this field is used alongside with a count created list of address resources `google_compute_address.foobar.*.self_link`,
        /// the access level resource for the address resource must have a `lifecycle` block with `create_before_destroy = true` so
        /// the number of resources can be increased/decreased without triggering the `resourceInUseByAnotherResource` error.
        pub nat_ips: pulumi_gestalt_rust::Output<Vec<String>>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_gestalt_rust::Output<String>,
        /// Region where the router and NAT reside.
        pub region: pulumi_gestalt_rust::Output<String>,
        /// The name of the Cloud Router in which this NAT will be configured.
        ///
        ///
        /// - - -
        pub router: pulumi_gestalt_rust::Output<String>,
        /// A list of rules associated with this NAT.
        /// Structure is documented below.
        pub rules: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::compute::RouterNatRule>>,
        >,
        /// How NAT should be configured per Subnetwork.
        /// If `ALL_SUBNETWORKS_ALL_IP_RANGES`, all of the
        /// IP ranges in every Subnetwork are allowed to Nat.
        /// If `ALL_SUBNETWORKS_ALL_PRIMARY_IP_RANGES`, all of the primary IP
        /// ranges in every Subnetwork are allowed to Nat.
        /// `LIST_OF_SUBNETWORKS`: A list of Subnetworks are allowed to Nat
        /// (specified in the field subnetwork below). Note that if this field
        /// contains ALL_SUBNETWORKS_ALL_IP_RANGES or
        /// ALL_SUBNETWORKS_ALL_PRIMARY_IP_RANGES, then there should not be any
        /// other RouterNat section in any Router for this network in this region.
        /// Possible values are: `ALL_SUBNETWORKS_ALL_IP_RANGES`, `ALL_SUBNETWORKS_ALL_PRIMARY_IP_RANGES`, `LIST_OF_SUBNETWORKS`.
        pub source_subnetwork_ip_ranges_to_nat: pulumi_gestalt_rust::Output<String>,
        /// One or more subnetwork NAT configurations. Only used if
        /// `source_subnetwork_ip_ranges_to_nat` is set to `LIST_OF_SUBNETWORKS`
        /// Structure is documented below.
        pub subnetworks: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::compute::RouterNatSubnetwork>>,
        >,
        /// Timeout (in seconds) for TCP established connections.
        /// Defaults to 1200s if not set.
        pub tcp_established_idle_timeout_sec: pulumi_gestalt_rust::Output<Option<i32>>,
        /// Timeout (in seconds) for TCP connections that are in TIME_WAIT state.
        /// Defaults to 120s if not set.
        pub tcp_time_wait_timeout_sec: pulumi_gestalt_rust::Output<Option<i32>>,
        /// Timeout (in seconds) for TCP transitory connections.
        /// Defaults to 30s if not set.
        pub tcp_transitory_idle_timeout_sec: pulumi_gestalt_rust::Output<Option<i32>>,
        /// Indicates whether this NAT is used for public or private IP translation.
        /// If unspecified, it defaults to PUBLIC.
        /// If `PUBLIC` NAT used for public IP translation.
        /// If `PRIVATE` NAT used for private IP translation.
        /// Default value is `PUBLIC`.
        /// Possible values are: `PUBLIC`, `PRIVATE`.
        pub type_: pulumi_gestalt_rust::Output<Option<String>>,
        /// Timeout (in seconds) for UDP connections. Defaults to 30s if not set.
        pub udp_idle_timeout_sec: pulumi_gestalt_rust::Output<Option<i32>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: RouterNatArgs,
    ) -> RouterNatResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let auto_network_tier_binding = args.auto_network_tier.get_output(context);
        let drain_nat_ips_binding = args.drain_nat_ips.get_output(context);
        let enable_dynamic_port_allocation_binding = args
            .enable_dynamic_port_allocation
            .get_output(context);
        let enable_endpoint_independent_mapping_binding = args
            .enable_endpoint_independent_mapping
            .get_output(context);
        let endpoint_types_binding = args.endpoint_types.get_output(context);
        let icmp_idle_timeout_sec_binding = args
            .icmp_idle_timeout_sec
            .get_output(context);
        let initial_nat_ips_binding = args.initial_nat_ips.get_output(context);
        let log_config_binding = args.log_config.get_output(context);
        let max_ports_per_vm_binding = args.max_ports_per_vm.get_output(context);
        let min_ports_per_vm_binding = args.min_ports_per_vm.get_output(context);
        let name_binding = args.name.get_output(context);
        let nat_ip_allocate_option_binding = args
            .nat_ip_allocate_option
            .get_output(context);
        let nat_ips_binding = args.nat_ips.get_output(context);
        let project_binding = args.project.get_output(context);
        let region_binding = args.region.get_output(context);
        let router_binding = args.router.get_output(context);
        let rules_binding = args.rules.get_output(context);
        let source_subnetwork_ip_ranges_to_nat_binding = args
            .source_subnetwork_ip_ranges_to_nat
            .get_output(context);
        let subnetworks_binding = args.subnetworks.get_output(context);
        let tcp_established_idle_timeout_sec_binding = args
            .tcp_established_idle_timeout_sec
            .get_output(context);
        let tcp_time_wait_timeout_sec_binding = args
            .tcp_time_wait_timeout_sec
            .get_output(context);
        let tcp_transitory_idle_timeout_sec_binding = args
            .tcp_transitory_idle_timeout_sec
            .get_output(context);
        let type__binding = args.type_.get_output(context);
        let udp_idle_timeout_sec_binding = args.udp_idle_timeout_sec.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:compute/routerNat:RouterNat".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "autoNetworkTier".into(),
                    value: auto_network_tier_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "drainNatIps".into(),
                    value: drain_nat_ips_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "enableDynamicPortAllocation".into(),
                    value: enable_dynamic_port_allocation_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "enableEndpointIndependentMapping".into(),
                    value: enable_endpoint_independent_mapping_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "endpointTypes".into(),
                    value: endpoint_types_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "icmpIdleTimeoutSec".into(),
                    value: icmp_idle_timeout_sec_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "initialNatIps".into(),
                    value: initial_nat_ips_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "logConfig".into(),
                    value: log_config_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "maxPortsPerVm".into(),
                    value: max_ports_per_vm_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "minPortsPerVm".into(),
                    value: min_ports_per_vm_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "natIpAllocateOption".into(),
                    value: nat_ip_allocate_option_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "natIps".into(),
                    value: nat_ips_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "project".into(),
                    value: project_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "region".into(),
                    value: region_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "router".into(),
                    value: router_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "rules".into(),
                    value: rules_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "sourceSubnetworkIpRangesToNat".into(),
                    value: source_subnetwork_ip_ranges_to_nat_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "subnetworks".into(),
                    value: subnetworks_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tcpEstablishedIdleTimeoutSec".into(),
                    value: tcp_established_idle_timeout_sec_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tcpTimeWaitTimeoutSec".into(),
                    value: tcp_time_wait_timeout_sec_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tcpTransitoryIdleTimeoutSec".into(),
                    value: tcp_transitory_idle_timeout_sec_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "type".into(),
                    value: type__binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "udpIdleTimeoutSec".into(),
                    value: udp_idle_timeout_sec_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        RouterNatResult {
            auto_network_tier: o.get_field("autoNetworkTier"),
            drain_nat_ips: o.get_field("drainNatIps"),
            enable_dynamic_port_allocation: o.get_field("enableDynamicPortAllocation"),
            enable_endpoint_independent_mapping: o
                .get_field("enableEndpointIndependentMapping"),
            endpoint_types: o.get_field("endpointTypes"),
            icmp_idle_timeout_sec: o.get_field("icmpIdleTimeoutSec"),
            initial_nat_ips: o.get_field("initialNatIps"),
            log_config: o.get_field("logConfig"),
            max_ports_per_vm: o.get_field("maxPortsPerVm"),
            min_ports_per_vm: o.get_field("minPortsPerVm"),
            name: o.get_field("name"),
            nat_ip_allocate_option: o.get_field("natIpAllocateOption"),
            nat_ips: o.get_field("natIps"),
            project: o.get_field("project"),
            region: o.get_field("region"),
            router: o.get_field("router"),
            rules: o.get_field("rules"),
            source_subnetwork_ip_ranges_to_nat: o
                .get_field("sourceSubnetworkIpRangesToNat"),
            subnetworks: o.get_field("subnetworks"),
            tcp_established_idle_timeout_sec: o
                .get_field("tcpEstablishedIdleTimeoutSec"),
            tcp_time_wait_timeout_sec: o.get_field("tcpTimeWaitTimeoutSec"),
            tcp_transitory_idle_timeout_sec: o.get_field("tcpTransitoryIdleTimeoutSec"),
            type_: o.get_field("type"),
            udp_idle_timeout_sec: o.get_field("udpIdleTimeoutSec"),
        }
    }
}
