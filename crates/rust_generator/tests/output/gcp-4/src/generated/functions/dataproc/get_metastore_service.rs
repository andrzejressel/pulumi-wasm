#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
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
        context: &pulumi_gestalt_rust::Context,
        args: GetMetastoreServiceArgs,
    ) -> GetMetastoreServiceResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let location_binding = args.location.get_output(context);
        let project_binding = args.project.get_output(context);
        let service_id_binding = args.service_id.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "gcp:dataproc/getMetastoreService:getMetastoreService".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "location".into(),
                    value: location_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "project".into(),
                    value: project_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "serviceId".into(),
                    value: service_id_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetMetastoreServiceResult {
            artifact_gcs_uri: o.get_field("artifactGcsUri"),
            database_type: o.get_field("databaseType"),
            deletion_protection: o.get_field("deletionProtection"),
            effective_labels: o.get_field("effectiveLabels"),
            encryption_configs: o.get_field("encryptionConfigs"),
            endpoint_uri: o.get_field("endpointUri"),
            hive_metastore_configs: o.get_field("hiveMetastoreConfigs"),
            id: o.get_field("id"),
            labels: o.get_field("labels"),
            location: o.get_field("location"),
            maintenance_windows: o.get_field("maintenanceWindows"),
            metadata_integrations: o.get_field("metadataIntegrations"),
            name: o.get_field("name"),
            network: o.get_field("network"),
            network_configs: o.get_field("networkConfigs"),
            port: o.get_field("port"),
            project: o.get_field("project"),
            pulumi_labels: o.get_field("pulumiLabels"),
            release_channel: o.get_field("releaseChannel"),
            scaling_configs: o.get_field("scalingConfigs"),
            scheduled_backups: o.get_field("scheduledBackups"),
            service_id: o.get_field("serviceId"),
            state: o.get_field("state"),
            state_message: o.get_field("stateMessage"),
            telemetry_configs: o.get_field("telemetryConfigs"),
            tier: o.get_field("tier"),
            uid: o.get_field("uid"),
        }
    }
}
