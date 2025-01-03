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
pub mod subnet {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct SubnetArgs {
        /// A free-text description of the resource. Max length 1024 characters.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// The ranges of ipv4 addresses that are owned by this subnetwork, in CIDR format.
        #[builder(into, default)]
        pub ipv4_cidrs: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// The ranges of ipv6 addresses that are owned by this subnetwork, in CIDR format.
        #[builder(into, default)]
        pub ipv6_cidrs: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// Labels associated with this resource.
        ///
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field `effective_labels` for all of the labels present on the resource.
        #[builder(into, default)]
        pub labels: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The Google Cloud region to which the target Distributed Cloud Edge zone belongs.
        #[builder(into)]
        pub location: pulumi_wasm_rust::Output<String>,
        /// The ID of the network to which this router belongs.
        /// Must be of the form: `projects/{{project}}/locations/{{location}}/zones/{{zone}}/networks/{{network_id}}`
        #[builder(into)]
        pub network: pulumi_wasm_rust::Output<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_wasm_rust::Output<Option<String>>,
        /// A unique ID that identifies this subnet.
        ///
        ///
        /// - - -
        #[builder(into)]
        pub subnet_id: pulumi_wasm_rust::Output<String>,
        /// VLAN ID for this subnetwork. If not specified, one is assigned automatically.
        #[builder(into, default)]
        pub vlan_id: pulumi_wasm_rust::Output<Option<i32>>,
        /// The name of the target Distributed Cloud Edge zone.
        #[builder(into)]
        pub zone: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct SubnetResult {
        /// The time when the subnet was created.
        /// A timestamp in RFC3339 UTC "Zulu" format, with nanosecond resolution and up to nine
        /// fractional digits. Examples: `2014-10-02T15:01:23Z` and `2014-10-02T15:01:23.045123456Z`.
        pub create_time: pulumi_wasm_rust::Output<String>,
        /// A free-text description of the resource. Max length 1024 characters.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// All of labels (key/value pairs) present on the resource in GCP, including the labels configured through Pulumi, other clients and services.
        pub effective_labels: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The ranges of ipv4 addresses that are owned by this subnetwork, in CIDR format.
        pub ipv4_cidrs: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// The ranges of ipv6 addresses that are owned by this subnetwork, in CIDR format.
        pub ipv6_cidrs: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// Labels associated with this resource.
        ///
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field `effective_labels` for all of the labels present on the resource.
        pub labels: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The Google Cloud region to which the target Distributed Cloud Edge zone belongs.
        pub location: pulumi_wasm_rust::Output<String>,
        /// The canonical name of this resource, with format
        /// `projects/{{project}}/locations/{{location}}/zones/{{zone}}/subnets/{{subnet_id}}`
        pub name: pulumi_wasm_rust::Output<String>,
        /// The ID of the network to which this router belongs.
        /// Must be of the form: `projects/{{project}}/locations/{{location}}/zones/{{zone}}/networks/{{network_id}}`
        pub network: pulumi_wasm_rust::Output<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_wasm_rust::Output<String>,
        /// The combination of labels configured directly on the resource
        /// and default labels configured on the provider.
        pub pulumi_labels: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Current stage of the resource to the device by config push.
        pub state: pulumi_wasm_rust::Output<String>,
        /// A unique ID that identifies this subnet.
        ///
        ///
        /// - - -
        pub subnet_id: pulumi_wasm_rust::Output<String>,
        /// The time when the subnet was last updated.
        /// A timestamp in RFC3339 UTC "Zulu" format, with nanosecond resolution and up to nine
        /// fractional digits. Examples: `2014-10-02T15:01:23Z` and `2014-10-02T15:01:23.045123456Z`.
        pub update_time: pulumi_wasm_rust::Output<String>,
        /// VLAN ID for this subnetwork. If not specified, one is assigned automatically.
        pub vlan_id: pulumi_wasm_rust::Output<i32>,
        /// The name of the target Distributed Cloud Edge zone.
        pub zone: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: SubnetArgs) -> SubnetResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let description_binding = args.description.get_inner();
        let ipv4_cidrs_binding = args.ipv4_cidrs.get_inner();
        let ipv6_cidrs_binding = args.ipv6_cidrs.get_inner();
        let labels_binding = args.labels.get_inner();
        let location_binding = args.location.get_inner();
        let network_binding = args.network.get_inner();
        let project_binding = args.project.get_inner();
        let subnet_id_binding = args.subnet_id.get_inner();
        let vlan_id_binding = args.vlan_id.get_inner();
        let zone_binding = args.zone.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:edgenetwork/subnet:Subnet".into(),
            name: name.to_string(),
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
                    name: "ipv4Cidrs".into(),
                },
                register_interface::ResultField {
                    name: "ipv6Cidrs".into(),
                },
                register_interface::ResultField {
                    name: "labels".into(),
                },
                register_interface::ResultField {
                    name: "location".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "network".into(),
                },
                register_interface::ResultField {
                    name: "project".into(),
                },
                register_interface::ResultField {
                    name: "pulumiLabels".into(),
                },
                register_interface::ResultField {
                    name: "state".into(),
                },
                register_interface::ResultField {
                    name: "subnetId".into(),
                },
                register_interface::ResultField {
                    name: "updateTime".into(),
                },
                register_interface::ResultField {
                    name: "vlanId".into(),
                },
                register_interface::ResultField {
                    name: "zone".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        SubnetResult {
            create_time: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("createTime").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            effective_labels: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("effectiveLabels").unwrap(),
            ),
            ipv4_cidrs: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("ipv4Cidrs").unwrap(),
            ),
            ipv6_cidrs: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("ipv6Cidrs").unwrap(),
            ),
            labels: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("labels").unwrap(),
            ),
            location: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("location").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            network: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("network").unwrap(),
            ),
            project: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("project").unwrap(),
            ),
            pulumi_labels: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("pulumiLabels").unwrap(),
            ),
            state: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("state").unwrap(),
            ),
            subnet_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("subnetId").unwrap(),
            ),
            update_time: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("updateTime").unwrap(),
            ),
            vlan_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("vlanId").unwrap(),
            ),
            zone: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("zone").unwrap(),
            ),
        }
    }
}
