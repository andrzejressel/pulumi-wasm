/// The internal range resource for IPAM operations within a VPC network. Used to represent a private address range along with behavioral characterstics of that range (its usage and peering behavior). Networking resources can link to this range if they are created as belonging to it.
///
///
/// To get more information about InternalRange, see:
///
/// * [API documentation](https://cloud.google.com/network-connectivity/docs/reference/networkconnectivity/rest/v1/projects.locations.internalRanges)
/// * How-to Guides
///     * [Use internal ranges](https://cloud.google.com/vpc/docs/create-use-internal-ranges)
///
/// ## Example Usage
///
/// ### Network Connectivity Internal Ranges Basic
///
///
/// ```yaml
/// resources:
///   default:
///     type: gcp:networkconnectivity:InternalRange
///     properties:
///       name: basic
///       description: Test internal range
///       network: ${defaultNetwork.selfLink}
///       usage: FOR_VPC
///       peering: FOR_SELF
///       ipCidrRange: 10.0.0.0/24
///       labels:
///         label-a: b
///   defaultNetwork:
///     type: gcp:compute:Network
///     name: default
///     properties:
///       name: internal-ranges
///       autoCreateSubnetworks: false
/// ```
/// ### Network Connectivity Internal Ranges Automatic Reservation
///
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let default = internal_range::create(
///         "default",
///         InternalRangeArgs::builder()
///             .name("automatic-reservation")
///             .network("${defaultNetwork.id}")
///             .peering("FOR_SELF")
///             .prefix_length(24)
///             .target_cidr_ranges(vec!["192.16.0.0/16",])
///             .usage("FOR_VPC")
///             .build_struct(),
///     );
///     let defaultNetwork = network::create(
///         "defaultNetwork",
///         NetworkArgs::builder()
///             .auto_create_subnetworks(false)
///             .name("internal-ranges")
///             .build_struct(),
///     );
/// }
/// ```
/// ### Network Connectivity Internal Ranges External Ranges
///
///
/// ```yaml
/// resources:
///   default:
///     type: gcp:networkconnectivity:InternalRange
///     properties:
///       name: external-ranges
///       network: ${defaultNetwork.id}
///       usage: EXTERNAL_TO_VPC
///       peering: FOR_SELF
///       ipCidrRange: 172.16.0.0/24
///       labels:
///         external-reserved-range: on-premises
///   defaultNetwork:
///     type: gcp:compute:Network
///     name: default
///     properties:
///       name: internal-ranges
///       autoCreateSubnetworks: false
/// ```
/// ### Network Connectivity Internal Ranges Reserve With Overlap
///
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let default = internal_range::create(
///         "default",
///         InternalRangeArgs::builder()
///             .description("Test internal range")
///             .ip_cidr_range("10.0.0.0/30")
///             .name("overlap-range")
///             .network("${defaultNetwork.id}")
///             .overlaps(vec!["OVERLAP_EXISTING_SUBNET_RANGE",])
///             .peering("FOR_SELF")
///             .usage("FOR_VPC")
///             .build_struct(),
///     );
///     let defaultNetwork = network::create(
///         "defaultNetwork",
///         NetworkArgs::builder()
///             .auto_create_subnetworks(false)
///             .name("internal-ranges")
///             .build_struct(),
///     );
///     let defaultSubnetwork = subnetwork::create(
///         "defaultSubnetwork",
///         SubnetworkArgs::builder()
///             .ip_cidr_range("10.0.0.0/24")
///             .name("overlapping-subnet")
///             .network("${defaultNetwork.id}")
///             .region("us-central1")
///             .build_struct(),
///     );
/// }
/// ```
/// ### Network Connectivity Internal Ranges Migration
///
///
/// ```yaml
/// resources:
///   default:
///     type: gcp:networkconnectivity:InternalRange
///     properties:
///       name: migration
///       description: Test internal range
///       network: ${defaultNetwork.selfLink}
///       usage: FOR_MIGRATION
///       peering: FOR_SELF
///       ipCidrRange: 10.1.0.0/16
///       migration:
///         source: ${source.selfLink}
///         target: projects/${targetProject.projectId}/regions/us-central1/subnetworks/target-subnet
///   defaultNetwork:
///     type: gcp:compute:Network
///     name: default
///     properties:
///       name: internal-ranges
///       autoCreateSubnetworks: false
///   source:
///     type: gcp:compute:Subnetwork
///     properties:
///       name: source-subnet
///       ipCidrRange: 10.1.0.0/16
///       region: us-central1
///       network: ${defaultNetwork.name}
/// variables:
///   targetProject:
///     fn::invoke:
///       function: gcp:organizations:getProject
///       arguments: {}
/// ```
///
/// ## Import
///
/// InternalRange can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/locations/global/internalRanges/{{name}}`
///
/// * `{{project}}/{{name}}`
///
/// * `{{name}}`
///
/// When using the `pulumi import` command, InternalRange can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:networkconnectivity/internalRange:InternalRange default projects/{{project}}/locations/global/internalRanges/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:networkconnectivity/internalRange:InternalRange default {{project}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:networkconnectivity/internalRange:InternalRange default {{name}}
/// ```
///
#[allow(clippy::doc_lazy_continuation)]
pub mod internal_range {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct InternalRangeArgs {
        /// An optional description of this resource.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The IP range that this internal range defines.
        #[builder(into, default)]
        pub ip_cidr_range: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// User-defined labels.
        ///
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field `effective_labels` for all of the labels present on the resource.
        #[builder(into, default)]
        pub labels: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Specification for migration with source and target resource names.
        /// Structure is documented below.
        #[builder(into, default)]
        pub migration: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::networkconnectivity::InternalRangeMigration>,
        >,
        /// The name of the policy based route.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Fully-qualified URL of the network that this route applies to, for example: projects/my-project/global/networks/my-network.
        #[builder(into)]
        pub network: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Optional. Types of resources that are allowed to overlap with the current internal range.
        /// Each value may be one of: `OVERLAP_ROUTE_RANGE`, `OVERLAP_EXISTING_SUBNET_RANGE`.
        #[builder(into, default)]
        pub overlaps: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// The type of peering set for this internal range.
        /// Possible values are: `FOR_SELF`, `FOR_PEER`, `NOT_SHARED`.
        ///
        ///
        /// - - -
        #[builder(into)]
        pub peering: pulumi_gestalt_rust::InputOrOutput<String>,
        /// An alternate to ipCidrRange. Can be set when trying to create a reservation that automatically finds a free range of the given size.
        /// If both ipCidrRange and prefixLength are set, there is an error if the range sizes do not match. Can also be used during updates to change the range size.
        #[builder(into, default)]
        pub prefix_length: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Optional. Can be set to narrow down or pick a different address space while searching for a free range.
        /// If not set, defaults to the "10.0.0.0/8" address space. This can be used to search in other rfc-1918 address spaces like "172.16.0.0/12" and "192.168.0.0/16" or non-rfc-1918 address spaces used in the VPC.
        #[builder(into, default)]
        pub target_cidr_ranges: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// The type of usage set for this InternalRange.
        /// Possible values are: `FOR_VPC`, `EXTERNAL_TO_VPC`, `FOR_MIGRATION`.
        #[builder(into)]
        pub usage: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct InternalRangeResult {
        /// An optional description of this resource.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// All of labels (key/value pairs) present on the resource in GCP, including the labels configured through Pulumi, other clients and services.
        pub effective_labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The IP range that this internal range defines.
        pub ip_cidr_range: pulumi_gestalt_rust::Output<String>,
        /// User-defined labels.
        ///
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field `effective_labels` for all of the labels present on the resource.
        pub labels: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Specification for migration with source and target resource names.
        /// Structure is documented below.
        pub migration: pulumi_gestalt_rust::Output<
            Option<super::super::types::networkconnectivity::InternalRangeMigration>,
        >,
        /// The name of the policy based route.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Fully-qualified URL of the network that this route applies to, for example: projects/my-project/global/networks/my-network.
        pub network: pulumi_gestalt_rust::Output<String>,
        /// Optional. Types of resources that are allowed to overlap with the current internal range.
        /// Each value may be one of: `OVERLAP_ROUTE_RANGE`, `OVERLAP_EXISTING_SUBNET_RANGE`.
        pub overlaps: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// The type of peering set for this internal range.
        /// Possible values are: `FOR_SELF`, `FOR_PEER`, `NOT_SHARED`.
        ///
        ///
        /// - - -
        pub peering: pulumi_gestalt_rust::Output<String>,
        /// An alternate to ipCidrRange. Can be set when trying to create a reservation that automatically finds a free range of the given size.
        /// If both ipCidrRange and prefixLength are set, there is an error if the range sizes do not match. Can also be used during updates to change the range size.
        pub prefix_length: pulumi_gestalt_rust::Output<Option<i32>>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_gestalt_rust::Output<String>,
        /// The combination of labels configured directly on the resource
        /// and default labels configured on the provider.
        pub pulumi_labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Optional. Can be set to narrow down or pick a different address space while searching for a free range.
        /// If not set, defaults to the "10.0.0.0/8" address space. This can be used to search in other rfc-1918 address spaces like "172.16.0.0/12" and "192.168.0.0/16" or non-rfc-1918 address spaces used in the VPC.
        pub target_cidr_ranges: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// The type of usage set for this InternalRange.
        /// Possible values are: `FOR_VPC`, `EXTERNAL_TO_VPC`, `FOR_MIGRATION`.
        pub usage: pulumi_gestalt_rust::Output<String>,
        /// Output only. The list of resources that refer to this internal range.
        /// Resources that use the internal range for their range allocation are referred to as users of the range.
        /// Other resources mark themselves as users while doing so by creating a reference to this internal range. Having a user, based on this reference, prevents deletion of the internal range referred to. Can be empty.
        pub users: pulumi_gestalt_rust::Output<Vec<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: InternalRangeArgs,
    ) -> InternalRangeResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let description_binding = args.description.get_output(context).get_inner();
        let ip_cidr_range_binding = args.ip_cidr_range.get_output(context).get_inner();
        let labels_binding = args.labels.get_output(context).get_inner();
        let migration_binding = args.migration.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let network_binding = args.network.get_output(context).get_inner();
        let overlaps_binding = args.overlaps.get_output(context).get_inner();
        let peering_binding = args.peering.get_output(context).get_inner();
        let prefix_length_binding = args.prefix_length.get_output(context).get_inner();
        let project_binding = args.project.get_output(context).get_inner();
        let target_cidr_ranges_binding = args
            .target_cidr_ranges
            .get_output(context)
            .get_inner();
        let usage_binding = args.usage.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:networkconnectivity/internalRange:InternalRange".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "ipCidrRange".into(),
                    value: &ip_cidr_range_binding,
                },
                register_interface::ObjectField {
                    name: "labels".into(),
                    value: &labels_binding,
                },
                register_interface::ObjectField {
                    name: "migration".into(),
                    value: &migration_binding,
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
                    name: "overlaps".into(),
                    value: &overlaps_binding,
                },
                register_interface::ObjectField {
                    name: "peering".into(),
                    value: &peering_binding,
                },
                register_interface::ObjectField {
                    name: "prefixLength".into(),
                    value: &prefix_length_binding,
                },
                register_interface::ObjectField {
                    name: "project".into(),
                    value: &project_binding,
                },
                register_interface::ObjectField {
                    name: "targetCidrRanges".into(),
                    value: &target_cidr_ranges_binding,
                },
                register_interface::ObjectField {
                    name: "usage".into(),
                    value: &usage_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        InternalRangeResult {
            description: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            effective_labels: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("effectiveLabels"),
            ),
            ip_cidr_range: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("ipCidrRange"),
            ),
            labels: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("labels"),
            ),
            migration: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("migration"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            network: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("network"),
            ),
            overlaps: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("overlaps"),
            ),
            peering: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("peering"),
            ),
            prefix_length: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("prefixLength"),
            ),
            project: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("project"),
            ),
            pulumi_labels: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("pulumiLabels"),
            ),
            target_cidr_ranges: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("targetCidrRanges"),
            ),
            usage: pulumi_gestalt_rust::__private::into_domain(o.extract_field("usage")),
            users: pulumi_gestalt_rust::__private::into_domain(o.extract_field("users")),
        }
    }
}
