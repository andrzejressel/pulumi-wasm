/// Manages a Cloud VM Cluster.
///
/// ## Example Usage
///
///
/// ## Import
///
/// Cloud VM Clusters can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:oracle/cloudVmCluster:CloudVmCluster example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/resourceGroup/providers/Oracle.Database/cloudVmClusters/cloudVmClusters1
/// ```
///
pub mod cloud_vm_cluster {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct CloudVmClusterArgs {
        /// The backup subnet CIDR of the Virtual Network associated with the Cloud VM Cluster. Changing this forces a new Cloud VM Cluster to be created.
        #[builder(into, default)]
        pub backup_subnet_cidr: pulumi_wasm_rust::Output<Option<String>>,
        /// The [OCID](https://docs.cloud.oracle.com/iaas/Content/General/Concepts/identifiers.htm) of the Cloud Exadata infrastructure. Changing this forces a new Cloud VM Cluster to be created.
        #[builder(into)]
        pub cloud_exadata_infrastructure_id: pulumi_wasm_rust::Output<String>,
        /// The cluster name for Cloud VM Cluster. Changing this forces a new Cloud VM Cluster to be created.
        #[builder(into, default)]
        pub cluster_name: pulumi_wasm_rust::Output<Option<String>>,
        /// The number of CPU cores enabled on the Cloud VM Cluster. Changing this forces a new Cloud VM Cluster to be created.
        #[builder(into)]
        pub cpu_core_count: pulumi_wasm_rust::Output<i32>,
        /// A `data_collection_options` block as defined below. Changing this forces a new Cloud VM Cluster to be created.
        #[builder(into, default)]
        pub data_collection_options: pulumi_wasm_rust::Output<
            Option<super::super::types::oracle::CloudVmClusterDataCollectionOptions>,
        >,
        /// The percentage assigned to DATA storage (user data and database files). Changing this forces a new Cloud VM Cluster to be created. The remaining percentage is assigned to RECO storage (database redo logs, archive logs, and recovery manager backups). Accepted values are `35`, `40`, `60` and `80`.
        #[builder(into, default)]
        pub data_storage_percentage: pulumi_wasm_rust::Output<Option<i32>>,
        /// The data disk group size to be allocated in TBs. Changing this forces a new Cloud VM Cluster to be created.
        #[builder(into, default)]
        pub data_storage_size_in_tbs: pulumi_wasm_rust::Output<Option<f64>>,
        /// The local node storage to be allocated in GBs. Changing this forces a new Cloud VM Cluster to be created.
        #[builder(into, default)]
        pub db_node_storage_size_in_gbs: pulumi_wasm_rust::Output<Option<i32>>,
        /// The list of DB servers. Changing this forces a new Cloud VM Cluster to be created.
        #[builder(into)]
        pub db_servers: pulumi_wasm_rust::Output<Vec<String>>,
        /// The user-friendly name for the Cloud VM Cluster. Changing this forces a new Cloud VM Cluster to be created. The name does not need to be unique.
        #[builder(into)]
        pub display_name: pulumi_wasm_rust::Output<String>,
        /// The name of the OCI Private DNS Zone to be associated with the Cloud VM Cluster. This is required for specifying your own private domain name. Changing this forces a new Cloud VM Cluster to be created.
        #[builder(into, default)]
        pub domain: pulumi_wasm_rust::Output<Option<String>>,
        /// A valid Oracle Grid Infrastructure (GI) software version. Changing this forces a new Cloud VM Cluster to be created.
        #[builder(into)]
        pub gi_version: pulumi_wasm_rust::Output<String>,
        /// The hostname for the Cloud VM Cluster without suffix. Changing this forces a new Cloud VM Cluster to be created.
        #[builder(into)]
        pub hostname: pulumi_wasm_rust::Output<String>,
        /// The Oracle license model that applies to the Cloud VM Cluster, either `BringYourOwnLicense` or `LicenseIncluded`. Changing this forces a new Cloud VM Cluster to be created.
        #[builder(into)]
        pub license_model: pulumi_wasm_rust::Output<String>,
        /// If true, database backup on local Exadata storage is configured for the Cloud VM Cluster. If `false`, database backup on local Exadata storage is not available in the Cloud VM Cluster. Changing this forces a new Cloud VM Cluster to be created.
        #[builder(into, default)]
        pub local_backup_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// The Azure Region where the Cloud VM Cluster should exist. Changing this forces a new Cloud VM Cluster to be created.
        #[builder(into, default)]
        pub location: pulumi_wasm_rust::Output<Option<String>>,
        /// The memory to be allocated in GBs. Changing this forces a new Cloud VM Cluster to be created.
        #[builder(into, default)]
        pub memory_size_in_gbs: pulumi_wasm_rust::Output<Option<i32>>,
        /// The name which should be used for this Cloud VM Cluster. Changing this forces a new Cloud VM Cluster to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of the Resource Group where the Cloud VM Cluster should exist. Changing this forces a new Cloud VM Cluster to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// The TCP Single Client Access Name (SCAN) port. The default port to 1521. Changing this forces a new Cloud VM Cluster to be created.
        #[builder(into, default)]
        pub scan_listener_port_tcp: pulumi_wasm_rust::Output<Option<i32>>,
        /// The TCPS Single Client Access Name (SCAN) port. The default port to 2484. Changing this forces a new Cloud VM Cluster to be created.
        #[builder(into, default)]
        pub scan_listener_port_tcp_ssl: pulumi_wasm_rust::Output<Option<i32>>,
        /// If true, the sparse disk group is configured for the Cloud VM Cluster. If `false`, the sparse disk group is not created. Changing this forces a new Cloud VM Cluster to be created.
        #[builder(into, default)]
        pub sparse_diskgroup_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// The public key portion of one or more key pairs used for SSH access to the Cloud VM Cluster. Changing this forces a new Cloud VM Cluster to be created.
        #[builder(into)]
        pub ssh_public_keys: pulumi_wasm_rust::Output<Vec<String>>,
        /// The ID of the subnet associated with the Cloud VM Cluster. Changing this forces a new Cloud VM Cluster to be created.
        #[builder(into)]
        pub subnet_id: pulumi_wasm_rust::Output<String>,
        /// A mapping of tags which should be assigned to the Cloud VM Cluster.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The time zone of the Cloud VM Cluster. For details, see [Exadata Infrastructure Time Zones](https://docs.cloud.oracle.com/iaas/Content/Database/References/timezones.htm). Changing this forces a new Cloud VM Cluster to be created.
        #[builder(into, default)]
        pub time_zone: pulumi_wasm_rust::Output<Option<String>>,
        /// The ID of the Virtual Network associated with the Cloud VM Cluster. Changing this forces a new Cloud VM Cluster to be created.
        #[builder(into)]
        pub virtual_network_id: pulumi_wasm_rust::Output<String>,
        /// The OCID of the OCI Private DNS Zone to be associated with the Cloud VM Cluster. This is required for specifying your own private domain name. Changing this forces a new Cloud VM Cluster to be created.
        #[builder(into, default)]
        pub zone_id: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct CloudVmClusterResult {
        /// The backup subnet CIDR of the Virtual Network associated with the Cloud VM Cluster. Changing this forces a new Cloud VM Cluster to be created.
        pub backup_subnet_cidr: pulumi_wasm_rust::Output<Option<String>>,
        /// The [OCID](https://docs.cloud.oracle.com/iaas/Content/General/Concepts/identifiers.htm) of the Cloud Exadata infrastructure. Changing this forces a new Cloud VM Cluster to be created.
        pub cloud_exadata_infrastructure_id: pulumi_wasm_rust::Output<String>,
        /// The cluster name for Cloud VM Cluster. Changing this forces a new Cloud VM Cluster to be created.
        pub cluster_name: pulumi_wasm_rust::Output<String>,
        /// The number of CPU cores enabled on the Cloud VM Cluster. Changing this forces a new Cloud VM Cluster to be created.
        pub cpu_core_count: pulumi_wasm_rust::Output<i32>,
        /// A `data_collection_options` block as defined below. Changing this forces a new Cloud VM Cluster to be created.
        pub data_collection_options: pulumi_wasm_rust::Output<
            super::super::types::oracle::CloudVmClusterDataCollectionOptions,
        >,
        /// The percentage assigned to DATA storage (user data and database files). Changing this forces a new Cloud VM Cluster to be created. The remaining percentage is assigned to RECO storage (database redo logs, archive logs, and recovery manager backups). Accepted values are `35`, `40`, `60` and `80`.
        pub data_storage_percentage: pulumi_wasm_rust::Output<i32>,
        /// The data disk group size to be allocated in TBs. Changing this forces a new Cloud VM Cluster to be created.
        pub data_storage_size_in_tbs: pulumi_wasm_rust::Output<f64>,
        /// The local node storage to be allocated in GBs. Changing this forces a new Cloud VM Cluster to be created.
        pub db_node_storage_size_in_gbs: pulumi_wasm_rust::Output<i32>,
        /// The list of DB servers. Changing this forces a new Cloud VM Cluster to be created.
        pub db_servers: pulumi_wasm_rust::Output<Vec<String>>,
        /// The user-friendly name for the Cloud VM Cluster. Changing this forces a new Cloud VM Cluster to be created. The name does not need to be unique.
        pub display_name: pulumi_wasm_rust::Output<String>,
        /// The name of the OCI Private DNS Zone to be associated with the Cloud VM Cluster. This is required for specifying your own private domain name. Changing this forces a new Cloud VM Cluster to be created.
        pub domain: pulumi_wasm_rust::Output<String>,
        /// A valid Oracle Grid Infrastructure (GI) software version. Changing this forces a new Cloud VM Cluster to be created.
        pub gi_version: pulumi_wasm_rust::Output<String>,
        /// The hostname for the Cloud VM Cluster without suffix. Changing this forces a new Cloud VM Cluster to be created.
        pub hostname: pulumi_wasm_rust::Output<String>,
        /// The hostname for the Cloud VM Cluster with suffix.
        pub hostname_actual: pulumi_wasm_rust::Output<String>,
        /// The Oracle license model that applies to the Cloud VM Cluster, either `BringYourOwnLicense` or `LicenseIncluded`. Changing this forces a new Cloud VM Cluster to be created.
        pub license_model: pulumi_wasm_rust::Output<String>,
        /// If true, database backup on local Exadata storage is configured for the Cloud VM Cluster. If `false`, database backup on local Exadata storage is not available in the Cloud VM Cluster. Changing this forces a new Cloud VM Cluster to be created.
        pub local_backup_enabled: pulumi_wasm_rust::Output<bool>,
        /// The Azure Region where the Cloud VM Cluster should exist. Changing this forces a new Cloud VM Cluster to be created.
        pub location: pulumi_wasm_rust::Output<String>,
        /// The memory to be allocated in GBs. Changing this forces a new Cloud VM Cluster to be created.
        pub memory_size_in_gbs: pulumi_wasm_rust::Output<i32>,
        /// The name which should be used for this Cloud VM Cluster. Changing this forces a new Cloud VM Cluster to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The [OCID](https://docs.cloud.oracle.com/iaas/Content/General/Concepts/identifiers.htm) of the Cloud VM Cluster.
        pub ocid: pulumi_wasm_rust::Output<String>,
        /// The name of the Resource Group where the Cloud VM Cluster should exist. Changing this forces a new Cloud VM Cluster to be created.
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// The TCP Single Client Access Name (SCAN) port. The default port to 1521. Changing this forces a new Cloud VM Cluster to be created.
        pub scan_listener_port_tcp: pulumi_wasm_rust::Output<Option<i32>>,
        /// The TCPS Single Client Access Name (SCAN) port. The default port to 2484. Changing this forces a new Cloud VM Cluster to be created.
        pub scan_listener_port_tcp_ssl: pulumi_wasm_rust::Output<Option<i32>>,
        /// If true, the sparse disk group is configured for the Cloud VM Cluster. If `false`, the sparse disk group is not created. Changing this forces a new Cloud VM Cluster to be created.
        pub sparse_diskgroup_enabled: pulumi_wasm_rust::Output<bool>,
        /// The public key portion of one or more key pairs used for SSH access to the Cloud VM Cluster. Changing this forces a new Cloud VM Cluster to be created.
        pub ssh_public_keys: pulumi_wasm_rust::Output<Vec<String>>,
        /// The ID of the subnet associated with the Cloud VM Cluster. Changing this forces a new Cloud VM Cluster to be created.
        pub subnet_id: pulumi_wasm_rust::Output<String>,
        /// A mapping of tags which should be assigned to the Cloud VM Cluster.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The time zone of the Cloud VM Cluster. For details, see [Exadata Infrastructure Time Zones](https://docs.cloud.oracle.com/iaas/Content/Database/References/timezones.htm). Changing this forces a new Cloud VM Cluster to be created.
        pub time_zone: pulumi_wasm_rust::Output<String>,
        /// The ID of the Virtual Network associated with the Cloud VM Cluster. Changing this forces a new Cloud VM Cluster to be created.
        pub virtual_network_id: pulumi_wasm_rust::Output<String>,
        /// The OCID of the OCI Private DNS Zone to be associated with the Cloud VM Cluster. This is required for specifying your own private domain name. Changing this forces a new Cloud VM Cluster to be created.
        pub zone_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: CloudVmClusterArgs) -> CloudVmClusterResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let backup_subnet_cidr_binding = args.backup_subnet_cidr.get_inner();
        let cloud_exadata_infrastructure_id_binding = args
            .cloud_exadata_infrastructure_id
            .get_inner();
        let cluster_name_binding = args.cluster_name.get_inner();
        let cpu_core_count_binding = args.cpu_core_count.get_inner();
        let data_collection_options_binding = args.data_collection_options.get_inner();
        let data_storage_percentage_binding = args.data_storage_percentage.get_inner();
        let data_storage_size_in_tbs_binding = args.data_storage_size_in_tbs.get_inner();
        let db_node_storage_size_in_gbs_binding = args
            .db_node_storage_size_in_gbs
            .get_inner();
        let db_servers_binding = args.db_servers.get_inner();
        let display_name_binding = args.display_name.get_inner();
        let domain_binding = args.domain.get_inner();
        let gi_version_binding = args.gi_version.get_inner();
        let hostname_binding = args.hostname.get_inner();
        let license_model_binding = args.license_model.get_inner();
        let local_backup_enabled_binding = args.local_backup_enabled.get_inner();
        let location_binding = args.location.get_inner();
        let memory_size_in_gbs_binding = args.memory_size_in_gbs.get_inner();
        let name_binding = args.name.get_inner();
        let resource_group_name_binding = args.resource_group_name.get_inner();
        let scan_listener_port_tcp_binding = args.scan_listener_port_tcp.get_inner();
        let scan_listener_port_tcp_ssl_binding = args
            .scan_listener_port_tcp_ssl
            .get_inner();
        let sparse_diskgroup_enabled_binding = args.sparse_diskgroup_enabled.get_inner();
        let ssh_public_keys_binding = args.ssh_public_keys.get_inner();
        let subnet_id_binding = args.subnet_id.get_inner();
        let tags_binding = args.tags.get_inner();
        let time_zone_binding = args.time_zone.get_inner();
        let virtual_network_id_binding = args.virtual_network_id.get_inner();
        let zone_id_binding = args.zone_id.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:oracle/cloudVmCluster:CloudVmCluster".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "backupSubnetCidr".into(),
                    value: &backup_subnet_cidr_binding,
                },
                register_interface::ObjectField {
                    name: "cloudExadataInfrastructureId".into(),
                    value: &cloud_exadata_infrastructure_id_binding,
                },
                register_interface::ObjectField {
                    name: "clusterName".into(),
                    value: &cluster_name_binding,
                },
                register_interface::ObjectField {
                    name: "cpuCoreCount".into(),
                    value: &cpu_core_count_binding,
                },
                register_interface::ObjectField {
                    name: "dataCollectionOptions".into(),
                    value: &data_collection_options_binding,
                },
                register_interface::ObjectField {
                    name: "dataStoragePercentage".into(),
                    value: &data_storage_percentage_binding,
                },
                register_interface::ObjectField {
                    name: "dataStorageSizeInTbs".into(),
                    value: &data_storage_size_in_tbs_binding,
                },
                register_interface::ObjectField {
                    name: "dbNodeStorageSizeInGbs".into(),
                    value: &db_node_storage_size_in_gbs_binding,
                },
                register_interface::ObjectField {
                    name: "dbServers".into(),
                    value: &db_servers_binding,
                },
                register_interface::ObjectField {
                    name: "displayName".into(),
                    value: &display_name_binding,
                },
                register_interface::ObjectField {
                    name: "domain".into(),
                    value: &domain_binding,
                },
                register_interface::ObjectField {
                    name: "giVersion".into(),
                    value: &gi_version_binding,
                },
                register_interface::ObjectField {
                    name: "hostname".into(),
                    value: &hostname_binding,
                },
                register_interface::ObjectField {
                    name: "licenseModel".into(),
                    value: &license_model_binding,
                },
                register_interface::ObjectField {
                    name: "localBackupEnabled".into(),
                    value: &local_backup_enabled_binding,
                },
                register_interface::ObjectField {
                    name: "location".into(),
                    value: &location_binding,
                },
                register_interface::ObjectField {
                    name: "memorySizeInGbs".into(),
                    value: &memory_size_in_gbs_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding,
                },
                register_interface::ObjectField {
                    name: "scanListenerPortTcp".into(),
                    value: &scan_listener_port_tcp_binding,
                },
                register_interface::ObjectField {
                    name: "scanListenerPortTcpSsl".into(),
                    value: &scan_listener_port_tcp_ssl_binding,
                },
                register_interface::ObjectField {
                    name: "sparseDiskgroupEnabled".into(),
                    value: &sparse_diskgroup_enabled_binding,
                },
                register_interface::ObjectField {
                    name: "sshPublicKeys".into(),
                    value: &ssh_public_keys_binding,
                },
                register_interface::ObjectField {
                    name: "subnetId".into(),
                    value: &subnet_id_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "timeZone".into(),
                    value: &time_zone_binding,
                },
                register_interface::ObjectField {
                    name: "virtualNetworkId".into(),
                    value: &virtual_network_id_binding,
                },
                register_interface::ObjectField {
                    name: "zoneId".into(),
                    value: &zone_id_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "backupSubnetCidr".into(),
                },
                register_interface::ResultField {
                    name: "cloudExadataInfrastructureId".into(),
                },
                register_interface::ResultField {
                    name: "clusterName".into(),
                },
                register_interface::ResultField {
                    name: "cpuCoreCount".into(),
                },
                register_interface::ResultField {
                    name: "dataCollectionOptions".into(),
                },
                register_interface::ResultField {
                    name: "dataStoragePercentage".into(),
                },
                register_interface::ResultField {
                    name: "dataStorageSizeInTbs".into(),
                },
                register_interface::ResultField {
                    name: "dbNodeStorageSizeInGbs".into(),
                },
                register_interface::ResultField {
                    name: "dbServers".into(),
                },
                register_interface::ResultField {
                    name: "displayName".into(),
                },
                register_interface::ResultField {
                    name: "domain".into(),
                },
                register_interface::ResultField {
                    name: "giVersion".into(),
                },
                register_interface::ResultField {
                    name: "hostname".into(),
                },
                register_interface::ResultField {
                    name: "hostnameActual".into(),
                },
                register_interface::ResultField {
                    name: "licenseModel".into(),
                },
                register_interface::ResultField {
                    name: "localBackupEnabled".into(),
                },
                register_interface::ResultField {
                    name: "location".into(),
                },
                register_interface::ResultField {
                    name: "memorySizeInGbs".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "ocid".into(),
                },
                register_interface::ResultField {
                    name: "resourceGroupName".into(),
                },
                register_interface::ResultField {
                    name: "scanListenerPortTcp".into(),
                },
                register_interface::ResultField {
                    name: "scanListenerPortTcpSsl".into(),
                },
                register_interface::ResultField {
                    name: "sparseDiskgroupEnabled".into(),
                },
                register_interface::ResultField {
                    name: "sshPublicKeys".into(),
                },
                register_interface::ResultField {
                    name: "subnetId".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "timeZone".into(),
                },
                register_interface::ResultField {
                    name: "virtualNetworkId".into(),
                },
                register_interface::ResultField {
                    name: "zoneId".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        CloudVmClusterResult {
            backup_subnet_cidr: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("backupSubnetCidr").unwrap(),
            ),
            cloud_exadata_infrastructure_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("cloudExadataInfrastructureId").unwrap(),
            ),
            cluster_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("clusterName").unwrap(),
            ),
            cpu_core_count: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("cpuCoreCount").unwrap(),
            ),
            data_collection_options: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("dataCollectionOptions").unwrap(),
            ),
            data_storage_percentage: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("dataStoragePercentage").unwrap(),
            ),
            data_storage_size_in_tbs: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("dataStorageSizeInTbs").unwrap(),
            ),
            db_node_storage_size_in_gbs: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("dbNodeStorageSizeInGbs").unwrap(),
            ),
            db_servers: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("dbServers").unwrap(),
            ),
            display_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("displayName").unwrap(),
            ),
            domain: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("domain").unwrap(),
            ),
            gi_version: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("giVersion").unwrap(),
            ),
            hostname: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("hostname").unwrap(),
            ),
            hostname_actual: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("hostnameActual").unwrap(),
            ),
            license_model: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("licenseModel").unwrap(),
            ),
            local_backup_enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("localBackupEnabled").unwrap(),
            ),
            location: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("location").unwrap(),
            ),
            memory_size_in_gbs: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("memorySizeInGbs").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            ocid: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("ocid").unwrap(),
            ),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceGroupName").unwrap(),
            ),
            scan_listener_port_tcp: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("scanListenerPortTcp").unwrap(),
            ),
            scan_listener_port_tcp_ssl: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("scanListenerPortTcpSsl").unwrap(),
            ),
            sparse_diskgroup_enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("sparseDiskgroupEnabled").unwrap(),
            ),
            ssh_public_keys: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("sshPublicKeys").unwrap(),
            ),
            subnet_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("subnetId").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            time_zone: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("timeZone").unwrap(),
            ),
            virtual_network_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("virtualNetworkId").unwrap(),
            ),
            zone_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("zoneId").unwrap(),
            ),
        }
    }
}