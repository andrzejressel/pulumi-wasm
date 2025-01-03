/// A Google Cloud Memcache instance.
///
///
/// To get more information about Instance, see:
///
/// * [API documentation](https://cloud.google.com/memorystore/docs/memcached/reference/rest/v1/projects.locations.instances)
/// * How-to Guides
///     * [Official Documentation](https://cloud.google.com/memcache/docs/creating-instances)
///
/// ## Example Usage
///
/// ### Memcache Instance Basic
///
///
/// ```yaml
/// resources:
///   # This example assumes this network already exists.
///   # // The API creates a tenant network per network authorized for a
///   # // Memcache instance and that network is not deleted when the user-created
///   # // network (authorized_network) is deleted, so this prevents issues
///   # // with tenant network quota.
///   # // If this network hasn't been created and you are using this example in your
///   # // config, add an additional network resource or change
///   # // this from "data"to "resource"
///   memcacheNetwork:
///     type: gcp:compute:Network
///     name: memcache_network
///     properties:
///       name: test-network
///   serviceRange:
///     type: gcp:compute:GlobalAddress
///     name: service_range
///     properties:
///       name: address
///       purpose: VPC_PEERING
///       addressType: INTERNAL
///       prefixLength: 16
///       network: ${memcacheNetwork.id}
///   privateServiceConnection:
///     type: gcp:servicenetworking:Connection
///     name: private_service_connection
///     properties:
///       network: ${memcacheNetwork.id}
///       service: servicenetworking.googleapis.com
///       reservedPeeringRanges:
///         - ${serviceRange.name}
///   instance:
///     type: gcp:memcache:Instance
///     properties:
///       name: test-instance
///       authorizedNetwork: ${privateServiceConnection.network}
///       labels:
///         env: test
///       nodeConfig:
///         cpuCount: 1
///         memorySizeMb: 1024
///       nodeCount: 1
///       memcacheVersion: MEMCACHE_1_5
///       maintenancePolicy:
///         weeklyMaintenanceWindows:
///           - day: SATURDAY
///             duration: 14400s
///             startTime:
///               hours: 0
///               minutes: 30
///               seconds: 0
///               nanos: 0
/// ```
///
/// ## Import
///
/// Instance can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/locations/{{region}}/instances/{{name}}`
///
/// * `{{project}}/{{region}}/{{name}}`
///
/// * `{{region}}/{{name}}`
///
/// * `{{name}}`
///
/// When using the `pulumi import` command, Instance can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:memcache/instance:Instance default projects/{{project}}/locations/{{region}}/instances/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:memcache/instance:Instance default {{project}}/{{region}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:memcache/instance:Instance default {{region}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:memcache/instance:Instance default {{name}}
/// ```
///
pub mod instance {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct InstanceArgs {
        /// The full name of the GCE network to connect the instance to. If not provided, 'default' will be used.
        #[builder(into, default)]
        pub authorized_network: pulumi_wasm_rust::Output<Option<String>>,
        /// A user-visible name for the instance.
        #[builder(into, default)]
        pub display_name: pulumi_wasm_rust::Output<Option<String>>,
        /// Resource labels to represent user-provided metadata. **Note**: This field is non-authoritative, and will only manage the
        /// labels present in your configuration. Please refer to the field 'effective_labels' for all of the labels present on the
        /// resource.
        #[builder(into, default)]
        pub labels: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Maintenance policy for an instance.
        #[builder(into, default)]
        pub maintenance_policy: pulumi_wasm_rust::Output<
            Option<super::super::types::memcache::InstanceMaintenancePolicy>,
        >,
        /// User-specified parameters for this memcache instance.
        #[builder(into, default)]
        pub memcache_parameters: pulumi_wasm_rust::Output<
            Option<super::super::types::memcache::InstanceMemcacheParameters>,
        >,
        /// The major version of Memcached software. If not provided, latest supported version will be used. Currently the latest
        /// supported major version is MEMCACHE_1_5. The minor version will be automatically determined by our system based on the
        /// latest supported minor version. Default value: "MEMCACHE_1_5" Possible values: ["MEMCACHE_1_5", "MEMCACHE_1_6_15"]
        #[builder(into, default)]
        pub memcache_version: pulumi_wasm_rust::Output<Option<String>>,
        /// The resource name of the instance.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// Configuration for memcache nodes.
        /// Structure is documented below.
        #[builder(into)]
        pub node_config: pulumi_wasm_rust::Output<
            super::super::types::memcache::InstanceNodeConfig,
        >,
        /// Number of nodes in the memcache instance.
        #[builder(into)]
        pub node_count: pulumi_wasm_rust::Output<i32>,
        #[builder(into, default)]
        pub project: pulumi_wasm_rust::Output<Option<String>>,
        /// The region of the Memcache instance. If it is not provided, the provider region is used.
        #[builder(into, default)]
        pub region: pulumi_wasm_rust::Output<Option<String>>,
        /// Contains the name of allocated IP address ranges associated with the private service access connection for example,
        /// "test-default" associated with IP range 10.0.0.0/29.
        #[builder(into, default)]
        pub reserved_ip_range_ids: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// Zones where memcache nodes should be provisioned. If not provided, all zones will be used.
        #[builder(into, default)]
        pub zones: pulumi_wasm_rust::Output<Option<Vec<String>>>,
    }
    #[allow(dead_code)]
    pub struct InstanceResult {
        /// The full name of the GCE network to connect the instance to. If not provided, 'default' will be used.
        pub authorized_network: pulumi_wasm_rust::Output<String>,
        /// Creation timestamp in RFC3339 text format.
        pub create_time: pulumi_wasm_rust::Output<String>,
        /// Endpoint for Discovery API
        pub discovery_endpoint: pulumi_wasm_rust::Output<String>,
        /// A user-visible name for the instance.
        pub display_name: pulumi_wasm_rust::Output<String>,
        /// All of labels (key/value pairs) present on the resource in GCP, including the labels configured through Pulumi, other clients and services.
        pub effective_labels: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Resource labels to represent user-provided metadata. **Note**: This field is non-authoritative, and will only manage the
        /// labels present in your configuration. Please refer to the field 'effective_labels' for all of the labels present on the
        /// resource.
        pub labels: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Maintenance policy for an instance.
        pub maintenance_policy: pulumi_wasm_rust::Output<
            Option<super::super::types::memcache::InstanceMaintenancePolicy>,
        >,
        /// Output only. Published maintenance schedule.
        /// Structure is documented below.
        pub maintenance_schedules: pulumi_wasm_rust::Output<
            Vec<super::super::types::memcache::InstanceMaintenanceSchedule>,
        >,
        /// The full version of memcached server running on this instance.
        pub memcache_full_version: pulumi_wasm_rust::Output<String>,
        /// Additional information about the instance state, if available.
        /// Structure is documented below.
        pub memcache_nodes: pulumi_wasm_rust::Output<
            Vec<super::super::types::memcache::InstanceMemcacheNode>,
        >,
        /// User-specified parameters for this memcache instance.
        pub memcache_parameters: pulumi_wasm_rust::Output<
            Option<super::super::types::memcache::InstanceMemcacheParameters>,
        >,
        /// The major version of Memcached software. If not provided, latest supported version will be used. Currently the latest
        /// supported major version is MEMCACHE_1_5. The minor version will be automatically determined by our system based on the
        /// latest supported minor version. Default value: "MEMCACHE_1_5" Possible values: ["MEMCACHE_1_5", "MEMCACHE_1_6_15"]
        pub memcache_version: pulumi_wasm_rust::Output<Option<String>>,
        /// The resource name of the instance.
        pub name: pulumi_wasm_rust::Output<String>,
        /// Configuration for memcache nodes.
        /// Structure is documented below.
        pub node_config: pulumi_wasm_rust::Output<
            super::super::types::memcache::InstanceNodeConfig,
        >,
        /// Number of nodes in the memcache instance.
        pub node_count: pulumi_wasm_rust::Output<i32>,
        pub project: pulumi_wasm_rust::Output<String>,
        /// The combination of labels configured directly on the resource
        /// and default labels configured on the provider.
        pub pulumi_labels: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The region of the Memcache instance. If it is not provided, the provider region is used.
        pub region: pulumi_wasm_rust::Output<String>,
        /// Contains the name of allocated IP address ranges associated with the private service access connection for example,
        /// "test-default" associated with IP range 10.0.0.0/29.
        pub reserved_ip_range_ids: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// Zones where memcache nodes should be provisioned. If not provided, all zones will be used.
        pub zones: pulumi_wasm_rust::Output<Vec<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: InstanceArgs) -> InstanceResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let authorized_network_binding = args.authorized_network.get_inner();
        let display_name_binding = args.display_name.get_inner();
        let labels_binding = args.labels.get_inner();
        let maintenance_policy_binding = args.maintenance_policy.get_inner();
        let memcache_parameters_binding = args.memcache_parameters.get_inner();
        let memcache_version_binding = args.memcache_version.get_inner();
        let name_binding = args.name.get_inner();
        let node_config_binding = args.node_config.get_inner();
        let node_count_binding = args.node_count.get_inner();
        let project_binding = args.project.get_inner();
        let region_binding = args.region.get_inner();
        let reserved_ip_range_ids_binding = args.reserved_ip_range_ids.get_inner();
        let zones_binding = args.zones.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:memcache/instance:Instance".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "authorizedNetwork".into(),
                    value: &authorized_network_binding,
                },
                register_interface::ObjectField {
                    name: "displayName".into(),
                    value: &display_name_binding,
                },
                register_interface::ObjectField {
                    name: "labels".into(),
                    value: &labels_binding,
                },
                register_interface::ObjectField {
                    name: "maintenancePolicy".into(),
                    value: &maintenance_policy_binding,
                },
                register_interface::ObjectField {
                    name: "memcacheParameters".into(),
                    value: &memcache_parameters_binding,
                },
                register_interface::ObjectField {
                    name: "memcacheVersion".into(),
                    value: &memcache_version_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "nodeConfig".into(),
                    value: &node_config_binding,
                },
                register_interface::ObjectField {
                    name: "nodeCount".into(),
                    value: &node_count_binding,
                },
                register_interface::ObjectField {
                    name: "project".into(),
                    value: &project_binding,
                },
                register_interface::ObjectField {
                    name: "region".into(),
                    value: &region_binding,
                },
                register_interface::ObjectField {
                    name: "reservedIpRangeIds".into(),
                    value: &reserved_ip_range_ids_binding,
                },
                register_interface::ObjectField {
                    name: "zones".into(),
                    value: &zones_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "authorizedNetwork".into(),
                },
                register_interface::ResultField {
                    name: "createTime".into(),
                },
                register_interface::ResultField {
                    name: "discoveryEndpoint".into(),
                },
                register_interface::ResultField {
                    name: "displayName".into(),
                },
                register_interface::ResultField {
                    name: "effectiveLabels".into(),
                },
                register_interface::ResultField {
                    name: "labels".into(),
                },
                register_interface::ResultField {
                    name: "maintenancePolicy".into(),
                },
                register_interface::ResultField {
                    name: "maintenanceSchedules".into(),
                },
                register_interface::ResultField {
                    name: "memcacheFullVersion".into(),
                },
                register_interface::ResultField {
                    name: "memcacheNodes".into(),
                },
                register_interface::ResultField {
                    name: "memcacheParameters".into(),
                },
                register_interface::ResultField {
                    name: "memcacheVersion".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "nodeConfig".into(),
                },
                register_interface::ResultField {
                    name: "nodeCount".into(),
                },
                register_interface::ResultField {
                    name: "project".into(),
                },
                register_interface::ResultField {
                    name: "pulumiLabels".into(),
                },
                register_interface::ResultField {
                    name: "region".into(),
                },
                register_interface::ResultField {
                    name: "reservedIpRangeIds".into(),
                },
                register_interface::ResultField {
                    name: "zones".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        InstanceResult {
            authorized_network: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("authorizedNetwork").unwrap(),
            ),
            create_time: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("createTime").unwrap(),
            ),
            discovery_endpoint: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("discoveryEndpoint").unwrap(),
            ),
            display_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("displayName").unwrap(),
            ),
            effective_labels: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("effectiveLabels").unwrap(),
            ),
            labels: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("labels").unwrap(),
            ),
            maintenance_policy: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("maintenancePolicy").unwrap(),
            ),
            maintenance_schedules: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("maintenanceSchedules").unwrap(),
            ),
            memcache_full_version: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("memcacheFullVersion").unwrap(),
            ),
            memcache_nodes: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("memcacheNodes").unwrap(),
            ),
            memcache_parameters: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("memcacheParameters").unwrap(),
            ),
            memcache_version: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("memcacheVersion").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            node_config: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("nodeConfig").unwrap(),
            ),
            node_count: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("nodeCount").unwrap(),
            ),
            project: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("project").unwrap(),
            ),
            pulumi_labels: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("pulumiLabels").unwrap(),
            ),
            region: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("region").unwrap(),
            ),
            reserved_ip_range_ids: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("reservedIpRangeIds").unwrap(),
            ),
            zones: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("zones").unwrap(),
            ),
        }
    }
}
