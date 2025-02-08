/// A VPC network is a virtual version of the traditional physical networks
/// that exist within and between physical data centers. A VPC network
/// provides connectivity for your Compute Engine virtual machine (VM)
/// instances, Container Engine containers, App Engine Flex services, and
/// other network-related resources.
///
/// Each GCP project contains one or more VPC networks. Each VPC network is a
/// global entity spanning all GCP regions. This global VPC network allows VM
/// instances and other resources to communicate with each other via internal,
/// private IP addresses.
///
/// Each VPC network is subdivided into subnets, and each subnet is contained
/// within a single region. You can have more than one subnet in a region for
/// a given VPC network. Each subnet has a contiguous private RFC1918 IP
/// space. You create instances, containers, and the like in these subnets.
/// When you create an instance, you must create it in a subnet, and the
/// instance draws its internal IP address from that subnet.
///
/// Virtual machine (VM) instances in a VPC network can communicate with
/// instances in all other subnets of the same VPC network, regardless of
/// region, using their RFC1918 private IP addresses. You can isolate portions
/// of the network, even entire subnets, using firewall rules.
///
///
/// To get more information about Subnetwork, see:
///
/// * [API documentation](https://cloud.google.com/compute/docs/reference/rest/v1/subnetworks)
/// * How-to Guides
///     * [Cloud Networking](https://cloud.google.com/vpc/docs/using-vpc)
///     * [Private Google Access](https://cloud.google.com/vpc/docs/configure-private-google-access)
///
/// ## Example Usage
///
/// ### Subnetwork Basic
///
///
/// ```yaml
/// resources:
///   network-with-private-secondary-ip-ranges:
///     type: gcp:compute:Subnetwork
///     properties:
///       name: test-subnetwork
///       ipCidrRange: 10.2.0.0/16
///       region: us-central1
///       network: ${["custom-test"].id}
///       secondaryIpRanges:
///         - rangeName: tf-test-secondary-range-update1
///           ipCidrRange: 192.168.10.0/24
///   custom-test:
///     type: gcp:compute:Network
///     properties:
///       name: test-network
///       autoCreateSubnetworks: false
/// ```
/// ### Subnetwork Logging Config
///
///
/// ```yaml
/// resources:
///   subnet-with-logging:
///     type: gcp:compute:Subnetwork
///     properties:
///       name: log-test-subnetwork
///       ipCidrRange: 10.2.0.0/16
///       region: us-central1
///       network: ${["custom-test"].id}
///       logConfig:
///         aggregationInterval: INTERVAL_10_MIN
///         flowSampling: 0.5
///         metadata: INCLUDE_ALL_METADATA
///   custom-test:
///     type: gcp:compute:Network
///     properties:
///       name: log-test-network
///       autoCreateSubnetworks: false
/// ```
/// ### Subnetwork Internal L7lb
///
///
/// ```yaml
/// resources:
///   network-for-l7lb:
///     type: gcp:compute:Subnetwork
///     properties:
///       name: l7lb-test-subnetwork
///       ipCidrRange: 10.0.0.0/22
///       region: us-central1
///       purpose: REGIONAL_MANAGED_PROXY
///       role: ACTIVE
///       network: ${["custom-test"].id}
///   custom-test:
///     type: gcp:compute:Network
///     properties:
///       name: l7lb-test-network
///       autoCreateSubnetworks: false
/// ```
/// ### Subnetwork Ipv6
///
///
/// ```yaml
/// resources:
///   subnetwork-ipv6:
///     type: gcp:compute:Subnetwork
///     properties:
///       name: ipv6-test-subnetwork
///       ipCidrRange: 10.0.0.0/22
///       region: us-west2
///       stackType: IPV4_IPV6
///       ipv6AccessType: EXTERNAL
///       network: ${["custom-test"].id}
///   custom-test:
///     type: gcp:compute:Network
///     properties:
///       name: ipv6-test-network
///       autoCreateSubnetworks: false
/// ```
/// ### Subnetwork Internal Ipv6
///
///
/// ```yaml
/// resources:
///   subnetwork-internal-ipv6:
///     type: gcp:compute:Subnetwork
///     properties:
///       name: internal-ipv6-test-subnetwork
///       ipCidrRange: 10.0.0.0/22
///       region: us-west2
///       stackType: IPV4_IPV6
///       ipv6AccessType: INTERNAL
///       network: ${["custom-test"].id}
///   custom-test:
///     type: gcp:compute:Network
///     properties:
///       name: internal-ipv6-test-network
///       autoCreateSubnetworks: false
///       enableUlaInternalIpv6: true
/// ```
/// ### Subnetwork Purpose Private Nat
///
///
/// ```yaml
/// resources:
///   subnetwork-purpose-private-nat:
///     type: gcp:compute:Subnetwork
///     properties:
///       name: subnet-purpose-test-subnetwork
///       region: us-west2
///       ipCidrRange: 192.168.1.0/24
///       purpose: PRIVATE_NAT
///       network: ${["custom-test"].id}
///   custom-test:
///     type: gcp:compute:Network
///     properties:
///       name: subnet-purpose-test-network
///       autoCreateSubnetworks: false
/// ```
/// ### Subnetwork Cidr Overlap
///
///
/// ```yaml
/// resources:
///   subnetwork-cidr-overlap:
///     type: gcp:compute:Subnetwork
///     properties:
///       name: subnet-cidr-overlap
///       region: us-west2
///       ipCidrRange: 192.168.1.0/24
///       allowSubnetCidrRoutesOverlap: true
///       network: ${["net-cidr-overlap"].id}
///   net-cidr-overlap:
///     type: gcp:compute:Network
///     properties:
///       name: net-cidr-overlap
///       autoCreateSubnetworks: false
/// ```
/// ### Subnetwork Reserved Internal Range
///
///
/// ```yaml
/// resources:
///   subnetwork-reserved-internal-range:
///     type: gcp:compute:Subnetwork
///     properties:
///       name: subnetwork-reserved-internal-range
///       region: us-central1
///       network: ${default.id}
///       reservedInternalRange: networkconnectivity.googleapis.com/${reserved.id}
///   default:
///     type: gcp:compute:Network
///     properties:
///       name: network-reserved-internal-range
///       autoCreateSubnetworks: false
///   reserved:
///     type: gcp:networkconnectivity:InternalRange
///     properties:
///       name: reserved
///       network: ${default.id}
///       usage: FOR_VPC
///       peering: FOR_SELF
///       prefixLength: 24
///       targetCidrRanges:
///         - 10.0.0.0/8
/// ```
/// ### Subnetwork Reserved Secondary Range
///
///
/// ```yaml
/// resources:
///   subnetwork-reserved-secondary-range:
///     type: gcp:compute:Subnetwork
///     properties:
///       name: subnetwork-reserved-secondary-range
///       region: us-central1
///       network: ${default.id}
///       reservedInternalRange: networkconnectivity.googleapis.com/${reserved.id}
///       secondaryIpRanges:
///         - rangeName: secondary
///           reservedInternalRange: networkconnectivity.googleapis.com/${reservedSecondary.id}
///   default:
///     type: gcp:compute:Network
///     properties:
///       name: network-reserved-secondary-range
///       autoCreateSubnetworks: false
///   reserved:
///     type: gcp:networkconnectivity:InternalRange
///     properties:
///       name: reserved-primary
///       network: ${default.id}
///       usage: FOR_VPC
///       peering: FOR_SELF
///       prefixLength: 24
///       targetCidrRanges:
///         - 10.0.0.0/8
///   reservedSecondary:
///     type: gcp:networkconnectivity:InternalRange
///     name: reserved_secondary
///     properties:
///       name: reserved-secondary
///       network: ${default.id}
///       usage: FOR_VPC
///       peering: FOR_SELF
///       prefixLength: 16
///       targetCidrRanges:
///         - 10.0.0.0/8
/// ```
///
/// ## Import
///
/// Subnetwork can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/regions/{{region}}/subnetworks/{{name}}`
///
/// * `{{project}}/{{region}}/{{name}}`
///
/// * `{{region}}/{{name}}`
///
/// * `{{name}}`
///
/// When using the `pulumi import` command, Subnetwork can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:compute/subnetwork:Subnetwork default projects/{{project}}/regions/{{region}}/subnetworks/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:compute/subnetwork:Subnetwork default {{project}}/{{region}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:compute/subnetwork:Subnetwork default {{region}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:compute/subnetwork:Subnetwork default {{name}}
/// ```
///
#[allow(clippy::doc_lazy_continuation)]
pub mod subnetwork {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct SubnetworkArgs {
        /// Typically packets destined to IPs within the subnetwork range that do not match
        /// existing resources are dropped and prevented from leaving the VPC.
        /// Setting this field to true will allow these packets to match dynamic routes injected
        /// via BGP even if their destinations match existing subnet ranges.
        #[builder(into, default)]
        pub allow_subnet_cidr_routes_overlap: pulumi_gestalt_rust::InputOrOutput<
            Option<bool>,
        >,
        /// An optional description of this resource. Provide this property when
        /// you create the resource. This field can be set only at resource
        /// creation time.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The range of external IPv6 addresses that are owned by this subnetwork.
        #[builder(into, default)]
        pub external_ipv6_prefix: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The range of internal addresses that are owned by this subnetwork.
        /// Provide this property when you create the subnetwork. For example,
        /// 10.0.0.0/8 or 192.168.0.0/16. Ranges must be unique and
        /// non-overlapping within a network. Only IPv4 is supported.
        /// Field is optional when `reserved_internal_range` is defined, otherwise required.
        #[builder(into, default)]
        pub ip_cidr_range: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The access type of IPv6 address this subnet holds. It's immutable and can only be specified during creation
        /// or the first time the subnet is updated into IPV4_IPV6 dual stack. If the ipv6_type is EXTERNAL then this subnet
        /// cannot enable direct path.
        /// Possible values are: `EXTERNAL`, `INTERNAL`.
        #[builder(into, default)]
        pub ipv6_access_type: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// This field denotes the VPC flow logging options for this subnetwork. If
        /// logging is enabled, logs are exported to Cloud Logging. Flow logging
        /// isn't supported if the subnet `purpose` field is set to subnetwork is
        /// `REGIONAL_MANAGED_PROXY` or `GLOBAL_MANAGED_PROXY`.
        /// Structure is documented below.
        #[builder(into, default)]
        pub log_config: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::compute::SubnetworkLogConfig>,
        >,
        /// The name of the resource, provided by the client when initially
        /// creating the resource. The name must be 1-63 characters long, and
        /// comply with RFC1035. Specifically, the name must be 1-63 characters
        /// long and match the regular expression `a-z?` which
        /// means the first character must be a lowercase letter, and all
        /// following characters must be a dash, lowercase letter, or digit,
        /// except the last character, which cannot be a dash.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The network this subnet belongs to.
        /// Only networks that are in the distributed mode can have subnetworks.
        ///
        ///
        /// - - -
        #[builder(into)]
        pub network: pulumi_gestalt_rust::InputOrOutput<String>,
        /// When enabled, VMs in this subnetwork without external IP addresses can
        /// access Google APIs and services by using Private Google Access.
        #[builder(into, default)]
        pub private_ip_google_access: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// The private IPv6 google access type for the VMs in this subnet.
        #[builder(into, default)]
        pub private_ipv6_google_access: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The purpose of the resource. This field can be either `PRIVATE`, `REGIONAL_MANAGED_PROXY`, `GLOBAL_MANAGED_PROXY`, `PRIVATE_SERVICE_CONNECT` or `PRIVATE_NAT`.
        /// A subnet with purpose set to `REGIONAL_MANAGED_PROXY` is a user-created subnetwork that is reserved for regional Envoy-based load balancers.
        /// A subnetwork in a given region with purpose set to `GLOBAL_MANAGED_PROXY` is a proxy-only subnet and is shared between all the cross-regional Envoy-based load balancers.
        /// A subnetwork with purpose set to `PRIVATE_SERVICE_CONNECT` reserves the subnet for hosting a Private Service Connect published service.
        /// A subnetwork with purpose set to `PRIVATE_NAT` is used as source range for Private NAT gateways.
        /// Note that `REGIONAL_MANAGED_PROXY` is the preferred setting for all regional Envoy load balancers.
        /// If unspecified, the purpose defaults to `PRIVATE`.
        #[builder(into, default)]
        pub purpose: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The GCP region for this subnetwork.
        #[builder(into, default)]
        pub region: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ID of the reserved internal range. Must be prefixed with `networkconnectivity.googleapis.com`
        /// E.g. `networkconnectivity.googleapis.com/projects/{project}/locations/global/internalRanges/{rangeId}`
        #[builder(into, default)]
        pub reserved_internal_range: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The role of subnetwork.
        /// Currently, this field is only used when `purpose` is `REGIONAL_MANAGED_PROXY`.
        /// The value can be set to `ACTIVE` or `BACKUP`.
        /// An `ACTIVE` subnetwork is one that is currently being used for Envoy-based load balancers in a region.
        /// A `BACKUP` subnetwork is one that is ready to be promoted to `ACTIVE` or is currently draining.
        /// Possible values are: `ACTIVE`, `BACKUP`.
        #[builder(into, default)]
        pub role: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// An array of configurations for secondary IP ranges for VM instances
        /// contained in this subnetwork. The primary IP of such VM must belong
        /// to the primary ipCidrRange of the subnetwork. The alias IPs may belong
        /// to either primary or secondary ranges.
        /// Structure is documented below.
        #[builder(into, default)]
        pub secondary_ip_ranges: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::compute::SubnetworkSecondaryIpRange>>,
        >,
        /// Controls the removal behavior of secondary_ip_range.
        /// When false, removing secondary_ip_range from config will not produce a diff as
        /// the provider will default to the API's value.
        /// When true, the provider will treat removing secondary_ip_range as sending an
        /// empty list of secondary IP ranges to the API.
        /// Defaults to false.
        #[builder(into, default)]
        pub send_secondary_ip_range_if_empty: pulumi_gestalt_rust::InputOrOutput<
            Option<bool>,
        >,
        /// The stack type for this subnet to identify whether the IPv6 feature is enabled or not.
        /// If not specified IPV4_ONLY will be used.
        /// Possible values are: `IPV4_ONLY`, `IPV4_IPV6`.
        #[builder(into, default)]
        pub stack_type: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct SubnetworkResult {
        /// Typically packets destined to IPs within the subnetwork range that do not match
        /// existing resources are dropped and prevented from leaving the VPC.
        /// Setting this field to true will allow these packets to match dynamic routes injected
        /// via BGP even if their destinations match existing subnet ranges.
        pub allow_subnet_cidr_routes_overlap: pulumi_gestalt_rust::Output<bool>,
        /// Creation timestamp in RFC3339 text format.
        pub creation_timestamp: pulumi_gestalt_rust::Output<String>,
        /// An optional description of this resource. Provide this property when
        /// you create the resource. This field can be set only at resource
        /// creation time.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// The range of external IPv6 addresses that are owned by this subnetwork.
        pub external_ipv6_prefix: pulumi_gestalt_rust::Output<String>,
        /// Fingerprint of this resource. This field is used internally during updates of this resource.
        pub fingerprint: pulumi_gestalt_rust::Output<String>,
        /// The gateway address for default routes to reach destination addresses
        /// outside this subnetwork.
        pub gateway_address: pulumi_gestalt_rust::Output<String>,
        /// The internal IPv6 address range that is assigned to this subnetwork.
        pub internal_ipv6_prefix: pulumi_gestalt_rust::Output<String>,
        /// The range of internal addresses that are owned by this subnetwork.
        /// Provide this property when you create the subnetwork. For example,
        /// 10.0.0.0/8 or 192.168.0.0/16. Ranges must be unique and
        /// non-overlapping within a network. Only IPv4 is supported.
        /// Field is optional when `reserved_internal_range` is defined, otherwise required.
        pub ip_cidr_range: pulumi_gestalt_rust::Output<String>,
        /// The access type of IPv6 address this subnet holds. It's immutable and can only be specified during creation
        /// or the first time the subnet is updated into IPV4_IPV6 dual stack. If the ipv6_type is EXTERNAL then this subnet
        /// cannot enable direct path.
        /// Possible values are: `EXTERNAL`, `INTERNAL`.
        pub ipv6_access_type: pulumi_gestalt_rust::Output<Option<String>>,
        /// The range of internal IPv6 addresses that are owned by this subnetwork.
        pub ipv6_cidr_range: pulumi_gestalt_rust::Output<String>,
        /// This field denotes the VPC flow logging options for this subnetwork. If
        /// logging is enabled, logs are exported to Cloud Logging. Flow logging
        /// isn't supported if the subnet `purpose` field is set to subnetwork is
        /// `REGIONAL_MANAGED_PROXY` or `GLOBAL_MANAGED_PROXY`.
        /// Structure is documented below.
        pub log_config: pulumi_gestalt_rust::Output<
            Option<super::super::types::compute::SubnetworkLogConfig>,
        >,
        /// The name of the resource, provided by the client when initially
        /// creating the resource. The name must be 1-63 characters long, and
        /// comply with RFC1035. Specifically, the name must be 1-63 characters
        /// long and match the regular expression `a-z?` which
        /// means the first character must be a lowercase letter, and all
        /// following characters must be a dash, lowercase letter, or digit,
        /// except the last character, which cannot be a dash.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The network this subnet belongs to.
        /// Only networks that are in the distributed mode can have subnetworks.
        ///
        ///
        /// - - -
        pub network: pulumi_gestalt_rust::Output<String>,
        /// When enabled, VMs in this subnetwork without external IP addresses can
        /// access Google APIs and services by using Private Google Access.
        pub private_ip_google_access: pulumi_gestalt_rust::Output<bool>,
        /// The private IPv6 google access type for the VMs in this subnet.
        pub private_ipv6_google_access: pulumi_gestalt_rust::Output<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_gestalt_rust::Output<String>,
        /// The purpose of the resource. This field can be either `PRIVATE`, `REGIONAL_MANAGED_PROXY`, `GLOBAL_MANAGED_PROXY`, `PRIVATE_SERVICE_CONNECT` or `PRIVATE_NAT`.
        /// A subnet with purpose set to `REGIONAL_MANAGED_PROXY` is a user-created subnetwork that is reserved for regional Envoy-based load balancers.
        /// A subnetwork in a given region with purpose set to `GLOBAL_MANAGED_PROXY` is a proxy-only subnet and is shared between all the cross-regional Envoy-based load balancers.
        /// A subnetwork with purpose set to `PRIVATE_SERVICE_CONNECT` reserves the subnet for hosting a Private Service Connect published service.
        /// A subnetwork with purpose set to `PRIVATE_NAT` is used as source range for Private NAT gateways.
        /// Note that `REGIONAL_MANAGED_PROXY` is the preferred setting for all regional Envoy load balancers.
        /// If unspecified, the purpose defaults to `PRIVATE`.
        pub purpose: pulumi_gestalt_rust::Output<String>,
        /// The GCP region for this subnetwork.
        pub region: pulumi_gestalt_rust::Output<String>,
        /// The ID of the reserved internal range. Must be prefixed with `networkconnectivity.googleapis.com`
        /// E.g. `networkconnectivity.googleapis.com/projects/{project}/locations/global/internalRanges/{rangeId}`
        pub reserved_internal_range: pulumi_gestalt_rust::Output<Option<String>>,
        /// The role of subnetwork.
        /// Currently, this field is only used when `purpose` is `REGIONAL_MANAGED_PROXY`.
        /// The value can be set to `ACTIVE` or `BACKUP`.
        /// An `ACTIVE` subnetwork is one that is currently being used for Envoy-based load balancers in a region.
        /// A `BACKUP` subnetwork is one that is ready to be promoted to `ACTIVE` or is currently draining.
        /// Possible values are: `ACTIVE`, `BACKUP`.
        pub role: pulumi_gestalt_rust::Output<Option<String>>,
        /// An array of configurations for secondary IP ranges for VM instances
        /// contained in this subnetwork. The primary IP of such VM must belong
        /// to the primary ipCidrRange of the subnetwork. The alias IPs may belong
        /// to either primary or secondary ranges.
        /// Structure is documented below.
        pub secondary_ip_ranges: pulumi_gestalt_rust::Output<
            Vec<super::super::types::compute::SubnetworkSecondaryIpRange>,
        >,
        /// The URI of the created resource.
        pub self_link: pulumi_gestalt_rust::Output<String>,
        /// Controls the removal behavior of secondary_ip_range.
        /// When false, removing secondary_ip_range from config will not produce a diff as
        /// the provider will default to the API's value.
        /// When true, the provider will treat removing secondary_ip_range as sending an
        /// empty list of secondary IP ranges to the API.
        /// Defaults to false.
        pub send_secondary_ip_range_if_empty: pulumi_gestalt_rust::Output<Option<bool>>,
        /// The stack type for this subnet to identify whether the IPv6 feature is enabled or not.
        /// If not specified IPV4_ONLY will be used.
        /// Possible values are: `IPV4_ONLY`, `IPV4_IPV6`.
        pub stack_type: pulumi_gestalt_rust::Output<String>,
        /// The unique identifier number for the resource. This identifier is defined by the server.
        pub subnetwork_id: pulumi_gestalt_rust::Output<i32>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: SubnetworkArgs,
    ) -> SubnetworkResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let allow_subnet_cidr_routes_overlap_binding = args
            .allow_subnet_cidr_routes_overlap
            .get_output(context)
            .get_inner();
        let description_binding = args.description.get_output(context).get_inner();
        let external_ipv6_prefix_binding = args
            .external_ipv6_prefix
            .get_output(context)
            .get_inner();
        let ip_cidr_range_binding = args.ip_cidr_range.get_output(context).get_inner();
        let ipv6_access_type_binding = args
            .ipv6_access_type
            .get_output(context)
            .get_inner();
        let log_config_binding = args.log_config.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let network_binding = args.network.get_output(context).get_inner();
        let private_ip_google_access_binding = args
            .private_ip_google_access
            .get_output(context)
            .get_inner();
        let private_ipv6_google_access_binding = args
            .private_ipv6_google_access
            .get_output(context)
            .get_inner();
        let project_binding = args.project.get_output(context).get_inner();
        let purpose_binding = args.purpose.get_output(context).get_inner();
        let region_binding = args.region.get_output(context).get_inner();
        let reserved_internal_range_binding = args
            .reserved_internal_range
            .get_output(context)
            .get_inner();
        let role_binding = args.role.get_output(context).get_inner();
        let secondary_ip_ranges_binding = args
            .secondary_ip_ranges
            .get_output(context)
            .get_inner();
        let send_secondary_ip_range_if_empty_binding = args
            .send_secondary_ip_range_if_empty
            .get_output(context)
            .get_inner();
        let stack_type_binding = args.stack_type.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:compute/subnetwork:Subnetwork".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "allowSubnetCidrRoutesOverlap".into(),
                    value: &allow_subnet_cidr_routes_overlap_binding,
                },
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "externalIpv6Prefix".into(),
                    value: &external_ipv6_prefix_binding,
                },
                register_interface::ObjectField {
                    name: "ipCidrRange".into(),
                    value: &ip_cidr_range_binding,
                },
                register_interface::ObjectField {
                    name: "ipv6AccessType".into(),
                    value: &ipv6_access_type_binding,
                },
                register_interface::ObjectField {
                    name: "logConfig".into(),
                    value: &log_config_binding,
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
                    name: "privateIpGoogleAccess".into(),
                    value: &private_ip_google_access_binding,
                },
                register_interface::ObjectField {
                    name: "privateIpv6GoogleAccess".into(),
                    value: &private_ipv6_google_access_binding,
                },
                register_interface::ObjectField {
                    name: "project".into(),
                    value: &project_binding,
                },
                register_interface::ObjectField {
                    name: "purpose".into(),
                    value: &purpose_binding,
                },
                register_interface::ObjectField {
                    name: "region".into(),
                    value: &region_binding,
                },
                register_interface::ObjectField {
                    name: "reservedInternalRange".into(),
                    value: &reserved_internal_range_binding,
                },
                register_interface::ObjectField {
                    name: "role".into(),
                    value: &role_binding,
                },
                register_interface::ObjectField {
                    name: "secondaryIpRanges".into(),
                    value: &secondary_ip_ranges_binding,
                },
                register_interface::ObjectField {
                    name: "sendSecondaryIpRangeIfEmpty".into(),
                    value: &send_secondary_ip_range_if_empty_binding,
                },
                register_interface::ObjectField {
                    name: "stackType".into(),
                    value: &stack_type_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        SubnetworkResult {
            allow_subnet_cidr_routes_overlap: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("allowSubnetCidrRoutesOverlap"),
            ),
            creation_timestamp: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("creationTimestamp"),
            ),
            description: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            external_ipv6_prefix: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("externalIpv6Prefix"),
            ),
            fingerprint: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("fingerprint"),
            ),
            gateway_address: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("gatewayAddress"),
            ),
            internal_ipv6_prefix: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("internalIpv6Prefix"),
            ),
            ip_cidr_range: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("ipCidrRange"),
            ),
            ipv6_access_type: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("ipv6AccessType"),
            ),
            ipv6_cidr_range: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("ipv6CidrRange"),
            ),
            log_config: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("logConfig"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            network: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("network"),
            ),
            private_ip_google_access: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("privateIpGoogleAccess"),
            ),
            private_ipv6_google_access: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("privateIpv6GoogleAccess"),
            ),
            project: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("project"),
            ),
            purpose: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("purpose"),
            ),
            region: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("region"),
            ),
            reserved_internal_range: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("reservedInternalRange"),
            ),
            role: pulumi_gestalt_rust::__private::into_domain(o.extract_field("role")),
            secondary_ip_ranges: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("secondaryIpRanges"),
            ),
            self_link: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("selfLink"),
            ),
            send_secondary_ip_range_if_empty: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("sendSecondaryIpRangeIfEmpty"),
            ),
            stack_type: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("stackType"),
            ),
            subnetwork_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("subnetworkId"),
            ),
        }
    }
}
