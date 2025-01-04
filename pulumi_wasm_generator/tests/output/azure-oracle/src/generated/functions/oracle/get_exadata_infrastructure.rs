pub mod get_exadata_infrastructure {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetExadataInfrastructureArgs {
        /// The name of this Cloud Exadata Infrastructure.
        #[builder(into)]
        pub name: pulumi_wasm_rust::Output<String>,
        /// The name of the Resource Group where the Cloud Exadata Infrastructure exists.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct GetExadataInfrastructureResult {
        /// The requested number of additional storage servers activated for the Cloud Exadata Infrastructure.
        pub activated_storage_count: pulumi_wasm_rust::Output<i32>,
        /// The requested number of additional storage servers for the Cloud Exadata Infrastructure.
        pub additional_storage_count: pulumi_wasm_rust::Output<i32>,
        /// The available storage can be allocated to the Cloud Exadata Infrastructure resource, in gigabytes (GB).
        pub available_storage_size_in_gbs: pulumi_wasm_rust::Output<i32>,
        /// The number of compute servers for the Cloud Exadata Infrastructure.
        pub compute_count: pulumi_wasm_rust::Output<i32>,
        /// The total number of CPU cores allocated.
        pub cpu_count: pulumi_wasm_rust::Output<i32>,
        /// A `customer_contacts` block as defined below.
        pub customer_contacts: pulumi_wasm_rust::Output<Vec<String>>,
        /// The data storage size in terabytes of the DATA disk group.
        pub data_storage_size_in_tbs: pulumi_wasm_rust::Output<f64>,
        /// The local node storage allocated in GBs.
        pub db_node_storage_size_in_gbs: pulumi_wasm_rust::Output<i32>,
        /// The software version of the database servers (dom0) in the Cloud Exadata Infrastructure.
        pub db_server_version: pulumi_wasm_rust::Output<String>,
        /// The user-friendly name for the Cloud Exadata Infrastructure resource. The name does not need to be unique.
        pub display_name: pulumi_wasm_rust::Output<String>,
        /// A `estimated_patching_time` block as defined below.
        pub estimated_patching_times: pulumi_wasm_rust::Output<
            Vec<
                super::super::super::types::oracle::GetExadataInfrastructureEstimatedPatchingTime,
            >,
        >,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        /// The [OCID](https://docs.oracle.com/en-us/iaas/Content/General/Concepts/identifiers.htm) of the last maintenance run.
        pub last_maintenance_run_id: pulumi_wasm_rust::Output<String>,
        /// Additional information about the current lifecycle state.
        pub lifecycle_details: pulumi_wasm_rust::Output<String>,
        /// Cloud Exadata Infrastructure lifecycle state.
        pub lifecycle_state: pulumi_wasm_rust::Output<String>,
        /// The Azure Region where the Cloud Exadata Infrastructure exists.
        pub location: pulumi_wasm_rust::Output<String>,
        /// A `maintenance_window` block as defined below.
        pub maintenance_windows: pulumi_wasm_rust::Output<
            Vec<
                super::super::super::types::oracle::GetExadataInfrastructureMaintenanceWindow,
            >,
        >,
        /// The total number of CPU cores available.
        pub max_cpu_count: pulumi_wasm_rust::Output<i32>,
        /// The total available DATA disk group size.
        pub max_data_storage_in_tbs: pulumi_wasm_rust::Output<f64>,
        /// The total local node storage available in GBs.
        pub max_db_node_storage_size_in_gbs: pulumi_wasm_rust::Output<i32>,
        /// The total memory available in GBs.
        pub max_memory_in_gbs: pulumi_wasm_rust::Output<i32>,
        /// The memory allocated in GBs.
        pub memory_size_in_gbs: pulumi_wasm_rust::Output<i32>,
        /// The monthly software version of the database servers (dom0) in the Cloud Exadata Infrastructure.
        pub monthly_db_server_version: pulumi_wasm_rust::Output<String>,
        /// The monthly software version of the storage servers (cells) in the Cloud Exadata Infrastructure.
        pub monthly_storage_server_version: pulumi_wasm_rust::Output<String>,
        pub name: pulumi_wasm_rust::Output<String>,
        /// The [OCID](https://docs.oracle.com/en-us/iaas/Content/General/Concepts/identifiers.htm) of the next maintenance run.
        pub next_maintenance_run_id: pulumi_wasm_rust::Output<String>,
        /// The URL of the resource in the OCI console.
        pub oci_url: pulumi_wasm_rust::Output<String>,
        /// The [OCID](https://docs.oracle.com/en-us/iaas/Content/General/Concepts/identifiers.htm) of the Cloud Exadata Infrastructure.
        pub ocid: pulumi_wasm_rust::Output<String>,
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// The model name of the Cloud Exadata Infrastructure resource.
        pub shape: pulumi_wasm_rust::Output<String>,
        /// The number of storage servers for the Cloud Exadata Infrastructure.
        pub storage_count: pulumi_wasm_rust::Output<i32>,
        /// The software version of the storage servers (cells) in the Cloud Exadata Infrastructure.
        pub storage_server_version: pulumi_wasm_rust::Output<String>,
        /// A mapping of tags assigned to the Cloud Exadata Infrastructure.
        pub tags: pulumi_wasm_rust::Output<std::collections::HashMap<String, String>>,
        /// The date and time the Cloud Exadata Infrastructure resource was created.
        pub time_created: pulumi_wasm_rust::Output<String>,
        /// The total storage allocated to the Cloud Exadata Infrastructure resource, in gigabytes (GB).
        pub total_storage_size_in_gbs: pulumi_wasm_rust::Output<i32>,
        /// The Cloud Exadata Infrastructure Azure zones.
        pub zones: pulumi_wasm_rust::Output<Vec<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(args: GetExadataInfrastructureArgs) -> GetExadataInfrastructureResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let name_binding = args.name.get_inner();
        let resource_group_name_binding = args.resource_group_name.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "azure:oracle/getExadataInfrastructure:getExadataInfrastructure"
                .into(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "activatedStorageCount".into(),
                },
                register_interface::ResultField {
                    name: "additionalStorageCount".into(),
                },
                register_interface::ResultField {
                    name: "availableStorageSizeInGbs".into(),
                },
                register_interface::ResultField {
                    name: "computeCount".into(),
                },
                register_interface::ResultField {
                    name: "cpuCount".into(),
                },
                register_interface::ResultField {
                    name: "customerContacts".into(),
                },
                register_interface::ResultField {
                    name: "dataStorageSizeInTbs".into(),
                },
                register_interface::ResultField {
                    name: "dbNodeStorageSizeInGbs".into(),
                },
                register_interface::ResultField {
                    name: "dbServerVersion".into(),
                },
                register_interface::ResultField {
                    name: "displayName".into(),
                },
                register_interface::ResultField {
                    name: "estimatedPatchingTimes".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "lastMaintenanceRunId".into(),
                },
                register_interface::ResultField {
                    name: "lifecycleDetails".into(),
                },
                register_interface::ResultField {
                    name: "lifecycleState".into(),
                },
                register_interface::ResultField {
                    name: "location".into(),
                },
                register_interface::ResultField {
                    name: "maintenanceWindows".into(),
                },
                register_interface::ResultField {
                    name: "maxCpuCount".into(),
                },
                register_interface::ResultField {
                    name: "maxDataStorageInTbs".into(),
                },
                register_interface::ResultField {
                    name: "maxDbNodeStorageSizeInGbs".into(),
                },
                register_interface::ResultField {
                    name: "maxMemoryInGbs".into(),
                },
                register_interface::ResultField {
                    name: "memorySizeInGbs".into(),
                },
                register_interface::ResultField {
                    name: "monthlyDbServerVersion".into(),
                },
                register_interface::ResultField {
                    name: "monthlyStorageServerVersion".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "nextMaintenanceRunId".into(),
                },
                register_interface::ResultField {
                    name: "ociUrl".into(),
                },
                register_interface::ResultField {
                    name: "ocid".into(),
                },
                register_interface::ResultField {
                    name: "resourceGroupName".into(),
                },
                register_interface::ResultField {
                    name: "shape".into(),
                },
                register_interface::ResultField {
                    name: "storageCount".into(),
                },
                register_interface::ResultField {
                    name: "storageServerVersion".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "timeCreated".into(),
                },
                register_interface::ResultField {
                    name: "totalStorageSizeInGbs".into(),
                },
                register_interface::ResultField {
                    name: "zones".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetExadataInfrastructureResult {
            activated_storage_count: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("activatedStorageCount").unwrap(),
            ),
            additional_storage_count: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("additionalStorageCount").unwrap(),
            ),
            available_storage_size_in_gbs: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("availableStorageSizeInGbs").unwrap(),
            ),
            compute_count: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("computeCount").unwrap(),
            ),
            cpu_count: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("cpuCount").unwrap(),
            ),
            customer_contacts: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("customerContacts").unwrap(),
            ),
            data_storage_size_in_tbs: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("dataStorageSizeInTbs").unwrap(),
            ),
            db_node_storage_size_in_gbs: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("dbNodeStorageSizeInGbs").unwrap(),
            ),
            db_server_version: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("dbServerVersion").unwrap(),
            ),
            display_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("displayName").unwrap(),
            ),
            estimated_patching_times: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("estimatedPatchingTimes").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            last_maintenance_run_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("lastMaintenanceRunId").unwrap(),
            ),
            lifecycle_details: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("lifecycleDetails").unwrap(),
            ),
            lifecycle_state: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("lifecycleState").unwrap(),
            ),
            location: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("location").unwrap(),
            ),
            maintenance_windows: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("maintenanceWindows").unwrap(),
            ),
            max_cpu_count: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("maxCpuCount").unwrap(),
            ),
            max_data_storage_in_tbs: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("maxDataStorageInTbs").unwrap(),
            ),
            max_db_node_storage_size_in_gbs: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("maxDbNodeStorageSizeInGbs").unwrap(),
            ),
            max_memory_in_gbs: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("maxMemoryInGbs").unwrap(),
            ),
            memory_size_in_gbs: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("memorySizeInGbs").unwrap(),
            ),
            monthly_db_server_version: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("monthlyDbServerVersion").unwrap(),
            ),
            monthly_storage_server_version: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("monthlyStorageServerVersion").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            next_maintenance_run_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("nextMaintenanceRunId").unwrap(),
            ),
            oci_url: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("ociUrl").unwrap(),
            ),
            ocid: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("ocid").unwrap(),
            ),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceGroupName").unwrap(),
            ),
            shape: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("shape").unwrap(),
            ),
            storage_count: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("storageCount").unwrap(),
            ),
            storage_server_version: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("storageServerVersion").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            time_created: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("timeCreated").unwrap(),
            ),
            total_storage_size_in_gbs: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("totalStorageSizeInGbs").unwrap(),
            ),
            zones: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("zones").unwrap(),
            ),
        }
    }
}
