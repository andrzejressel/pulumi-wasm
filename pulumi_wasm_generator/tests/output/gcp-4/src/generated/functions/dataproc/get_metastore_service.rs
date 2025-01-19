pub mod get_metastore_service {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetMetastoreServiceArgs {
        /// The location where the metastore service resides.
        ///
        /// - - -
        #[builder(into)]
        pub location: pulumi_wasm_rust::Output<String>,
        /// The project in which the resource belongs. If it
        /// is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_wasm_rust::Output<Option<String>>,
        /// The ID of the metastore service.
        #[builder(into)]
        pub service_id: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct GetMetastoreServiceResult {
        pub artifact_gcs_uri: pulumi_wasm_rust::Output<String>,
        pub database_type: pulumi_wasm_rust::Output<String>,
        pub deletion_protection: pulumi_wasm_rust::Output<bool>,
        pub effective_labels: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        pub encryption_configs: pulumi_wasm_rust::Output<
            Vec<
                super::super::super::types::dataproc::GetMetastoreServiceEncryptionConfig,
            >,
        >,
        pub endpoint_uri: pulumi_wasm_rust::Output<String>,
        pub hive_metastore_configs: pulumi_wasm_rust::Output<
            Vec<
                super::super::super::types::dataproc::GetMetastoreServiceHiveMetastoreConfig,
            >,
        >,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        pub labels: pulumi_wasm_rust::Output<std::collections::HashMap<String, String>>,
        pub location: pulumi_wasm_rust::Output<String>,
        pub maintenance_windows: pulumi_wasm_rust::Output<
            Vec<
                super::super::super::types::dataproc::GetMetastoreServiceMaintenanceWindow,
            >,
        >,
        pub metadata_integrations: pulumi_wasm_rust::Output<
            Vec<
                super::super::super::types::dataproc::GetMetastoreServiceMetadataIntegration,
            >,
        >,
        pub name: pulumi_wasm_rust::Output<String>,
        pub network: pulumi_wasm_rust::Output<String>,
        pub network_configs: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::dataproc::GetMetastoreServiceNetworkConfig>,
        >,
        pub port: pulumi_wasm_rust::Output<i32>,
        pub project: pulumi_wasm_rust::Output<Option<String>>,
        pub pulumi_labels: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        pub release_channel: pulumi_wasm_rust::Output<String>,
        pub scaling_configs: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::dataproc::GetMetastoreServiceScalingConfig>,
        >,
        pub scheduled_backups: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::dataproc::GetMetastoreServiceScheduledBackup>,
        >,
        pub service_id: pulumi_wasm_rust::Output<String>,
        pub state: pulumi_wasm_rust::Output<String>,
        pub state_message: pulumi_wasm_rust::Output<String>,
        pub telemetry_configs: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::dataproc::GetMetastoreServiceTelemetryConfig>,
        >,
        pub tier: pulumi_wasm_rust::Output<String>,
        pub uid: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(args: GetMetastoreServiceArgs) -> GetMetastoreServiceResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let location_binding = args.location.get_inner();
        let project_binding = args.project.get_inner();
        let service_id_binding = args.service_id.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "gcp:dataproc/getMetastoreService:getMetastoreService".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "location".into(),
                    value: &location_binding,
                },
                register_interface::ObjectField {
                    name: "project".into(),
                    value: &project_binding,
                },
                register_interface::ObjectField {
                    name: "serviceId".into(),
                    value: &service_id_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "artifactGcsUri".into(),
                },
                register_interface::ResultField {
                    name: "databaseType".into(),
                },
                register_interface::ResultField {
                    name: "deletionProtection".into(),
                },
                register_interface::ResultField {
                    name: "effectiveLabels".into(),
                },
                register_interface::ResultField {
                    name: "encryptionConfigs".into(),
                },
                register_interface::ResultField {
                    name: "endpointUri".into(),
                },
                register_interface::ResultField {
                    name: "hiveMetastoreConfigs".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "labels".into(),
                },
                register_interface::ResultField {
                    name: "location".into(),
                },
                register_interface::ResultField {
                    name: "maintenanceWindows".into(),
                },
                register_interface::ResultField {
                    name: "metadataIntegrations".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "network".into(),
                },
                register_interface::ResultField {
                    name: "networkConfigs".into(),
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
                    name: "releaseChannel".into(),
                },
                register_interface::ResultField {
                    name: "scalingConfigs".into(),
                },
                register_interface::ResultField {
                    name: "scheduledBackups".into(),
                },
                register_interface::ResultField {
                    name: "serviceId".into(),
                },
                register_interface::ResultField {
                    name: "state".into(),
                },
                register_interface::ResultField {
                    name: "stateMessage".into(),
                },
                register_interface::ResultField {
                    name: "telemetryConfigs".into(),
                },
                register_interface::ResultField {
                    name: "tier".into(),
                },
                register_interface::ResultField {
                    name: "uid".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetMetastoreServiceResult {
            artifact_gcs_uri: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("artifactGcsUri").unwrap(),
            ),
            database_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("databaseType").unwrap(),
            ),
            deletion_protection: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("deletionProtection").unwrap(),
            ),
            effective_labels: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("effectiveLabels").unwrap(),
            ),
            encryption_configs: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("encryptionConfigs").unwrap(),
            ),
            endpoint_uri: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("endpointUri").unwrap(),
            ),
            hive_metastore_configs: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("hiveMetastoreConfigs").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            labels: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("labels").unwrap(),
            ),
            location: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("location").unwrap(),
            ),
            maintenance_windows: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("maintenanceWindows").unwrap(),
            ),
            metadata_integrations: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("metadataIntegrations").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            network: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("network").unwrap(),
            ),
            network_configs: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("networkConfigs").unwrap(),
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
            release_channel: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("releaseChannel").unwrap(),
            ),
            scaling_configs: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("scalingConfigs").unwrap(),
            ),
            scheduled_backups: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("scheduledBackups").unwrap(),
            ),
            service_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("serviceId").unwrap(),
            ),
            state: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("state").unwrap(),
            ),
            state_message: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("stateMessage").unwrap(),
            ),
            telemetry_configs: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("telemetryConfigs").unwrap(),
            ),
            tier: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tier").unwrap(),
            ),
            uid: pulumi_wasm_rust::__private::into_domain(hashmap.remove("uid").unwrap()),
        }
    }
}
