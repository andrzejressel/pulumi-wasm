#[allow(clippy::doc_lazy_continuation)]
pub mod get_metastore_service {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetMetastoreServiceArgs {
        /// The location where the metastore service resides.
        ///
        /// - - -
        #[builder(into)]
        pub location: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The project in which the resource belongs. If it
        /// is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ID of the metastore service.
        #[builder(into)]
        pub service_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetMetastoreServiceResult {
        pub artifact_gcs_uri: pulumi_gestalt_rust::Output<String>,
        pub database_type: pulumi_gestalt_rust::Output<String>,
        pub deletion_protection: pulumi_gestalt_rust::Output<bool>,
        pub effective_labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        pub encryption_configs: pulumi_gestalt_rust::Output<
            Vec<
                super::super::super::types::dataproc::GetMetastoreServiceEncryptionConfig,
            >,
        >,
        pub endpoint_uri: pulumi_gestalt_rust::Output<String>,
        pub hive_metastore_configs: pulumi_gestalt_rust::Output<
            Vec<
                super::super::super::types::dataproc::GetMetastoreServiceHiveMetastoreConfig,
            >,
        >,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        pub labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        pub location: pulumi_gestalt_rust::Output<String>,
        pub maintenance_windows: pulumi_gestalt_rust::Output<
            Vec<
                super::super::super::types::dataproc::GetMetastoreServiceMaintenanceWindow,
            >,
        >,
        pub metadata_integrations: pulumi_gestalt_rust::Output<
            Vec<
                super::super::super::types::dataproc::GetMetastoreServiceMetadataIntegration,
            >,
        >,
        pub name: pulumi_gestalt_rust::Output<String>,
        pub network: pulumi_gestalt_rust::Output<String>,
        pub network_configs: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::dataproc::GetMetastoreServiceNetworkConfig>,
        >,
        pub port: pulumi_gestalt_rust::Output<i32>,
        pub project: pulumi_gestalt_rust::Output<Option<String>>,
        pub pulumi_labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        pub release_channel: pulumi_gestalt_rust::Output<String>,
        pub scaling_configs: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::dataproc::GetMetastoreServiceScalingConfig>,
        >,
        pub scheduled_backups: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::dataproc::GetMetastoreServiceScheduledBackup>,
        >,
        pub service_id: pulumi_gestalt_rust::Output<String>,
        pub state: pulumi_gestalt_rust::Output<String>,
        pub state_message: pulumi_gestalt_rust::Output<String>,
        pub telemetry_configs: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::dataproc::GetMetastoreServiceTelemetryConfig>,
        >,
        pub tier: pulumi_gestalt_rust::Output<String>,
        pub uid: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::PulumiContext,
        args: GetMetastoreServiceArgs,
    ) -> GetMetastoreServiceResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let location_binding = args.location.get_output(context).get_inner();
        let project_binding = args.project.get_output(context).get_inner();
        let service_id_binding = args.service_id.get_output(context).get_inner();
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
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetMetastoreServiceResult {
            artifact_gcs_uri: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("artifactGcsUri"),
            ),
            database_type: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("databaseType"),
            ),
            deletion_protection: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("deletionProtection"),
            ),
            effective_labels: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("effectiveLabels"),
            ),
            encryption_configs: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("encryptionConfigs"),
            ),
            endpoint_uri: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("endpointUri"),
            ),
            hive_metastore_configs: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("hiveMetastoreConfigs"),
            ),
            id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("id")),
            labels: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("labels"),
            ),
            location: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("location"),
            ),
            maintenance_windows: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("maintenanceWindows"),
            ),
            metadata_integrations: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("metadataIntegrations"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            network: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("network"),
            ),
            network_configs: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("networkConfigs"),
            ),
            port: pulumi_gestalt_rust::__private::into_domain(o.extract_field("port")),
            project: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("project"),
            ),
            pulumi_labels: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("pulumiLabels"),
            ),
            release_channel: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("releaseChannel"),
            ),
            scaling_configs: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("scalingConfigs"),
            ),
            scheduled_backups: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("scheduledBackups"),
            ),
            service_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("serviceId"),
            ),
            state: pulumi_gestalt_rust::__private::into_domain(o.extract_field("state")),
            state_message: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("stateMessage"),
            ),
            telemetry_configs: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("telemetryConfigs"),
            ),
            tier: pulumi_gestalt_rust::__private::into_domain(o.extract_field("tier")),
            uid: pulumi_gestalt_rust::__private::into_domain(o.extract_field("uid")),
        }
    }
}
