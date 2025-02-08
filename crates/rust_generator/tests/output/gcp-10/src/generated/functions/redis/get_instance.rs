#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_instance {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetInstanceArgs {
        /// The name of a Redis instance.
        ///
        /// - - -
        #[builder(into)]
        pub name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The project in which the resource belongs. If it
        /// is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The region in which the resource belongs. If it
        /// is not provided, the provider region is used.
        #[builder(into, default)]
        pub region: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetInstanceResult {
        pub alternative_location_id: pulumi_gestalt_rust::Output<String>,
        pub auth_enabled: pulumi_gestalt_rust::Output<bool>,
        pub auth_string: pulumi_gestalt_rust::Output<String>,
        pub authorized_network: pulumi_gestalt_rust::Output<String>,
        pub connect_mode: pulumi_gestalt_rust::Output<String>,
        pub create_time: pulumi_gestalt_rust::Output<String>,
        pub current_location_id: pulumi_gestalt_rust::Output<String>,
        pub customer_managed_key: pulumi_gestalt_rust::Output<String>,
        pub display_name: pulumi_gestalt_rust::Output<String>,
        pub effective_labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        pub host: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        pub labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        pub location_id: pulumi_gestalt_rust::Output<String>,
        pub maintenance_policies: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::redis::GetInstanceMaintenancePolicy>,
        >,
        pub maintenance_schedules: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::redis::GetInstanceMaintenanceSchedule>,
        >,
        pub maintenance_version: pulumi_gestalt_rust::Output<String>,
        pub memory_size_gb: pulumi_gestalt_rust::Output<i32>,
        pub name: pulumi_gestalt_rust::Output<String>,
        pub nodes: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::redis::GetInstanceNode>,
        >,
        pub persistence_configs: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::redis::GetInstancePersistenceConfig>,
        >,
        pub persistence_iam_identity: pulumi_gestalt_rust::Output<String>,
        pub port: pulumi_gestalt_rust::Output<i32>,
        pub project: pulumi_gestalt_rust::Output<Option<String>>,
        pub pulumi_labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        pub read_endpoint: pulumi_gestalt_rust::Output<String>,
        pub read_endpoint_port: pulumi_gestalt_rust::Output<i32>,
        pub read_replicas_mode: pulumi_gestalt_rust::Output<String>,
        pub redis_configs: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        pub redis_version: pulumi_gestalt_rust::Output<String>,
        pub region: pulumi_gestalt_rust::Output<Option<String>>,
        pub replica_count: pulumi_gestalt_rust::Output<i32>,
        pub reserved_ip_range: pulumi_gestalt_rust::Output<String>,
        pub secondary_ip_range: pulumi_gestalt_rust::Output<String>,
        pub server_ca_certs: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::redis::GetInstanceServerCaCert>,
        >,
        pub tier: pulumi_gestalt_rust::Output<String>,
        pub transit_encryption_mode: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::PulumiContext,
        args: GetInstanceArgs,
    ) -> GetInstanceResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
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
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetInstanceResult {
            alternative_location_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("alternativeLocationId"),
            ),
            auth_enabled: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("authEnabled"),
            ),
            auth_string: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("authString"),
            ),
            authorized_network: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("authorizedNetwork"),
            ),
            connect_mode: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("connectMode"),
            ),
            create_time: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("createTime"),
            ),
            current_location_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("currentLocationId"),
            ),
            customer_managed_key: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("customerManagedKey"),
            ),
            display_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("displayName"),
            ),
            effective_labels: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("effectiveLabels"),
            ),
            host: pulumi_gestalt_rust::__private::into_domain(o.extract_field("host")),
            id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("id")),
            labels: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("labels"),
            ),
            location_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("locationId"),
            ),
            maintenance_policies: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("maintenancePolicies"),
            ),
            maintenance_schedules: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("maintenanceSchedules"),
            ),
            maintenance_version: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("maintenanceVersion"),
            ),
            memory_size_gb: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("memorySizeGb"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            nodes: pulumi_gestalt_rust::__private::into_domain(o.extract_field("nodes")),
            persistence_configs: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("persistenceConfigs"),
            ),
            persistence_iam_identity: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("persistenceIamIdentity"),
            ),
            port: pulumi_gestalt_rust::__private::into_domain(o.extract_field("port")),
            project: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("project"),
            ),
            pulumi_labels: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("pulumiLabels"),
            ),
            read_endpoint: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("readEndpoint"),
            ),
            read_endpoint_port: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("readEndpointPort"),
            ),
            read_replicas_mode: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("readReplicasMode"),
            ),
            redis_configs: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("redisConfigs"),
            ),
            redis_version: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("redisVersion"),
            ),
            region: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("region"),
            ),
            replica_count: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("replicaCount"),
            ),
            reserved_ip_range: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("reservedIpRange"),
            ),
            secondary_ip_range: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("secondaryIpRange"),
            ),
            server_ca_certs: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("serverCaCerts"),
            ),
            tier: pulumi_gestalt_rust::__private::into_domain(o.extract_field("tier")),
            transit_encryption_mode: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("transitEncryptionMode"),
            ),
        }
    }
}
