/// A Distributed Cloud Edge subnet, which provides L2 isolation within a network.
///
///
/// To get more information about Subnet, see:
///
/// * [API documentation](https://cloud.google.com/distributed-cloud/edge/latest/docs/reference/network/rest/v1/projects.locations.zones.subnets)
/// * How-to Guides
///     * [Create and manage subnetworks](https://cloud.google.com/distributed-cloud/edge/latest/docs/subnetworks#api)
///
/// ## Example Usage
///
/// ### Edgenetwork Subnet
///
///
/// ```yaml
/// resources:
///   exampleSubnet:
///     type: gcp:edgenetwork:Subnet
///     name: example_subnet
///     properties:
///       subnetId: example-subnet
///       location: us-west1
///       zone: ""
///       description: Example subnet.
///       network: ${exampleNetwork.id}
///       ipv4Cidrs:
///         - 4.4.4.1/24
///       labels:
///         environment: dev
///   exampleNetwork:
///     type: gcp:edgenetwork:Network
///     name: example_network
///     properties:
///       networkId: example-network
///       location: us-west1
///       zone: ""
///       description: Example network.
///       mtu: 9000
/// ```
/// ### Edgenetwork Subnet With Vlan Id
///
///
/// ```yaml
/// resources:
///   exampleSubnetWithVlanId:
///     type: gcp:edgenetwork:Subnet
///     name: example_subnet_with_vlan_id
///     properties:
///       subnetId: example-subnet-with-vlan-id
///       location: us-west1
///       zone: ""
///       description: Example subnet with VLAN ID.
///       network: ${exampleNetwork.id}
///       ipv6Cidrs:
///         - 4444:4444:4444:4444::1/64
///       vlanId: 44
///       labels:
///         environment: dev
///   exampleNetwork:
///     type: gcp:edgenetwork:Network
///     name: example_network
///     properties:
///       networkId: example-network
///       location: us-west1
///       zone: ""
///       description: Example network.
///       mtu: 9000
/// ```
///
/// ## Import
///
/// Subnet can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/locations/{{location}}/zones/{{zone}}/subnets/{{subnet_id}}`
///
/// * `{{project}}/{{location}}/{{zone}}/{{subnet_id}}`
///
/// * `{{location}}/{{zone}}/{{subnet_id}}`
///
/// * `{{location}}/{{subnet_id}}`
///
/// * `{{name}}`
///
/// When using the `pulumi import` command, Subnet can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:edgenetwork/subnet:Subnet default projects/{{project}}/locations/{{location}}/zones/{{zone}}/subnets/{{subnet_id}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:edgenetwork/subnet:Subnet default {{project}}/{{location}}/{{zone}}/{{subnet_id}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:edgenetwork/subnet:Subnet default {{location}}/{{zone}}/{{subnet_id}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:edgenetwork/subnet:Subnet default {{location}}/{{subnet_id}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:edgenetwork/subnet:Subnet default {{name}}
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod subnet {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct SubnetArgs {
        /// A free-text description of the resource. Max length 1024 characters.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ranges of ipv4 addresses that are owned by this subnetwork, in CIDR format.
        #[builder(into, default)]
        pub ipv4_cidrs: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// The ranges of ipv6 addresses that are owned by this subnetwork, in CIDR format.
        #[builder(into, default)]
        pub ipv6_cidrs: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// Labels associated with this resource.
        ///
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field `effective_labels` for all of the labels present on the resource.
        #[builder(into, default)]
        pub labels: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The Google Cloud region to which the target Distributed Cloud Edge zone belongs.
        #[builder(into)]
        pub location: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The ID of the network to which this router belongs.
        /// Must be of the form: `projects/{{project}}/locations/{{location}}/zones/{{zone}}/networks/{{network_id}}`
        #[builder(into)]
        pub network: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A unique ID that identifies this subnet.
        ///
        ///
        /// - - -
        #[builder(into)]
        pub subnet_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// VLAN ID for this subnetwork. If not specified, one is assigned automatically.
        #[builder(into, default)]
        pub vlan_id: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// The name of the target Distributed Cloud Edge zone.
        #[builder(into)]
        pub zone: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct SubnetResult {
        /// The time when the subnet was created.
        /// A timestamp in RFC3339 UTC "Zulu" format, with nanosecond resolution and up to nine
        /// fractional digits. Examples: `2014-10-02T15:01:23Z` and `2014-10-02T15:01:23.045123456Z`.
        pub create_time: pulumi_gestalt_rust::Output<String>,
        /// A free-text description of the resource. Max length 1024 characters.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// All of labels (key/value pairs) present on the resource in GCP, including the labels configured through Pulumi, other clients and services.
        pub effective_labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The ranges of ipv4 addresses that are owned by this subnetwork, in CIDR format.
        pub ipv4_cidrs: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// The ranges of ipv6 addresses that are owned by this subnetwork, in CIDR format.
        pub ipv6_cidrs: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// Labels associated with this resource.
        ///
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field `effective_labels` for all of the labels present on the resource.
        pub labels: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The Google Cloud region to which the target Distributed Cloud Edge zone belongs.
        pub location: pulumi_gestalt_rust::Output<String>,
        /// The canonical name of this resource, with format
        /// `projects/{{project}}/locations/{{location}}/zones/{{zone}}/subnets/{{subnet_id}}`
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The ID of the network to which this router belongs.
        /// Must be of the form: `projects/{{project}}/locations/{{location}}/zones/{{zone}}/networks/{{network_id}}`
        pub network: pulumi_gestalt_rust::Output<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_gestalt_rust::Output<String>,
        /// The combination of labels configured directly on the resource
        /// and default labels configured on the provider.
        pub pulumi_labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Current stage of the resource to the device by config push.
        pub state: pulumi_gestalt_rust::Output<String>,
        /// A unique ID that identifies this subnet.
        ///
        ///
        /// - - -
        pub subnet_id: pulumi_gestalt_rust::Output<String>,
        /// The time when the subnet was last updated.
        /// A timestamp in RFC3339 UTC "Zulu" format, with nanosecond resolution and up to nine
        /// fractional digits. Examples: `2014-10-02T15:01:23Z` and `2014-10-02T15:01:23.045123456Z`.
        pub update_time: pulumi_gestalt_rust::Output<String>,
        /// VLAN ID for this subnetwork. If not specified, one is assigned automatically.
        pub vlan_id: pulumi_gestalt_rust::Output<i32>,
        /// The name of the target Distributed Cloud Edge zone.
        pub zone: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: SubnetArgs,
    ) -> SubnetResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let description_binding = args.description.get_output(context).get_inner();
        let ipv4_cidrs_binding = args.ipv4_cidrs.get_output(context).get_inner();
        let ipv6_cidrs_binding = args.ipv6_cidrs.get_output(context).get_inner();
        let labels_binding = args.labels.get_output(context).get_inner();
        let location_binding = args.location.get_output(context).get_inner();
        let network_binding = args.network.get_output(context).get_inner();
        let project_binding = args.project.get_output(context).get_inner();
        let subnet_id_binding = args.subnet_id.get_output(context).get_inner();
        let vlan_id_binding = args.vlan_id.get_output(context).get_inner();
        let zone_binding = args.zone.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:edgenetwork/subnet:Subnet".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "ipv4Cidrs".into(),
                    value: &ipv4_cidrs_binding,
                },
                register_interface::ObjectField {
                    name: "ipv6Cidrs".into(),
                    value: &ipv6_cidrs_binding,
                },
                register_interface::ObjectField {
                    name: "labels".into(),
                    value: &labels_binding,
                },
                register_interface::ObjectField {
                    name: "location".into(),
                    value: &location_binding,
                },
                register_interface::ObjectField {
                    name: "network".into(),
                    value: &network_binding,
                },
                register_interface::ObjectField {
                    name: "project".into(),
                    value: &project_binding,
                },
                register_interface::ObjectField {
                    name: "subnetId".into(),
                    value: &subnet_id_binding,
                },
                register_interface::ObjectField {
                    name: "vlanId".into(),
                    value: &vlan_id_binding,
                },
                register_interface::ObjectField {
                    name: "zone".into(),
                    value: &zone_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        SubnetResult {
            create_time: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("createTime"),
            ),
            description: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            effective_labels: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("effectiveLabels"),
            ),
            ipv4_cidrs: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("ipv4Cidrs"),
            ),
            ipv6_cidrs: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("ipv6Cidrs"),
            ),
            labels: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("labels"),
            ),
            location: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("location"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            network: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("network"),
            ),
            project: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("project"),
            ),
            pulumi_labels: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("pulumiLabels"),
            ),
            state: pulumi_gestalt_rust::__private::into_domain(o.extract_field("state")),
            subnet_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("subnetId"),
            ),
            update_time: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("updateTime"),
            ),
            vlan_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("vlanId"),
            ),
            zone: pulumi_gestalt_rust::__private::into_domain(o.extract_field("zone")),
        }
    }
}
