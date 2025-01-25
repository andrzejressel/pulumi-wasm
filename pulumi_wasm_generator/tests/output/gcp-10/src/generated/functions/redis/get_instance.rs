pub mod get_instance {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetInstanceArgs {
        /// The name of a Redis instance.
        ///
        /// - - -
        #[builder(into)]
        pub name: pulumi_wasm_rust::InputOrOutput<String>,
        /// The project in which the resource belongs. If it
        /// is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The region in which the resource belongs. If it
        /// is not provided, the provider region is used.
        #[builder(into, default)]
        pub region: pulumi_wasm_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetInstanceResult {
        pub alternative_location_id: pulumi_wasm_rust::Output<String>,
        pub auth_enabled: pulumi_wasm_rust::Output<bool>,
        pub auth_string: pulumi_wasm_rust::Output<String>,
        pub authorized_network: pulumi_wasm_rust::Output<String>,
        pub connect_mode: pulumi_wasm_rust::Output<String>,
        pub create_time: pulumi_wasm_rust::Output<String>,
        pub current_location_id: pulumi_wasm_rust::Output<String>,
        pub customer_managed_key: pulumi_wasm_rust::Output<String>,
        pub display_name: pulumi_wasm_rust::Output<String>,
        pub effective_labels: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        pub host: pulumi_wasm_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        pub labels: pulumi_wasm_rust::Output<std::collections::HashMap<String, String>>,
        pub location_id: pulumi_wasm_rust::Output<String>,
        pub maintenance_policies: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::redis::GetInstanceMaintenancePolicy>,
        >,
        pub maintenance_schedules: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::redis::GetInstanceMaintenanceSchedule>,
        >,
        pub maintenance_version: pulumi_wasm_rust::Output<String>,
        pub memory_size_gb: pulumi_wasm_rust::Output<i32>,
        pub name: pulumi_wasm_rust::Output<String>,
        pub nodes: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::redis::GetInstanceNode>,
        >,
        pub persistence_configs: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::redis::GetInstancePersistenceConfig>,
        >,
        pub persistence_iam_identity: pulumi_wasm_rust::Output<String>,
        pub port: pulumi_wasm_rust::Output<i32>,
        pub project: pulumi_wasm_rust::Output<Option<String>>,
        pub pulumi_labels: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        pub read_endpoint: pulumi_wasm_rust::Output<String>,
        pub read_endpoint_port: pulumi_wasm_rust::Output<i32>,
        pub read_replicas_mode: pulumi_wasm_rust::Output<String>,
        pub redis_configs: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        pub redis_version: pulumi_wasm_rust::Output<String>,
        pub region: pulumi_wasm_rust::Output<Option<String>>,
        pub replica_count: pulumi_wasm_rust::Output<i32>,
        pub reserved_ip_range: pulumi_wasm_rust::Output<String>,
        pub secondary_ip_range: pulumi_wasm_rust::Output<String>,
        pub server_ca_certs: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::redis::GetInstanceServerCaCert>,
        >,
        pub tier: pulumi_wasm_rust::Output<String>,
        pub transit_encryption_mode: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetInstanceArgs,
    ) -> GetInstanceResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let name_binding = args.name.get_output(context).get_inner();
        let project_binding = args.project.get_output(context).get_inner();
        let region_binding = args.region.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "gcp:redis/getInstance:getInstance".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "project".into(),
                    value: &project_binding,
                },
                register_interface::ObjectField {
                    name: "region".into(),
                    value: &region_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "alternativeLocationId".into(),
                },
                register_interface::ResultField {
                    name: "authEnabled".into(),
                },
                register_interface::ResultField {
                    name: "authString".into(),
                },
                register_interface::ResultField {
                    name: "authorizedNetwork".into(),
                },
                register_interface::ResultField {
                    name: "connectMode".into(),
                },
                register_interface::ResultField {
                    name: "createTime".into(),
                },
                register_interface::ResultField {
                    name: "currentLocationId".into(),
                },
                register_interface::ResultField {
                    name: "customerManagedKey".into(),
                },
                register_interface::ResultField {
                    name: "displayName".into(),
                },
                register_interface::ResultField {
                    name: "effectiveLabels".into(),
                },
                register_interface::ResultField {
                    name: "host".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "labels".into(),
                },
                register_interface::ResultField {
                    name: "locationId".into(),
                },
                register_interface::ResultField {
                    name: "maintenancePolicies".into(),
                },
                register_interface::ResultField {
                    name: "maintenanceSchedules".into(),
                },
                register_interface::ResultField {
                    name: "maintenanceVersion".into(),
                },
                register_interface::ResultField {
                    name: "memorySizeGb".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "nodes".into(),
                },
                register_interface::ResultField {
                    name: "persistenceConfigs".into(),
                },
                register_interface::ResultField {
                    name: "persistenceIamIdentity".into(),
                },
                register_interface::ResultField {
                    name: "port".into(),
                },
                register_interface::ResultField {
                    name: "project".into(),
                },
                register_interface::ResultField {
                    name: "pulumiLabels".into(),
                },
                register_interface::ResultField {
                    name: "readEndpoint".into(),
                },
                register_interface::ResultField {
                    name: "readEndpointPort".into(),
                },
                register_interface::ResultField {
                    name: "readReplicasMode".into(),
                },
                register_interface::ResultField {
                    name: "redisConfigs".into(),
                },
                register_interface::ResultField {
                    name: "redisVersion".into(),
                },
                register_interface::ResultField {
                    name: "region".into(),
                },
                register_interface::ResultField {
                    name: "replicaCount".into(),
                },
                register_interface::ResultField {
                    name: "reservedIpRange".into(),
                },
                register_interface::ResultField {
                    name: "secondaryIpRange".into(),
                },
                register_interface::ResultField {
                    name: "serverCaCerts".into(),
                },
                register_interface::ResultField {
                    name: "tier".into(),
                },
                register_interface::ResultField {
                    name: "transitEncryptionMode".into(),
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetInstanceResult {
            alternative_location_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("alternativeLocationId").unwrap(),
            ),
            auth_enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("authEnabled").unwrap(),
            ),
            auth_string: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("authString").unwrap(),
            ),
            authorized_network: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("authorizedNetwork").unwrap(),
            ),
            connect_mode: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("connectMode").unwrap(),
            ),
            create_time: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("createTime").unwrap(),
            ),
            current_location_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("currentLocationId").unwrap(),
            ),
            customer_managed_key: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("customerManagedKey").unwrap(),
            ),
            display_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("displayName").unwrap(),
            ),
            effective_labels: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("effectiveLabels").unwrap(),
            ),
            host: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("host").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            labels: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("labels").unwrap(),
            ),
            location_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("locationId").unwrap(),
            ),
            maintenance_policies: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("maintenancePolicies").unwrap(),
            ),
            maintenance_schedules: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("maintenanceSchedules").unwrap(),
            ),
            maintenance_version: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("maintenanceVersion").unwrap(),
            ),
            memory_size_gb: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("memorySizeGb").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            nodes: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("nodes").unwrap(),
            ),
            persistence_configs: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("persistenceConfigs").unwrap(),
            ),
            persistence_iam_identity: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("persistenceIamIdentity").unwrap(),
            ),
            port: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("port").unwrap(),
            ),
            project: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("project").unwrap(),
            ),
            pulumi_labels: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("pulumiLabels").unwrap(),
            ),
            read_endpoint: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("readEndpoint").unwrap(),
            ),
            read_endpoint_port: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("readEndpointPort").unwrap(),
            ),
            read_replicas_mode: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("readReplicasMode").unwrap(),
            ),
            redis_configs: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("redisConfigs").unwrap(),
            ),
            redis_version: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("redisVersion").unwrap(),
            ),
            region: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("region").unwrap(),
            ),
            replica_count: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("replicaCount").unwrap(),
            ),
            reserved_ip_range: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("reservedIpRange").unwrap(),
            ),
            secondary_ip_range: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("secondaryIpRange").unwrap(),
            ),
            server_ca_certs: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("serverCaCerts").unwrap(),
            ),
            tier: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tier").unwrap(),
            ),
            transit_encryption_mode: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("transitEncryptionMode").unwrap(),
            ),
        }
    }
}
