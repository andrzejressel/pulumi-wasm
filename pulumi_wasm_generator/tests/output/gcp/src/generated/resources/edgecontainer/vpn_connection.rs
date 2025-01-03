/// A VPN connection
///
///
/// To get more information about VpnConnection, see:
///
/// * [API documentation](https://cloud.google.com/distributed-cloud/edge/latest/docs/reference/container/rest/v1/projects.locations.vpnConnections)
/// * How-to Guides
///     * [Google Distributed Cloud Edge](https://cloud.google.com/distributed-cloud/edge/latest/docs)
///
/// ## Example Usage
///
/// ### Edgecontainer Vpn Connection
///
///
/// ```yaml
/// resources:
///   cluster:
///     type: gcp:edgecontainer:Cluster
///     properties:
///       name: default
///       location: us-central1
///       authorization:
///         adminUsers:
///           username: admin@hashicorptest.com
///       networking:
///         clusterIpv4CidrBlocks:
///           - 10.0.0.0/16
///         servicesIpv4CidrBlocks:
///           - 10.1.0.0/16
///       fleet:
///         project: projects/${project.number}
///   nodePool:
///     type: gcp:edgecontainer:NodePool
///     name: node_pool
///     properties:
///       name: nodepool-1
///       cluster: ${cluster.name}
///       location: us-central1
///       nodeLocation: us-central1-edge-example-edgesite
///       nodeCount: 3
///   default:
///     type: gcp:edgecontainer:VpnConnection
///     properties:
///       name: vpn-connection-1
///       location: us-central1
///       cluster: projects/${project.number}/locations/us-east1/clusters/${cluster.name}
///       vpc: ${vpc.name}
///       enableHighAvailability: true
///       labels:
///         my_key: my_val
///         other_key: other_val
///     options:
///       dependsOn:
///         - ${nodePool}
///   vpc:
///     type: gcp:compute:Network
///     properties:
///       name: example-vpc
/// variables:
///   project:
///     fn::invoke:
///       function: gcp:organizations:getProject
///       arguments: {}
/// ```
///
/// ## Import
///
/// VpnConnection can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/locations/{{location}}/vpnConnections/{{name}}`
///
/// * `{{project}}/{{location}}/{{name}}`
///
/// * `{{location}}/{{name}}`
///
/// When using the `pulumi import` command, VpnConnection can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:edgecontainer/vpnConnection:VpnConnection default projects/{{project}}/locations/{{location}}/vpnConnections/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:edgecontainer/vpnConnection:VpnConnection default {{project}}/{{location}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:edgecontainer/vpnConnection:VpnConnection default {{location}}/{{name}}
/// ```
///
pub mod vpn_connection {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct VpnConnectionArgs {
        /// The canonical Cluster name to connect to. It is in the form of projects/{project}/locations/{location}/clusters/{cluster}.
        #[builder(into)]
        pub cluster: pulumi_wasm_rust::Output<String>,
        /// Whether this VPN connection has HA enabled on cluster side. If enabled, when creating VPN connection we will attempt to use 2 ANG floating IPs.
        #[builder(into, default)]
        pub enable_high_availability: pulumi_wasm_rust::Output<Option<bool>>,
        /// Labels associated with this resource.
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field `effective_labels` for all of the labels present on the resource.
        #[builder(into, default)]
        pub labels: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Google Cloud Platform location.
        ///
        ///
        /// - - -
        #[builder(into)]
        pub location: pulumi_wasm_rust::Output<String>,
        /// The resource name of VPN connection
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// NAT gateway IP, or WAN IP address. If a customer has multiple NAT IPs, the customer needs to configure NAT such that only one external IP maps to the GMEC Anthos cluster.
        /// This is empty if NAT is not used.
        #[builder(into, default)]
        pub nat_gateway_ip: pulumi_wasm_rust::Output<Option<String>>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_wasm_rust::Output<Option<String>>,
        /// The VPN connection Cloud Router name.
        #[builder(into, default)]
        pub router: pulumi_wasm_rust::Output<Option<String>>,
        /// The network ID of VPC to connect to.
        #[builder(into, default)]
        pub vpc: pulumi_wasm_rust::Output<Option<String>>,
        /// Project detail of the VPC network. Required if VPC is in a different project than the cluster project.
        /// Structure is documented below.
        #[builder(into, default)]
        pub vpc_project: pulumi_wasm_rust::Output<
            Option<super::super::types::edgecontainer::VpnConnectionVpcProject>,
        >,
    }
    #[allow(dead_code)]
    pub struct VpnConnectionResult {
        /// The canonical Cluster name to connect to. It is in the form of projects/{project}/locations/{location}/clusters/{cluster}.
        pub cluster: pulumi_wasm_rust::Output<String>,
        /// The time when the VPN connection was created.
        pub create_time: pulumi_wasm_rust::Output<String>,
        /// A nested object resource.
        /// Structure is documented below.
        pub details: pulumi_wasm_rust::Output<
            Vec<super::super::types::edgecontainer::VpnConnectionDetail>,
        >,
        /// All of labels (key/value pairs) present on the resource in GCP, including the labels configured through Pulumi, other clients and services.
        pub effective_labels: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Whether this VPN connection has HA enabled on cluster side. If enabled, when creating VPN connection we will attempt to use 2 ANG floating IPs.
        pub enable_high_availability: pulumi_wasm_rust::Output<bool>,
        /// Labels associated with this resource.
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field `effective_labels` for all of the labels present on the resource.
        pub labels: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Google Cloud Platform location.
        ///
        ///
        /// - - -
        pub location: pulumi_wasm_rust::Output<String>,
        /// The resource name of VPN connection
        pub name: pulumi_wasm_rust::Output<String>,
        /// NAT gateway IP, or WAN IP address. If a customer has multiple NAT IPs, the customer needs to configure NAT such that only one external IP maps to the GMEC Anthos cluster.
        /// This is empty if NAT is not used.
        pub nat_gateway_ip: pulumi_wasm_rust::Output<Option<String>>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_wasm_rust::Output<String>,
        /// The combination of labels configured directly on the resource
        /// and default labels configured on the provider.
        pub pulumi_labels: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The VPN connection Cloud Router name.
        pub router: pulumi_wasm_rust::Output<Option<String>>,
        /// The time when the VPN connection was last updated.
        pub update_time: pulumi_wasm_rust::Output<String>,
        /// The network ID of VPC to connect to.
        pub vpc: pulumi_wasm_rust::Output<Option<String>>,
        /// Project detail of the VPC network. Required if VPC is in a different project than the cluster project.
        /// Structure is documented below.
        pub vpc_project: pulumi_wasm_rust::Output<
            Option<super::super::types::edgecontainer::VpnConnectionVpcProject>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: VpnConnectionArgs) -> VpnConnectionResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let cluster_binding = args.cluster.get_inner();
        let enable_high_availability_binding = args.enable_high_availability.get_inner();
        let labels_binding = args.labels.get_inner();
        let location_binding = args.location.get_inner();
        let name_binding = args.name.get_inner();
        let nat_gateway_ip_binding = args.nat_gateway_ip.get_inner();
        let project_binding = args.project.get_inner();
        let router_binding = args.router.get_inner();
        let vpc_binding = args.vpc.get_inner();
        let vpc_project_binding = args.vpc_project.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:edgecontainer/vpnConnection:VpnConnection".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "cluster".into(),
                    value: &cluster_binding,
                },
                register_interface::ObjectField {
                    name: "enableHighAvailability".into(),
                    value: &enable_high_availability_binding,
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
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "natGatewayIp".into(),
                    value: &nat_gateway_ip_binding,
                },
                register_interface::ObjectField {
                    name: "project".into(),
                    value: &project_binding,
                },
                register_interface::ObjectField {
                    name: "router".into(),
                    value: &router_binding,
                },
                register_interface::ObjectField {
                    name: "vpc".into(),
                    value: &vpc_binding,
                },
                register_interface::ObjectField {
                    name: "vpcProject".into(),
                    value: &vpc_project_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "cluster".into(),
                },
                register_interface::ResultField {
                    name: "createTime".into(),
                },
                register_interface::ResultField {
                    name: "details".into(),
                },
                register_interface::ResultField {
                    name: "effectiveLabels".into(),
                },
                register_interface::ResultField {
                    name: "enableHighAvailability".into(),
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
                    name: "natGatewayIp".into(),
                },
                register_interface::ResultField {
                    name: "project".into(),
                },
                register_interface::ResultField {
                    name: "pulumiLabels".into(),
                },
                register_interface::ResultField {
                    name: "router".into(),
                },
                register_interface::ResultField {
                    name: "updateTime".into(),
                },
                register_interface::ResultField {
                    name: "vpc".into(),
                },
                register_interface::ResultField {
                    name: "vpcProject".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        VpnConnectionResult {
            cluster: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("cluster").unwrap(),
            ),
            create_time: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("createTime").unwrap(),
            ),
            details: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("details").unwrap(),
            ),
            effective_labels: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("effectiveLabels").unwrap(),
            ),
            enable_high_availability: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("enableHighAvailability").unwrap(),
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
            nat_gateway_ip: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("natGatewayIp").unwrap(),
            ),
            project: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("project").unwrap(),
            ),
            pulumi_labels: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("pulumiLabels").unwrap(),
            ),
            router: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("router").unwrap(),
            ),
            update_time: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("updateTime").unwrap(),
            ),
            vpc: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("vpc").unwrap(),
            ),
            vpc_project: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("vpcProject").unwrap(),
            ),
        }
    }
}
