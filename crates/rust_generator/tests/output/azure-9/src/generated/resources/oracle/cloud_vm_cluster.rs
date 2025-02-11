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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod cloud_vm_cluster {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct CloudVmClusterArgs {
        /// The backup subnet CIDR of the Virtual Network associated with the Cloud VM Cluster. Changing this forces a new Cloud VM Cluster to be created.
        #[builder(into, default)]
        pub backup_subnet_cidr: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The [OCID](https://docs.cloud.oracle.com/iaas/Content/General/Concepts/identifiers.htm) of the Cloud Exadata infrastructure. Changing this forces a new Cloud VM Cluster to be created.
        #[builder(into)]
        pub cloud_exadata_infrastructure_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The cluster name for Cloud VM Cluster. Changing this forces a new Cloud VM Cluster to be created.
        #[builder(into, default)]
        pub cluster_name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The number of CPU cores enabled on the Cloud VM Cluster. Changing this forces a new Cloud VM Cluster to be created.
        #[builder(into)]
        pub cpu_core_count: pulumi_gestalt_rust::InputOrOutput<i32>,
        /// A `data_collection_options` block as defined below. Changing this forces a new Cloud VM Cluster to be created.
        #[builder(into, default)]
        pub data_collection_options: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::oracle::CloudVmClusterDataCollectionOptions>,
        >,
        /// The percentage assigned to DATA storage (user data and database files). Changing this forces a new Cloud VM Cluster to be created. The remaining percentage is assigned to RECO storage (database redo logs, archive logs, and recovery manager backups). Accepted values are `35`, `40`, `60` and `80`.
        #[builder(into, default)]
        pub data_storage_percentage: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// The data disk group size to be allocated in TBs. Changing this forces a new Cloud VM Cluster to be created.
        #[builder(into, default)]
        pub data_storage_size_in_tbs: pulumi_gestalt_rust::InputOrOutput<Option<f64>>,
        /// The local node storage to be allocated in GBs. Changing this forces a new Cloud VM Cluster to be created.
        #[builder(into, default)]
        pub db_node_storage_size_in_gbs: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// The list of DB servers. Changing this forces a new Cloud VM Cluster to be created.
        #[builder(into)]
        pub db_servers: pulumi_gestalt_rust::InputOrOutput<Vec<String>>,
        /// The user-friendly name for the Cloud VM Cluster. Changing this forces a new Cloud VM Cluster to be created. The name does not need to be unique.
        #[builder(into)]
        pub display_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the OCI Private DNS Zone to be associated with the Cloud VM Cluster. This is required for specifying your own private domain name. Changing this forces a new Cloud VM Cluster to be created.
        #[builder(into, default)]
        pub domain: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A valid Oracle Grid Infrastructure (GI) software version. Changing this forces a new Cloud VM Cluster to be created.
        #[builder(into)]
        pub gi_version: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The hostname for the Cloud VM Cluster without suffix. Changing this forces a new Cloud VM Cluster to be created.
        #[builder(into)]
        pub hostname: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The Oracle license model that applies to the Cloud VM Cluster, either `BringYourOwnLicense` or `LicenseIncluded`. Changing this forces a new Cloud VM Cluster to be created.
        #[builder(into)]
        pub license_model: pulumi_gestalt_rust::InputOrOutput<String>,
        /// If true, database backup on local Exadata storage is configured for the Cloud VM Cluster. If `false`, database backup on local Exadata storage is not available in the Cloud VM Cluster. Changing this forces a new Cloud VM Cluster to be created.
        #[builder(into, default)]
        pub local_backup_enabled: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// The Azure Region where the Cloud VM Cluster should exist. Changing this forces a new Cloud VM Cluster to be created.
        #[builder(into, default)]
        pub location: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The memory to be allocated in GBs. Changing this forces a new Cloud VM Cluster to be created.
        #[builder(into, default)]
        pub memory_size_in_gbs: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// The name which should be used for this Cloud VM Cluster. Changing this forces a new Cloud VM Cluster to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of the Resource Group where the Cloud VM Cluster should exist. Changing this forces a new Cloud VM Cluster to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The TCP Single Client Access Name (SCAN) port. The default port to 1521. Changing this forces a new Cloud VM Cluster to be created.
        #[builder(into, default)]
        pub scan_listener_port_tcp: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// The TCPS Single Client Access Name (SCAN) port. The default port to 2484. Changing this forces a new Cloud VM Cluster to be created.
        #[builder(into, default)]
        pub scan_listener_port_tcp_ssl: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// If true, the sparse disk group is configured for the Cloud VM Cluster. If `false`, the sparse disk group is not created. Changing this forces a new Cloud VM Cluster to be created.
        #[builder(into, default)]
        pub sparse_diskgroup_enabled: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// The public key portion of one or more key pairs used for SSH access to the Cloud VM Cluster. Changing this forces a new Cloud VM Cluster to be created.
        #[builder(into)]
        pub ssh_public_keys: pulumi_gestalt_rust::InputOrOutput<Vec<String>>,
        /// The ID of the subnet associated with the Cloud VM Cluster. Changing this forces a new Cloud VM Cluster to be created.
        #[builder(into)]
        pub subnet_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A mapping of tags which should be assigned to the Cloud VM Cluster.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The time zone of the Cloud VM Cluster. For details, see [Exadata Infrastructure Time Zones](https://docs.cloud.oracle.com/iaas/Content/Database/References/timezones.htm). Changing this forces a new Cloud VM Cluster to be created.
        #[builder(into, default)]
        pub time_zone: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ID of the Virtual Network associated with the Cloud VM Cluster. Changing this forces a new Cloud VM Cluster to be created.
        #[builder(into)]
        pub virtual_network_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The OCID of the OCI Private DNS Zone to be associated with the Cloud VM Cluster. This is required for specifying your own private domain name. Changing this forces a new Cloud VM Cluster to be created.
        #[builder(into, default)]
        pub zone_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct CloudVmClusterResult {
        /// The backup subnet CIDR of the Virtual Network associated with the Cloud VM Cluster. Changing this forces a new Cloud VM Cluster to be created.
        pub backup_subnet_cidr: pulumi_gestalt_rust::Output<Option<String>>,
        /// The [OCID](https://docs.cloud.oracle.com/iaas/Content/General/Concepts/identifiers.htm) of the Cloud Exadata infrastructure. Changing this forces a new Cloud VM Cluster to be created.
        pub cloud_exadata_infrastructure_id: pulumi_gestalt_rust::Output<String>,
        /// The cluster name for Cloud VM Cluster. Changing this forces a new Cloud VM Cluster to be created.
        pub cluster_name: pulumi_gestalt_rust::Output<String>,
        /// The number of CPU cores enabled on the Cloud VM Cluster. Changing this forces a new Cloud VM Cluster to be created.
        pub cpu_core_count: pulumi_gestalt_rust::Output<i32>,
        /// A `data_collection_options` block as defined below. Changing this forces a new Cloud VM Cluster to be created.
        pub data_collection_options: pulumi_gestalt_rust::Output<
            super::super::types::oracle::CloudVmClusterDataCollectionOptions,
        >,
        /// The percentage assigned to DATA storage (user data and database files). Changing this forces a new Cloud VM Cluster to be created. The remaining percentage is assigned to RECO storage (database redo logs, archive logs, and recovery manager backups). Accepted values are `35`, `40`, `60` and `80`.
        pub data_storage_percentage: pulumi_gestalt_rust::Output<i32>,
        /// The data disk group size to be allocated in TBs. Changing this forces a new Cloud VM Cluster to be created.
        pub data_storage_size_in_tbs: pulumi_gestalt_rust::Output<f64>,
        /// The local node storage to be allocated in GBs. Changing this forces a new Cloud VM Cluster to be created.
        pub db_node_storage_size_in_gbs: pulumi_gestalt_rust::Output<i32>,
        /// The list of DB servers. Changing this forces a new Cloud VM Cluster to be created.
        pub db_servers: pulumi_gestalt_rust::Output<Vec<String>>,
        /// The user-friendly name for the Cloud VM Cluster. Changing this forces a new Cloud VM Cluster to be created. The name does not need to be unique.
        pub display_name: pulumi_gestalt_rust::Output<String>,
        /// The name of the OCI Private DNS Zone to be associated with the Cloud VM Cluster. This is required for specifying your own private domain name. Changing this forces a new Cloud VM Cluster to be created.
        pub domain: pulumi_gestalt_rust::Output<String>,
        /// A valid Oracle Grid Infrastructure (GI) software version. Changing this forces a new Cloud VM Cluster to be created.
        pub gi_version: pulumi_gestalt_rust::Output<String>,
        /// The hostname for the Cloud VM Cluster without suffix. Changing this forces a new Cloud VM Cluster to be created.
        pub hostname: pulumi_gestalt_rust::Output<String>,
        /// The hostname for the Cloud VM Cluster with suffix.
        pub hostname_actual: pulumi_gestalt_rust::Output<String>,
        /// The Oracle license model that applies to the Cloud VM Cluster, either `BringYourOwnLicense` or `LicenseIncluded`. Changing this forces a new Cloud VM Cluster to be created.
        pub license_model: pulumi_gestalt_rust::Output<String>,
        /// If true, database backup on local Exadata storage is configured for the Cloud VM Cluster. If `false`, database backup on local Exadata storage is not available in the Cloud VM Cluster. Changing this forces a new Cloud VM Cluster to be created.
        pub local_backup_enabled: pulumi_gestalt_rust::Output<bool>,
        /// The Azure Region where the Cloud VM Cluster should exist. Changing this forces a new Cloud VM Cluster to be created.
        pub location: pulumi_gestalt_rust::Output<String>,
        /// The memory to be allocated in GBs. Changing this forces a new Cloud VM Cluster to be created.
        pub memory_size_in_gbs: pulumi_gestalt_rust::Output<i32>,
        /// The name which should be used for this Cloud VM Cluster. Changing this forces a new Cloud VM Cluster to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The [OCID](https://docs.cloud.oracle.com/iaas/Content/General/Concepts/identifiers.htm) of the Cloud VM Cluster.
        pub ocid: pulumi_gestalt_rust::Output<String>,
        /// The name of the Resource Group where the Cloud VM Cluster should exist. Changing this forces a new Cloud VM Cluster to be created.
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// The TCP Single Client Access Name (SCAN) port. The default port to 1521. Changing this forces a new Cloud VM Cluster to be created.
        pub scan_listener_port_tcp: pulumi_gestalt_rust::Output<Option<i32>>,
        /// The TCPS Single Client Access Name (SCAN) port. The default port to 2484. Changing this forces a new Cloud VM Cluster to be created.
        pub scan_listener_port_tcp_ssl: pulumi_gestalt_rust::Output<Option<i32>>,
        /// If true, the sparse disk group is configured for the Cloud VM Cluster. If `false`, the sparse disk group is not created. Changing this forces a new Cloud VM Cluster to be created.
        pub sparse_diskgroup_enabled: pulumi_gestalt_rust::Output<bool>,
        /// The public key portion of one or more key pairs used for SSH access to the Cloud VM Cluster. Changing this forces a new Cloud VM Cluster to be created.
        pub ssh_public_keys: pulumi_gestalt_rust::Output<Vec<String>>,
        /// The ID of the subnet associated with the Cloud VM Cluster. Changing this forces a new Cloud VM Cluster to be created.
        pub subnet_id: pulumi_gestalt_rust::Output<String>,
        /// A mapping of tags which should be assigned to the Cloud VM Cluster.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The time zone of the Cloud VM Cluster. For details, see [Exadata Infrastructure Time Zones](https://docs.cloud.oracle.com/iaas/Content/Database/References/timezones.htm). Changing this forces a new Cloud VM Cluster to be created.
        pub time_zone: pulumi_gestalt_rust::Output<String>,
        /// The ID of the Virtual Network associated with the Cloud VM Cluster. Changing this forces a new Cloud VM Cluster to be created.
        pub virtual_network_id: pulumi_gestalt_rust::Output<String>,
        /// The OCID of the OCI Private DNS Zone to be associated with the Cloud VM Cluster. This is required for specifying your own private domain name. Changing this forces a new Cloud VM Cluster to be created.
        pub zone_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: CloudVmClusterArgs,
    ) -> CloudVmClusterResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let backup_subnet_cidr_binding = args.backup_subnet_cidr.get_output(context);
        let cloud_exadata_infrastructure_id_binding = args
            .cloud_exadata_infrastructure_id
            .get_output(context);
        let cluster_name_binding = args.cluster_name.get_output(context);
        let cpu_core_count_binding = args.cpu_core_count.get_output(context);
        let data_collection_options_binding = args
            .data_collection_options
            .get_output(context);
        let data_storage_percentage_binding = args
            .data_storage_percentage
            .get_output(context);
        let data_storage_size_in_tbs_binding = args
            .data_storage_size_in_tbs
            .get_output(context);
        let db_node_storage_size_in_gbs_binding = args
            .db_node_storage_size_in_gbs
            .get_output(context);
        let db_servers_binding = args.db_servers.get_output(context);
        let display_name_binding = args.display_name.get_output(context);
        let domain_binding = args.domain.get_output(context);
        let gi_version_binding = args.gi_version.get_output(context);
        let hostname_binding = args.hostname.get_output(context);
        let license_model_binding = args.license_model.get_output(context);
        let local_backup_enabled_binding = args.local_backup_enabled.get_output(context);
        let location_binding = args.location.get_output(context);
        let memory_size_in_gbs_binding = args.memory_size_in_gbs.get_output(context);
        let name_binding = args.name.get_output(context);
        let resource_group_name_binding = args.resource_group_name.get_output(context);
        let scan_listener_port_tcp_binding = args
            .scan_listener_port_tcp
            .get_output(context);
        let scan_listener_port_tcp_ssl_binding = args
            .scan_listener_port_tcp_ssl
            .get_output(context);
        let sparse_diskgroup_enabled_binding = args
            .sparse_diskgroup_enabled
            .get_output(context);
        let ssh_public_keys_binding = args.ssh_public_keys.get_output(context);
        let subnet_id_binding = args.subnet_id.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let time_zone_binding = args.time_zone.get_output(context);
        let virtual_network_id_binding = args.virtual_network_id.get_output(context);
        let zone_id_binding = args.zone_id.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:oracle/cloudVmCluster:CloudVmCluster".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "backupSubnetCidr".into(),
                    value: &backup_subnet_cidr_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "cloudExadataInfrastructureId".into(),
                    value: &cloud_exadata_infrastructure_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "clusterName".into(),
                    value: &cluster_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "cpuCoreCount".into(),
                    value: &cpu_core_count_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "dataCollectionOptions".into(),
                    value: &data_collection_options_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "dataStoragePercentage".into(),
                    value: &data_storage_percentage_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "dataStorageSizeInTbs".into(),
                    value: &data_storage_size_in_tbs_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "dbNodeStorageSizeInGbs".into(),
                    value: &db_node_storage_size_in_gbs_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "dbServers".into(),
                    value: &db_servers_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "displayName".into(),
                    value: &display_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "domain".into(),
                    value: &domain_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "giVersion".into(),
                    value: &gi_version_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "hostname".into(),
                    value: &hostname_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "licenseModel".into(),
                    value: &license_model_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "localBackupEnabled".into(),
                    value: &local_backup_enabled_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "location".into(),
                    value: &location_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "memorySizeInGbs".into(),
                    value: &memory_size_in_gbs_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "scanListenerPortTcp".into(),
                    value: &scan_listener_port_tcp_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "scanListenerPortTcpSsl".into(),
                    value: &scan_listener_port_tcp_ssl_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "sparseDiskgroupEnabled".into(),
                    value: &sparse_diskgroup_enabled_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "sshPublicKeys".into(),
                    value: &ssh_public_keys_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "subnetId".into(),
                    value: &subnet_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: &tags_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "timeZone".into(),
                    value: &time_zone_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "virtualNetworkId".into(),
                    value: &virtual_network_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "zoneId".into(),
                    value: &zone_id_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        CloudVmClusterResult {
            backup_subnet_cidr: o.get_field("backupSubnetCidr"),
            cloud_exadata_infrastructure_id: o.get_field("cloudExadataInfrastructureId"),
            cluster_name: o.get_field("clusterName"),
            cpu_core_count: o.get_field("cpuCoreCount"),
            data_collection_options: o.get_field("dataCollectionOptions"),
            data_storage_percentage: o.get_field("dataStoragePercentage"),
            data_storage_size_in_tbs: o.get_field("dataStorageSizeInTbs"),
            db_node_storage_size_in_gbs: o.get_field("dbNodeStorageSizeInGbs"),
            db_servers: o.get_field("dbServers"),
            display_name: o.get_field("displayName"),
            domain: o.get_field("domain"),
            gi_version: o.get_field("giVersion"),
            hostname: o.get_field("hostname"),
            hostname_actual: o.get_field("hostnameActual"),
            license_model: o.get_field("licenseModel"),
            local_backup_enabled: o.get_field("localBackupEnabled"),
            location: o.get_field("location"),
            memory_size_in_gbs: o.get_field("memorySizeInGbs"),
            name: o.get_field("name"),
            ocid: o.get_field("ocid"),
            resource_group_name: o.get_field("resourceGroupName"),
            scan_listener_port_tcp: o.get_field("scanListenerPortTcp"),
            scan_listener_port_tcp_ssl: o.get_field("scanListenerPortTcpSsl"),
            sparse_diskgroup_enabled: o.get_field("sparseDiskgroupEnabled"),
            ssh_public_keys: o.get_field("sshPublicKeys"),
            subnet_id: o.get_field("subnetId"),
            tags: o.get_field("tags"),
            time_zone: o.get_field("timeZone"),
            virtual_network_id: o.get_field("virtualNetworkId"),
            zone_id: o.get_field("zoneId"),
        }
    }
}
