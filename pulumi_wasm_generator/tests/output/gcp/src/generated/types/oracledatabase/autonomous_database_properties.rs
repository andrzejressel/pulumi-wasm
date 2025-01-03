#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct AutonomousDatabaseProperties {
    /// (Output)
    /// The amount of storage currently being used for user and system data, in
    /// terabytes.
    #[builder(into, default)]
    #[serde(rename = "actualUsedDataStorageSizeTb")]
    pub r#actual_used_data_storage_size_tb: Box<Option<f64>>,
    /// (Output)
    /// The amount of storage currently allocated for the database tables and
    /// billed for, rounded up in terabytes.
    #[builder(into, default)]
    #[serde(rename = "allocatedStorageSizeTb")]
    pub r#allocated_storage_size_tb: Box<Option<f64>>,
    /// (Output)
    /// Oracle APEX Application Development.
    /// https://docs.oracle.com/en-us/iaas/api/#/en/database/20160918/datatypes/AutonomousDatabaseApex
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "apexDetails")]
    pub r#apex_details: Box<Option<Vec<super::super::types::oracledatabase::AutonomousDatabasePropertiesApexDetail>>>,
    /// (Output)
    /// This field indicates the status of Data Guard and Access control for the
    /// Autonomous Database. The field's value is null if Data Guard is disabled
    /// or Access Control is disabled. The field's value is TRUE if both Data Guard
    /// and Access Control are enabled, and the Autonomous Database is using
    /// primary IP access control list (ACL) for standby. The field's value is
    /// FALSE if both Data Guard and Access Control are enabled, and the Autonomous
    /// Database is using a different IP access control list (ACL) for standby
    /// compared to primary.
    #[builder(into, default)]
    #[serde(rename = "arePrimaryAllowlistedIpsUsed")]
    pub r#are_primary_allowlisted_ips_used: Box<Option<bool>>,
    /// (Output)
    /// The Autonomous Container Database OCID.
    #[builder(into, default)]
    #[serde(rename = "autonomousContainerDatabaseId")]
    pub r#autonomous_container_database_id: Box<Option<String>>,
    /// (Output)
    /// The list of available Oracle Database upgrade versions for an Autonomous
    /// Database.
    #[builder(into, default)]
    #[serde(rename = "availableUpgradeVersions")]
    pub r#available_upgrade_versions: Box<Option<Vec<String>>>,
    /// The retention period for the Autonomous Database. This field is specified
    /// in days, can range from 1 day to 60 days, and has a default value of
    /// 60 days.
    #[builder(into, default)]
    #[serde(rename = "backupRetentionPeriodDays")]
    pub r#backup_retention_period_days: Box<Option<i32>>,
    /// The character set for the Autonomous Database. The default is AL32UTF8.
    #[builder(into, default)]
    #[serde(rename = "characterSet")]
    pub r#character_set: Box<Option<String>>,
    /// The number of compute servers for the Autonomous Database.
    #[builder(into, default)]
    #[serde(rename = "computeCount")]
    pub r#compute_count: Box<Option<f64>>,
    /// (Output)
    /// The connection string used to connect to the Autonomous Database.
    /// https://docs.oracle.com/en-us/iaas/api/#/en/database/20160918/datatypes/AutonomousDatabaseConnectionStrings
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "connectionStrings")]
    pub r#connection_strings: Box<Option<Vec<super::super::types::oracledatabase::AutonomousDatabasePropertiesConnectionString>>>,
    /// (Output)
    /// The URLs for accessing Oracle Application Express (APEX) and SQL Developer
    /// Web with a browser from a Compute instance.
    /// https://docs.oracle.com/en-us/iaas/api/#/en/database/20160918/datatypes/AutonomousDatabaseConnectionUrls
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "connectionUrls")]
    pub r#connection_urls: Box<Option<Vec<super::super::types::oracledatabase::AutonomousDatabasePropertiesConnectionUrl>>>,
    /// The list of customer contacts.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "customerContacts")]
    pub r#customer_contacts: Box<Option<Vec<super::super::types::oracledatabase::AutonomousDatabasePropertiesCustomerContact>>>,
    /// (Output)
    /// The current state of the Data Safe registration for the
    /// Autonomous Database.
    /// Possible values:
    /// DATA_SAFE_STATE_UNSPECIFIED
    /// REGISTERING
    /// REGISTERED
    /// DEREGISTERING
    /// NOT_REGISTERED
    /// FAILED
    #[builder(into, default)]
    #[serde(rename = "dataSafeState")]
    pub r#data_safe_state: Box<Option<String>>,
    /// The size of the data stored in the database, in gigabytes.
    #[builder(into, default)]
    #[serde(rename = "dataStorageSizeGb")]
    pub r#data_storage_size_gb: Box<Option<i32>>,
    /// The size of the data stored in the database, in terabytes.
    #[builder(into, default)]
    #[serde(rename = "dataStorageSizeTb")]
    pub r#data_storage_size_tb: Box<Option<i32>>,
    /// (Output)
    /// The current state of database management for the Autonomous Database.
    /// Possible values:
    /// DATABASE_MANAGEMENT_STATE_UNSPECIFIED
    /// ENABLING
    /// ENABLED
    /// DISABLING
    /// NOT_ENABLED
    /// FAILED_ENABLING
    /// FAILED_DISABLING
    #[builder(into, default)]
    #[serde(rename = "databaseManagementState")]
    pub r#database_management_state: Box<Option<String>>,
    /// The edition of the Autonomous Databases.
    /// Possible values:
    /// DATABASE_EDITION_UNSPECIFIED
    /// STANDARD_EDITION
    /// ENTERPRISE_EDITION
    #[builder(into, default)]
    #[serde(rename = "dbEdition")]
    pub r#db_edition: Box<Option<String>>,
    /// The Oracle Database version for the Autonomous Database.
    #[builder(into, default)]
    #[serde(rename = "dbVersion")]
    pub r#db_version: Box<Option<String>>,
    /// Possible values:
    /// DB_WORKLOAD_UNSPECIFIED
    /// OLTP
    /// DW
    /// AJD
    /// APEX
    #[builder(into)]
    #[serde(rename = "dbWorkload")]
    pub r#db_workload: Box<String>,
    /// (Output)
    /// This field indicates the number of seconds of data loss during a Data
    /// Guard failover.
    #[builder(into, default)]
    #[serde(rename = "failedDataRecoveryDuration")]
    pub r#failed_data_recovery_duration: Box<Option<String>>,
    /// This field indicates if auto scaling is enabled for the Autonomous Database
    /// CPU core count.
    #[builder(into, default)]
    #[serde(rename = "isAutoScalingEnabled")]
    pub r#is_auto_scaling_enabled: Box<Option<bool>>,
    /// (Output)
    /// This field indicates whether the Autonomous Database has local (in-region)
    /// Data Guard enabled.
    #[builder(into, default)]
    #[serde(rename = "isLocalDataGuardEnabled")]
    pub r#is_local_data_guard_enabled: Box<Option<bool>>,
    /// This field indicates if auto scaling is enabled for the Autonomous Database
    /// storage.
    #[builder(into, default)]
    #[serde(rename = "isStorageAutoScalingEnabled")]
    pub r#is_storage_auto_scaling_enabled: Box<Option<bool>>,
    /// The license type used for the Autonomous Database.
    /// Possible values:
    /// LICENSE_TYPE_UNSPECIFIED
    /// LICENSE_INCLUDED
    /// BRING_YOUR_OWN_LICENSE
    #[builder(into)]
    #[serde(rename = "licenseType")]
    pub r#license_type: Box<String>,
    /// (Output)
    /// The details of the current lifestyle state of the Autonomous Database.
    #[builder(into, default)]
    #[serde(rename = "lifecycleDetails")]
    pub r#lifecycle_details: Box<Option<String>>,
    /// (Output)
    /// This field indicates the maximum data loss limit for an Autonomous
    /// Database, in seconds.
    #[builder(into, default)]
    #[serde(rename = "localAdgAutoFailoverMaxDataLossLimit")]
    pub r#local_adg_auto_failover_max_data_loss_limit: Box<Option<i32>>,
    /// (Output)
    /// This field indicates the local disaster recovery (DR) type of an
    /// Autonomous Database.
    /// Possible values:
    /// LOCAL_DISASTER_RECOVERY_TYPE_UNSPECIFIED
    /// ADG
    /// BACKUP_BASED
    #[builder(into, default)]
    #[serde(rename = "localDisasterRecoveryType")]
    pub r#local_disaster_recovery_type: Box<Option<String>>,
    /// (Output)
    /// Autonomous Data Guard standby database details.
    /// https://docs.oracle.com/en-us/iaas/api/#/en/database/20160918/datatypes/AutonomousDatabaseStandbySummary
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "localStandbyDbs")]
    pub r#local_standby_dbs: Box<Option<Vec<super::super::types::oracledatabase::AutonomousDatabasePropertiesLocalStandbyDb>>>,
    /// (Output)
    /// The date and time when maintenance will begin.
    #[builder(into, default)]
    #[serde(rename = "maintenanceBeginTime")]
    pub r#maintenance_begin_time: Box<Option<String>>,
    /// (Output)
    /// The date and time when maintenance will end.
    #[builder(into, default)]
    #[serde(rename = "maintenanceEndTime")]
    pub r#maintenance_end_time: Box<Option<String>>,
    /// The maintenance schedule of the Autonomous Database.
    /// Possible values:
    /// MAINTENANCE_SCHEDULE_TYPE_UNSPECIFIED
    /// EARLY
    /// REGULAR
    #[builder(into, default)]
    #[serde(rename = "maintenanceScheduleType")]
    pub r#maintenance_schedule_type: Box<Option<String>>,
    /// (Output)
    /// The amount of memory enabled per ECPU, in gigabytes.
    #[builder(into, default)]
    #[serde(rename = "memoryPerOracleComputeUnitGbs")]
    pub r#memory_per_oracle_compute_unit_gbs: Box<Option<i32>>,
    /// (Output)
    /// The memory assigned to in-memory tables in an Autonomous Database.
    #[builder(into, default)]
    #[serde(rename = "memoryTableGbs")]
    pub r#memory_table_gbs: Box<Option<i32>>,
    /// This field specifies if the Autonomous Database requires mTLS connections.
    #[builder(into, default)]
    #[serde(rename = "mtlsConnectionRequired")]
    pub r#mtls_connection_required: Box<Option<bool>>,
    /// The national character set for the Autonomous Database. The default is
    /// AL16UTF16.
    #[builder(into, default)]
    #[serde(rename = "nCharacterSet")]
    pub r#n_character_set: Box<Option<String>>,
    /// (Output)
    /// The long term backup schedule of the Autonomous Database.
    #[builder(into, default)]
    #[serde(rename = "nextLongTermBackupTime")]
    pub r#next_long_term_backup_time: Box<Option<String>>,
    /// (Output)
    /// The Oracle Cloud Infrastructure link for the Autonomous Database.
    #[builder(into, default)]
    #[serde(rename = "ociUrl")]
    pub r#oci_url: Box<Option<String>>,
    /// (Output)
    /// OCID of the Autonomous Database.
    /// https://docs.oracle.com/en-us/iaas/Content/General/Concepts/identifiers.htm#Oracle
    #[builder(into, default)]
    #[serde(rename = "ocid")]
    pub r#ocid: Box<Option<String>>,
    /// (Output)
    /// This field indicates the current mode of the Autonomous Database.
    /// Possible values:
    /// OPEN_MODE_UNSPECIFIED
    /// READ_ONLY
    /// READ_WRITE
    #[builder(into, default)]
    #[serde(rename = "openMode")]
    pub r#open_mode: Box<Option<String>>,
    /// Possible values:
    /// OPERATIONS_INSIGHTS_STATE_UNSPECIFIED
    /// ENABLING
    /// ENABLED
    /// DISABLING
    /// NOT_ENABLED
    /// FAILED_ENABLING
    /// FAILED_DISABLING
    #[builder(into, default)]
    #[serde(rename = "operationsInsightsState")]
    pub r#operations_insights_state: Box<Option<String>>,
    /// (Output)
    /// The list of OCIDs of standby databases located in Autonomous Data Guard
    /// remote regions that are associated with the source database.
    #[builder(into, default)]
    #[serde(rename = "peerDbIds")]
    pub r#peer_db_ids: Box<Option<Vec<String>>>,
    /// (Output)
    /// The permission level of the Autonomous Database.
    /// Possible values:
    /// PERMISSION_LEVEL_UNSPECIFIED
    /// RESTRICTED
    /// UNRESTRICTED
    #[builder(into, default)]
    #[serde(rename = "permissionLevel")]
    pub r#permission_level: Box<Option<String>>,
    /// (Output)
    /// The private endpoint for the Autonomous Database.
    #[builder(into, default)]
    #[serde(rename = "privateEndpoint")]
    pub r#private_endpoint: Box<Option<String>>,
    /// The private endpoint IP address for the Autonomous Database.
    #[builder(into, default)]
    #[serde(rename = "privateEndpointIp")]
    pub r#private_endpoint_ip: Box<Option<String>>,
    /// The private endpoint label for the Autonomous Database.
    #[builder(into, default)]
    #[serde(rename = "privateEndpointLabel")]
    pub r#private_endpoint_label: Box<Option<String>>,
    /// (Output)
    /// The refresh mode of the cloned Autonomous Database.
    /// Possible values:
    /// REFRESHABLE_MODE_UNSPECIFIED
    /// AUTOMATIC
    /// MANUAL
    #[builder(into, default)]
    #[serde(rename = "refreshableMode")]
    pub r#refreshable_mode: Box<Option<String>>,
    /// (Output)
    /// The refresh State of the clone.
    /// Possible values:
    /// REFRESHABLE_STATE_UNSPECIFIED
    /// REFRESHING
    /// NOT_REFRESHING
    #[builder(into, default)]
    #[serde(rename = "refreshableState")]
    pub r#refreshable_state: Box<Option<String>>,
    /// (Output)
    /// The Data Guard role of the Autonomous Database.
    /// Possible values:
    /// ROLE_UNSPECIFIED
    /// PRIMARY
    /// STANDBY
    /// DISABLED_STANDBY
    /// BACKUP_COPY
    /// SNAPSHOT_STANDBY
    #[builder(into, default)]
    #[serde(rename = "role")]
    pub r#role: Box<Option<String>>,
    /// (Output)
    /// The list and details of the scheduled operations of the Autonomous
    /// Database.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "scheduledOperationDetails")]
    pub r#scheduled_operation_details: Box<Option<Vec<super::super::types::oracledatabase::AutonomousDatabasePropertiesScheduledOperationDetail>>>,
    /// (Output)
    /// The SQL Web Developer URL for the Autonomous Database.
    #[builder(into, default)]
    #[serde(rename = "sqlWebDeveloperUrl")]
    pub r#sql_web_developer_url: Box<Option<String>>,
    /// (Output)
    /// Possible values:
    /// STATE_UNSPECIFIED
    /// PROVISIONING
    /// AVAILABLE
    /// STOPPING
    /// STOPPED
    /// STARTING
    /// TERMINATING
    /// TERMINATED
    /// UNAVAILABLE
    /// RESTORE_IN_PROGRESS
    /// RESTORE_FAILED
    /// BACKUP_IN_PROGRESS
    /// SCALE_IN_PROGRESS
    /// AVAILABLE_NEEDS_ATTENTION
    /// UPDATING
    /// MAINTENANCE_IN_PROGRESS
    /// RESTARTING
    /// RECREATING
    /// ROLE_CHANGE_IN_PROGRESS
    /// UPGRADING
    /// INACCESSIBLE
    /// STANDBY
    #[builder(into, default)]
    #[serde(rename = "state")]
    pub r#state: Box<Option<String>>,
    /// (Output)
    /// The list of available regions that can be used to create a clone for the
    /// Autonomous Database.
    #[builder(into, default)]
    #[serde(rename = "supportedCloneRegions")]
    pub r#supported_clone_regions: Box<Option<Vec<String>>>,
    /// (Output)
    /// The storage space used by automatic backups of Autonomous Database, in
    /// gigabytes.
    #[builder(into, default)]
    #[serde(rename = "totalAutoBackupStorageSizeGbs")]
    pub r#total_auto_backup_storage_size_gbs: Box<Option<f64>>,
    /// (Output)
    /// The storage space used by Autonomous Database, in gigabytes.
    #[builder(into, default)]
    #[serde(rename = "usedDataStorageSizeTbs")]
    pub r#used_data_storage_size_tbs: Box<Option<i32>>,
}
