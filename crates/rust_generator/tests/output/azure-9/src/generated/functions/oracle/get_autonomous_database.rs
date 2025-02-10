#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_autonomous_database {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetAutonomousDatabaseArgs {
        /// The name of this Autonomous Database.
        #[builder(into)]
        pub name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the Resource Group where the Autonomous Database exists.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetAutonomousDatabaseResult {
        /// The current amount of storage in use for user and system data, in terabytes (TB).
        pub actual_used_data_storage_size_in_tbs: pulumi_gestalt_rust::Output<f64>,
        /// The amount of storage currently allocated for the database tables and billed for, rounded up. When auto-scaling is not enabled, this value is equal to the `dataStorageSizeInTBs` value. You can compare this value to the `actualUsedDataStorageSizeInTBs` value to determine if a manual shrink operation is appropriate for your allocated storage.
        pub allocated_storage_size_in_tbs: pulumi_gestalt_rust::Output<f64>,
        /// The client IP access control list (ACL). This feature is available for [Autonomous Database Serverless] (https://docs.oracle.com/en/cloud/paas/autonomous-database/index.html) and on Exadata Cloud@Customer. Only clients connecting from an IP address included in the ACL may access the Autonomous Database instance. If `arePrimaryWhitelistedIpsUsed` is 'TRUE' then Autonomous Database uses this primary's IP access control list (ACL) for the disaster recovery peer called `standbywhitelistedips`.
        pub allowed_ips: pulumi_gestalt_rust::Output<Vec<i32>>,
        /// Indicates if auto scaling is enabled for the Autonomous Database CPU core count.
        pub auto_scaling_enabled: pulumi_gestalt_rust::Output<bool>,
        /// Indicates if auto scaling is enabled for the Autonomous Database storage.
        pub auto_scaling_for_storage_enabled: pulumi_gestalt_rust::Output<bool>,
        /// The database [OCID](https://docs.cloud.oracle.com/iaas/Content/General/Concepts/identifiers.htm).
        pub autonomous_database_id: pulumi_gestalt_rust::Output<String>,
        /// List of Oracle Database versions available for a database upgrade. If there are no version upgrades available, this list is empty.
        pub available_upgrade_versions: pulumi_gestalt_rust::Output<Vec<String>>,
        /// Retention period, in days, for backups.
        pub backup_retention_period_in_days: pulumi_gestalt_rust::Output<i32>,
        /// The character set for the autonomous database.
        pub character_set: pulumi_gestalt_rust::Output<String>,
        /// The compute amount (CPUs) available to the database.
        pub compute_count: pulumi_gestalt_rust::Output<f64>,
        /// The number of CPU cores to be made available to the database. When the ECPU is selected, the value for cpuCoreCount is 0. For Autonomous Database on Dedicated Exadata infrastructure, the maximum number of cores is determined by the infrastructure shape. See [Characteristics of Infrastructure Shapes](https://www.oracle.com/pls/topic/lookup?ctx=en/cloud/paas/autonomous-database&id=ATPFG-GUID-B0F033C1-CC5A-42F0-B2E7-3CECFEDA1FD1) for shape details.
        pub cpu_core_count: pulumi_gestalt_rust::Output<i32>,
        /// The quantity of data in the database, in gigabytes.
        pub data_storage_size_in_gbs: pulumi_gestalt_rust::Output<i32>,
        /// The maximum storage that can be allocated for the database, in terabytes.
        pub data_storage_size_in_tbs: pulumi_gestalt_rust::Output<i32>,
        /// The DB node storage size in, in gigabytes.
        pub db_node_storage_size_in_gbs: pulumi_gestalt_rust::Output<i32>,
        /// A valid Oracle Database version for Autonomous Database.
        pub db_version: pulumi_gestalt_rust::Output<String>,
        /// The user-friendly name for the Autonomous Database. The name does not have to be unique.
        pub display_name: pulumi_gestalt_rust::Output<String>,
        /// Indicates the number of seconds of data loss for a Data Guard failover.
        pub failed_data_recovery_in_seconds: pulumi_gestalt_rust::Output<i32>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// The area assigned to In-Memory tables in Autonomous Database.
        pub in_memory_area_in_gbs: pulumi_gestalt_rust::Output<i32>,
        /// Information about the current lifecycle state.
        pub lifecycle_details: pulumi_gestalt_rust::Output<String>,
        /// Parameter that allows users to select an acceptable maximum data loss limit in seconds, up to which Automatic Failover will be triggered when necessary for a Local Autonomous Data Guard
        pub local_adg_auto_failover_max_data_loss_limit: pulumi_gestalt_rust::Output<
            i32,
        >,
        /// Indicates whether the Autonomous Database has local (in-region) Data Guard enabled. Not applicable to cross-region Autonomous Data Guard associations, or to Autonomous Databases using dedicated Exadata infrastructure or Exadata Cloud@Customer infrastructure.
        pub local_data_guard_enabled: pulumi_gestalt_rust::Output<bool>,
        /// The Azure Region where the Autonomous Database exists.
        pub location: pulumi_gestalt_rust::Output<String>,
        /// The amount of memory (in GBs) enabled per ECPU or OCPU.
        pub memory_per_oracle_compute_unit_in_gbs: pulumi_gestalt_rust::Output<i32>,
        /// Specifies if the Autonomous Database requires mTLS connections.
        pub mtls_connection_required: pulumi_gestalt_rust::Output<bool>,
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The national character set for the autonomous database.  The default is AL16UTF16. Allowed values are: AL16UTF16 or UTF8.
        pub national_character_set: pulumi_gestalt_rust::Output<String>,
        /// The date and time when the next long-term backup would be created.
        pub next_long_term_backup_time_stamp: pulumi_gestalt_rust::Output<String>,
        /// The URL of the resource in the OCI console.
        pub oci_url: pulumi_gestalt_rust::Output<String>,
        /// The [OCID](https://docs.oracle.com/en-us/iaas/Content/General/Concepts/identifiers.htm) of the autonomous database.
        pub ocid: pulumi_gestalt_rust::Output<String>,
        pub peer_db_id: pulumi_gestalt_rust::Output<String>,
        /// The list of [OCIDs](https://docs.oracle.com/iaas/Content/General/Concepts/identifiers.htm) of standby databases located in Autonomous Data Guard remote regions that are associated with the source database. Note that for Autonomous Database Serverless instances, standby databases located in the same region as the source primary database do not have OCIDs.
        pub peer_db_ids: pulumi_gestalt_rust::Output<Vec<String>>,
        /// Indicates if the Autonomous Database version is a preview version.
        pub preview: pulumi_gestalt_rust::Output<bool>,
        /// Indicates if the Autonomous Database version is a preview version with service terms accepted.
        pub preview_version_with_service_terms_accepted: pulumi_gestalt_rust::Output<
            bool,
        >,
        /// The private endpoint for the resource.
        pub private_endpoint: pulumi_gestalt_rust::Output<String>,
        /// The private endpoint Ip address for the resource.
        pub private_endpoint_ip: pulumi_gestalt_rust::Output<String>,
        /// The private endpoint label for the resource.
        pub private_endpoint_label: pulumi_gestalt_rust::Output<String>,
        /// An array of CPU values that an Autonomous Database can be scaled to.
        pub provisionable_cpuses: pulumi_gestalt_rust::Output<Vec<i32>>,
        /// Indicates whether the Autonomous Database has Cross Region Data Guard enabled. Not applicable to Autonomous Databases using dedicated Exadata infrastructure or Exadata Cloud@Customer infrastructure.
        pub remote_data_guard_enabled: pulumi_gestalt_rust::Output<bool>,
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// The URL of the Service Console for the Autonomous Database.
        pub service_console_url: pulumi_gestalt_rust::Output<String>,
        /// The URL of the SQL web developer.
        pub sql_web_developer_url: pulumi_gestalt_rust::Output<String>,
        /// The [OCID](https://docs.cloud.oracle.com/iaas/Content/General/Concepts/identifiers.htm) of the subnet the resource is associated with.
        pub subnet_id: pulumi_gestalt_rust::Output<String>,
        /// The list of regions that support the creation of an Autonomous Database clone or an Autonomous Data Guard standby database.
        pub supported_regions_to_clone_tos: pulumi_gestalt_rust::Output<Vec<i32>>,
        /// A mapping of tags assigned to the Autonomous Database.
        pub tags: pulumi_gestalt_rust::Output<std::collections::HashMap<String, String>>,
        /// The date and time the Autonomous Database was created.
        pub time_created: pulumi_gestalt_rust::Output<String>,
        /// The date and time the Autonomous Data Guard role was switched for the Autonomous Database. For databases that have standbys in both the primary Data Guard region and a remote Data Guard standby region, this is the latest timestamp of either the database using the "primary" role in the primary Data Guard region, or database located in the remote Data Guard standby region.
        pub time_data_guard_role_changed: pulumi_gestalt_rust::Output<String>,
        /// The date and time the Always Free database will be automatically deleted because of inactivity. If the database is in the STOPPED state and without activity until this time, it will be deleted.
        pub time_deletion_of_free_autonomous_database: pulumi_gestalt_rust::Output<
            String,
        >,
        /// The date and time that Autonomous Data Guard was enabled for an Autonomous Database where the standby was provisioned in the same region as the primary database.
        pub time_local_data_guard_enabled_on: pulumi_gestalt_rust::Output<String>,
        /// The date and time when maintenance will begin.
        pub time_maintenance_begin: pulumi_gestalt_rust::Output<String>,
        /// The date and time when maintenance will end.
        pub time_maintenance_end: pulumi_gestalt_rust::Output<String>,
        /// The timestamp of the last failover operation.
        pub time_of_last_failover: pulumi_gestalt_rust::Output<String>,
        /// The date and time when last refresh happened.
        pub time_of_last_refresh: pulumi_gestalt_rust::Output<String>,
        /// The refresh point timestamp (UTC). The refresh point is the time to which the database was most recently refreshed. Data created after the refresh point is not included in the refresh.
        pub time_of_last_refresh_point: pulumi_gestalt_rust::Output<String>,
        /// The timestamp of the last switchover operation for the Autonomous Database.
        pub time_of_last_switchover: pulumi_gestalt_rust::Output<String>,
        /// The date and time the Always Free database will be stopped because of inactivity. If this time is reached without any database activity, the database will automatically be put into the STOPPED state.
        pub time_reclamation_of_free_autonomous_database: pulumi_gestalt_rust::Output<
            String,
        >,
        /// The storage space consumed by Autonomous Database in GBs.
        pub used_data_storage_size_in_gbs: pulumi_gestalt_rust::Output<i32>,
        /// The amount of storage that has been used, in terabytes.
        pub used_data_storage_size_in_tbs: pulumi_gestalt_rust::Output<i32>,
        /// The ID to an Azure Resource Manager vnet resource.
        pub virtual_network_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetAutonomousDatabaseArgs,
    ) -> GetAutonomousDatabaseResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let name_binding = args.name.get_output(context);
        let resource_group_name_binding = args.resource_group_name.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "azure:oracle/getAutonomousDatabase:getAutonomousDatabase".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceGroupName".into(),
                    value: resource_group_name_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetAutonomousDatabaseResult {
            actual_used_data_storage_size_in_tbs: o
                .get_field("actualUsedDataStorageSizeInTbs"),
            allocated_storage_size_in_tbs: o.get_field("allocatedStorageSizeInTbs"),
            allowed_ips: o.get_field("allowedIps"),
            auto_scaling_enabled: o.get_field("autoScalingEnabled"),
            auto_scaling_for_storage_enabled: o
                .get_field("autoScalingForStorageEnabled"),
            autonomous_database_id: o.get_field("autonomousDatabaseId"),
            available_upgrade_versions: o.get_field("availableUpgradeVersions"),
            backup_retention_period_in_days: o.get_field("backupRetentionPeriodInDays"),
            character_set: o.get_field("characterSet"),
            compute_count: o.get_field("computeCount"),
            cpu_core_count: o.get_field("cpuCoreCount"),
            data_storage_size_in_gbs: o.get_field("dataStorageSizeInGbs"),
            data_storage_size_in_tbs: o.get_field("dataStorageSizeInTbs"),
            db_node_storage_size_in_gbs: o.get_field("dbNodeStorageSizeInGbs"),
            db_version: o.get_field("dbVersion"),
            display_name: o.get_field("displayName"),
            failed_data_recovery_in_seconds: o.get_field("failedDataRecoveryInSeconds"),
            id: o.get_field("id"),
            in_memory_area_in_gbs: o.get_field("inMemoryAreaInGbs"),
            lifecycle_details: o.get_field("lifecycleDetails"),
            local_adg_auto_failover_max_data_loss_limit: o
                .get_field("localAdgAutoFailoverMaxDataLossLimit"),
            local_data_guard_enabled: o.get_field("localDataGuardEnabled"),
            location: o.get_field("location"),
            memory_per_oracle_compute_unit_in_gbs: o
                .get_field("memoryPerOracleComputeUnitInGbs"),
            mtls_connection_required: o.get_field("mtlsConnectionRequired"),
            name: o.get_field("name"),
            national_character_set: o.get_field("nationalCharacterSet"),
            next_long_term_backup_time_stamp: o.get_field("nextLongTermBackupTimeStamp"),
            oci_url: o.get_field("ociUrl"),
            ocid: o.get_field("ocid"),
            peer_db_id: o.get_field("peerDbId"),
            peer_db_ids: o.get_field("peerDbIds"),
            preview: o.get_field("preview"),
            preview_version_with_service_terms_accepted: o
                .get_field("previewVersionWithServiceTermsAccepted"),
            private_endpoint: o.get_field("privateEndpoint"),
            private_endpoint_ip: o.get_field("privateEndpointIp"),
            private_endpoint_label: o.get_field("privateEndpointLabel"),
            provisionable_cpuses: o.get_field("provisionableCpuses"),
            remote_data_guard_enabled: o.get_field("remoteDataGuardEnabled"),
            resource_group_name: o.get_field("resourceGroupName"),
            service_console_url: o.get_field("serviceConsoleUrl"),
            sql_web_developer_url: o.get_field("sqlWebDeveloperUrl"),
            subnet_id: o.get_field("subnetId"),
            supported_regions_to_clone_tos: o.get_field("supportedRegionsToCloneTos"),
            tags: o.get_field("tags"),
            time_created: o.get_field("timeCreated"),
            time_data_guard_role_changed: o.get_field("timeDataGuardRoleChanged"),
            time_deletion_of_free_autonomous_database: o
                .get_field("timeDeletionOfFreeAutonomousDatabase"),
            time_local_data_guard_enabled_on: o.get_field("timeLocalDataGuardEnabledOn"),
            time_maintenance_begin: o.get_field("timeMaintenanceBegin"),
            time_maintenance_end: o.get_field("timeMaintenanceEnd"),
            time_of_last_failover: o.get_field("timeOfLastFailover"),
            time_of_last_refresh: o.get_field("timeOfLastRefresh"),
            time_of_last_refresh_point: o.get_field("timeOfLastRefreshPoint"),
            time_of_last_switchover: o.get_field("timeOfLastSwitchover"),
            time_reclamation_of_free_autonomous_database: o
                .get_field("timeReclamationOfFreeAutonomousDatabase"),
            used_data_storage_size_in_gbs: o.get_field("usedDataStorageSizeInGbs"),
            used_data_storage_size_in_tbs: o.get_field("usedDataStorageSizeInTbs"),
            virtual_network_id: o.get_field("virtualNetworkId"),
        }
    }
}
