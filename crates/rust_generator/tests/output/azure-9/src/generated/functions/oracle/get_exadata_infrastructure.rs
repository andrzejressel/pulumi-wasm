#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_exadata_infrastructure {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetExadataInfrastructureArgs {
        /// The name of this Cloud Exadata Infrastructure.
        #[builder(into)]
        pub name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the Resource Group where the Cloud Exadata Infrastructure exists.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetExadataInfrastructureResult {
        /// The requested number of additional storage servers activated for the Cloud Exadata Infrastructure.
        pub activated_storage_count: pulumi_gestalt_rust::Output<i32>,
        /// The requested number of additional storage servers for the Cloud Exadata Infrastructure.
        pub additional_storage_count: pulumi_gestalt_rust::Output<i32>,
        /// The available storage can be allocated to the Cloud Exadata Infrastructure resource, in gigabytes (GB).
        pub available_storage_size_in_gbs: pulumi_gestalt_rust::Output<i32>,
        /// The number of compute servers for the Cloud Exadata Infrastructure.
        pub compute_count: pulumi_gestalt_rust::Output<i32>,
        /// The total number of CPU cores allocated.
        pub cpu_count: pulumi_gestalt_rust::Output<i32>,
        /// A `customer_contacts` block as defined below.
        pub customer_contacts: pulumi_gestalt_rust::Output<Vec<String>>,
        /// The data storage size in terabytes of the DATA disk group.
        pub data_storage_size_in_tbs: pulumi_gestalt_rust::Output<f64>,
        /// The local node storage allocated in GBs.
        pub db_node_storage_size_in_gbs: pulumi_gestalt_rust::Output<i32>,
        /// The software version of the database servers (dom0) in the Cloud Exadata Infrastructure.
        pub db_server_version: pulumi_gestalt_rust::Output<String>,
        /// The user-friendly name for the Cloud Exadata Infrastructure resource. The name does not need to be unique.
        pub display_name: pulumi_gestalt_rust::Output<String>,
        /// A `estimated_patching_time` block as defined below.
        pub estimated_patching_times: pulumi_gestalt_rust::Output<
            Vec<
                super::super::super::types::oracle::GetExadataInfrastructureEstimatedPatchingTime,
            >,
        >,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// The [OCID](https://docs.oracle.com/en-us/iaas/Content/General/Concepts/identifiers.htm) of the last maintenance run.
        pub last_maintenance_run_id: pulumi_gestalt_rust::Output<String>,
        /// Additional information about the current lifecycle state.
        pub lifecycle_details: pulumi_gestalt_rust::Output<String>,
        /// Cloud Exadata Infrastructure lifecycle state.
        pub lifecycle_state: pulumi_gestalt_rust::Output<String>,
        /// The Azure Region where the Cloud Exadata Infrastructure exists.
        pub location: pulumi_gestalt_rust::Output<String>,
        /// A `maintenance_window` block as defined below.
        pub maintenance_windows: pulumi_gestalt_rust::Output<
            Vec<
                super::super::super::types::oracle::GetExadataInfrastructureMaintenanceWindow,
            >,
        >,
        /// The total number of CPU cores available.
        pub max_cpu_count: pulumi_gestalt_rust::Output<i32>,
        /// The total available DATA disk group size.
        pub max_data_storage_in_tbs: pulumi_gestalt_rust::Output<f64>,
        /// The total local node storage available in GBs.
        pub max_db_node_storage_size_in_gbs: pulumi_gestalt_rust::Output<i32>,
        /// The total memory available in GBs.
        pub max_memory_in_gbs: pulumi_gestalt_rust::Output<i32>,
        /// The memory allocated in GBs.
        pub memory_size_in_gbs: pulumi_gestalt_rust::Output<i32>,
        /// The monthly software version of the database servers (dom0) in the Cloud Exadata Infrastructure.
        pub monthly_db_server_version: pulumi_gestalt_rust::Output<String>,
        /// The monthly software version of the storage servers (cells) in the Cloud Exadata Infrastructure.
        pub monthly_storage_server_version: pulumi_gestalt_rust::Output<String>,
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The [OCID](https://docs.oracle.com/en-us/iaas/Content/General/Concepts/identifiers.htm) of the next maintenance run.
        pub next_maintenance_run_id: pulumi_gestalt_rust::Output<String>,
        /// The URL of the resource in the OCI console.
        pub oci_url: pulumi_gestalt_rust::Output<String>,
        /// The [OCID](https://docs.oracle.com/en-us/iaas/Content/General/Concepts/identifiers.htm) of the Cloud Exadata Infrastructure.
        pub ocid: pulumi_gestalt_rust::Output<String>,
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// The model name of the Cloud Exadata Infrastructure resource.
        pub shape: pulumi_gestalt_rust::Output<String>,
        /// The number of storage servers for the Cloud Exadata Infrastructure.
        pub storage_count: pulumi_gestalt_rust::Output<i32>,
        /// The software version of the storage servers (cells) in the Cloud Exadata Infrastructure.
        pub storage_server_version: pulumi_gestalt_rust::Output<String>,
        /// A mapping of tags assigned to the Cloud Exadata Infrastructure.
        pub tags: pulumi_gestalt_rust::Output<std::collections::HashMap<String, String>>,
        /// The date and time the Cloud Exadata Infrastructure resource was created.
        pub time_created: pulumi_gestalt_rust::Output<String>,
        /// The total storage allocated to the Cloud Exadata Infrastructure resource, in gigabytes (GB).
        pub total_storage_size_in_gbs: pulumi_gestalt_rust::Output<i32>,
        /// The Cloud Exadata Infrastructure Azure zones.
        pub zones: pulumi_gestalt_rust::Output<Vec<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetExadataInfrastructureArgs,
    ) -> GetExadataInfrastructureResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let name_binding = args.name.get_output(context);
        let resource_group_name_binding = args.resource_group_name.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "azure:oracle/getExadataInfrastructure:getExadataInfrastructure"
                .into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding.drop_type(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetExadataInfrastructureResult {
            activated_storage_count: o.get_field("activatedStorageCount"),
            additional_storage_count: o.get_field("additionalStorageCount"),
            available_storage_size_in_gbs: o.get_field("availableStorageSizeInGbs"),
            compute_count: o.get_field("computeCount"),
            cpu_count: o.get_field("cpuCount"),
            customer_contacts: o.get_field("customerContacts"),
            data_storage_size_in_tbs: o.get_field("dataStorageSizeInTbs"),
            db_node_storage_size_in_gbs: o.get_field("dbNodeStorageSizeInGbs"),
            db_server_version: o.get_field("dbServerVersion"),
            display_name: o.get_field("displayName"),
            estimated_patching_times: o.get_field("estimatedPatchingTimes"),
            id: o.get_field("id"),
            last_maintenance_run_id: o.get_field("lastMaintenanceRunId"),
            lifecycle_details: o.get_field("lifecycleDetails"),
            lifecycle_state: o.get_field("lifecycleState"),
            location: o.get_field("location"),
            maintenance_windows: o.get_field("maintenanceWindows"),
            max_cpu_count: o.get_field("maxCpuCount"),
            max_data_storage_in_tbs: o.get_field("maxDataStorageInTbs"),
            max_db_node_storage_size_in_gbs: o.get_field("maxDbNodeStorageSizeInGbs"),
            max_memory_in_gbs: o.get_field("maxMemoryInGbs"),
            memory_size_in_gbs: o.get_field("memorySizeInGbs"),
            monthly_db_server_version: o.get_field("monthlyDbServerVersion"),
            monthly_storage_server_version: o.get_field("monthlyStorageServerVersion"),
            name: o.get_field("name"),
            next_maintenance_run_id: o.get_field("nextMaintenanceRunId"),
            oci_url: o.get_field("ociUrl"),
            ocid: o.get_field("ocid"),
            resource_group_name: o.get_field("resourceGroupName"),
            shape: o.get_field("shape"),
            storage_count: o.get_field("storageCount"),
            storage_server_version: o.get_field("storageServerVersion"),
            tags: o.get_field("tags"),
            time_created: o.get_field("timeCreated"),
            total_storage_size_in_gbs: o.get_field("totalStorageSizeInGbs"),
            zones: o.get_field("zones"),
        }
    }
}
