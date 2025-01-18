/// Represents a network policy resource. Network policies are regional resources.
///
///
/// To get more information about NetworkPolicy, see:
///
/// * [API documentation](https://cloud.google.com/vmware-engine/docs/reference/rest/v1/projects.locations.networkPolicies)
///
/// ## Example Usage
///
/// ### Vmware Engine Network Policy Basic
///
///
/// ```yaml
/// resources:
///   network-policy-nw:
///     type: gcp:vmwareengine:Network
///     properties:
///       name: sample-network
///       location: global
///       type: STANDARD
///       description: VMwareEngine standard network sample
///   vmw-engine-network-policy:
///     type: gcp:vmwareengine:NetworkPolicy
///     properties:
///       location: us-west1
///       name: sample-network-policy
///       edgeServicesCidr: 192.168.30.0/26
///       vmwareEngineNetwork: ${["network-policy-nw"].id}
/// ```
/// ### Vmware Engine Network Policy Full
///
///
/// ```yaml
/// resources:
///   network-policy-nw:
///     type: gcp:vmwareengine:Network
///     properties:
///       name: sample-network
///       location: global
///       type: STANDARD
///       description: VMwareEngine standard network sample
///   vmw-engine-network-policy:
///     type: gcp:vmwareengine:NetworkPolicy
///     properties:
///       location: us-west1
///       name: sample-network-policy
///       edgeServicesCidr: 192.168.30.0/26
///       vmwareEngineNetwork: ${["network-policy-nw"].id}
///       description: Sample Network Policy
///       internetAccess:
///         enabled: true
///       externalIp:
///         enabled: true
/// ```
///
/// ## Import
///
/// NetworkPolicy can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/locations/{{location}}/networkPolicies/{{name}}`
///
/// * `{{project}}/{{location}}/{{name}}`
///
/// * `{{location}}/{{name}}`
///
/// When using the `pulumi import` command, NetworkPolicy can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:vmwareengine/networkPolicy:NetworkPolicy default projects/{{project}}/locations/{{location}}/networkPolicies/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:vmwareengine/networkPolicy:NetworkPolicy default {{project}}/{{location}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:vmwareengine/networkPolicy:NetworkPolicy default {{location}}/{{name}}
/// ```
///
pub mod network_policy {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct NetworkPolicyArgs {
        /// User-provided description for this network policy.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// IP address range in CIDR notation used to create internet access and external IP access.
        /// An RFC 1918 CIDR block, with a "/26" prefix, is required. The range cannot overlap with any
        /// prefixes either in the consumer VPC network or in use by the private clouds attached to that VPC network.
        #[builder(into)]
        pub edge_services_cidr: pulumi_wasm_rust::Output<String>,
        /// Network service that allows External IP addresses to be assigned to VMware workloads.
        /// This service can only be enabled when internetAccess is also enabled.
        /// Structure is documented below.
        #[builder(into, default)]
        pub external_ip: pulumi_wasm_rust::Output<
            Option<super::super::types::vmwareengine::NetworkPolicyExternalIp>,
        >,
        /// Network service that allows VMware workloads to access the internet.
        /// Structure is documented below.
        #[builder(into, default)]
        pub internet_access: pulumi_wasm_rust::Output<
            Option<super::super::types::vmwareengine::NetworkPolicyInternetAccess>,
        >,
        /// The resource name of the location (region) to create the new network policy in.
        /// Resource names are schemeless URIs that follow the conventions in https://cloud.google.com/apis/design/resource_names.
        /// For example: projects/my-project/locations/us-central1
        #[builder(into)]
        pub location: pulumi_wasm_rust::Output<String>,
        /// The ID of the Network Policy.
        ///
        ///
        /// - - -
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_wasm_rust::Output<Option<String>>,
        /// The relative resource name of the VMware Engine network. Specify the name in the following form:
        /// projects/{project}/locations/{location}/vmwareEngineNetworks/{vmwareEngineNetworkId} where {project}
        /// can either be a project number or a project ID.
        #[builder(into)]
        pub vmware_engine_network: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct NetworkPolicyResult {
        /// Creation time of this resource.
        /// A timestamp in RFC3339 UTC "Zulu" format, with nanosecond resolution and
        /// up to nine fractional digits. Examples: "2014-10-02T15:01:23Z" and "2014-10-02T15:01:23.045123456Z".
        pub create_time: pulumi_wasm_rust::Output<String>,
        /// User-provided description for this network policy.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// IP address range in CIDR notation used to create internet access and external IP access.
        /// An RFC 1918 CIDR block, with a "/26" prefix, is required. The range cannot overlap with any
        /// prefixes either in the consumer VPC network or in use by the private clouds attached to that VPC network.
        pub edge_services_cidr: pulumi_wasm_rust::Output<String>,
        /// Network service that allows External IP addresses to be assigned to VMware workloads.
        /// This service can only be enabled when internetAccess is also enabled.
        /// Structure is documented below.
        pub external_ip: pulumi_wasm_rust::Output<
            super::super::types::vmwareengine::NetworkPolicyExternalIp,
        >,
        /// Network service that allows VMware workloads to access the internet.
        /// Structure is documented below.
        pub internet_access: pulumi_wasm_rust::Output<
            super::super::types::vmwareengine::NetworkPolicyInternetAccess,
        >,
        /// The resource name of the location (region) to create the new network policy in.
        /// Resource names are schemeless URIs that follow the conventions in https://cloud.google.com/apis/design/resource_names.
        /// For example: projects/my-project/locations/us-central1
        pub location: pulumi_wasm_rust::Output<String>,
        /// The ID of the Network Policy.
        ///
        ///
        /// - - -
        pub name: pulumi_wasm_rust::Output<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_wasm_rust::Output<String>,
        /// System-generated unique identifier for the resource.
        pub uid: pulumi_wasm_rust::Output<String>,
        /// Last updated time of this resource.
        /// A timestamp in RFC3339 UTC "Zulu" format, with nanosecond resolution and up to nine
        /// fractional digits. Examples: "2014-10-02T15:01:23Z" and "2014-10-02T15:01:23.045123456Z".
        pub update_time: pulumi_wasm_rust::Output<String>,
        /// The relative resource name of the VMware Engine network. Specify the name in the following form:
        /// projects/{project}/locations/{location}/vmwareEngineNetworks/{vmwareEngineNetworkId} where {project}
        /// can either be a project number or a project ID.
        pub vmware_engine_network: pulumi_wasm_rust::Output<String>,
        /// The canonical name of the VMware Engine network in the form:
        /// projects/{project_number}/locations/{location}/vmwareEngineNetworks/{vmwareEngineNetworkId}
        pub vmware_engine_network_canonical: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: NetworkPolicyArgs) -> NetworkPolicyResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let description_binding = args.description.get_inner();
        let edge_services_cidr_binding = args.edge_services_cidr.get_inner();
        let external_ip_binding = args.external_ip.get_inner();
        let internet_access_binding = args.internet_access.get_inner();
        let location_binding = args.location.get_inner();
        let name_binding = args.name.get_inner();
        let project_binding = args.project.get_inner();
        let vmware_engine_network_binding = args.vmware_engine_network.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:vmwareengine/networkPolicy:NetworkPolicy".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "edgeServicesCidr".into(),
                    value: &edge_services_cidr_binding,
                },
                register_interface::ObjectField {
                    name: "externalIp".into(),
                    value: &external_ip_binding,
                },
                register_interface::ObjectField {
                    name: "internetAccess".into(),
                    value: &internet_access_binding,
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
                    name: "project".into(),
                    value: &project_binding,
                },
                register_interface::ObjectField {
                    name: "vmwareEngineNetwork".into(),
                    value: &vmware_engine_network_binding,
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
                    name: "edgeServicesCidr".into(),
                },
                register_interface::ResultField {
                    name: "externalIp".into(),
                },
                register_interface::ResultField {
                    name: "internetAccess".into(),
                },
                register_interface::ResultField {
                    name: "location".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "project".into(),
                },
                register_interface::ResultField {
                    name: "uid".into(),
                },
                register_interface::ResultField {
                    name: "updateTime".into(),
                },
                register_interface::ResultField {
                    name: "vmwareEngineNetwork".into(),
                },
                register_interface::ResultField {
                    name: "vmwareEngineNetworkCanonical".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        NetworkPolicyResult {
            create_time: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("createTime").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            edge_services_cidr: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("edgeServicesCidr").unwrap(),
            ),
            external_ip: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("externalIp").unwrap(),
            ),
            internet_access: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("internetAccess").unwrap(),
            ),
            location: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("location").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            project: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("project").unwrap(),
            ),
            uid: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("uid").unwrap(),
            ),
            update_time: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("updateTime").unwrap(),
            ),
            vmware_engine_network: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("vmwareEngineNetwork").unwrap(),
            ),
            vmware_engine_network_canonical: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("vmwareEngineNetworkCanonical").unwrap(),
            ),
        }
    }
}
