#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetDatabaseInstancesInstanceSetting {
    /// This specifies when the instance should be active. Can be either ALWAYS, NEVER or ON_DEMAND.
    #[builder(into)]
    #[serde(rename = "activationPolicy")]
    pub r#activation_policy: Box<String>,
    #[builder(into)]
    #[serde(rename = "activeDirectoryConfigs")]
    pub r#active_directory_configs: Box<Vec<super::super::types::sql::GetDatabaseInstancesInstanceSettingActiveDirectoryConfig>>,
    #[builder(into)]
    #[serde(rename = "advancedMachineFeatures")]
    pub r#advanced_machine_features: Box<Vec<super::super::types::sql::GetDatabaseInstancesInstanceSettingAdvancedMachineFeature>>,
    /// The availability type of the Cloud SQL instance, high availability
    /// (REGIONAL) or single zone (ZONAL). For all instances, ensure that
    /// settings.backup_configuration.enabled is set to true.
    /// For MySQL instances, ensure that settings.backup_configuration.binary_log_enabled is set to true.
    /// For Postgres instances, ensure that settings.backup_configuration.point_in_time_recovery_enabled
    /// is set to true. Defaults to ZONAL.
    #[builder(into)]
    #[serde(rename = "availabilityType")]
    pub r#availability_type: Box<String>,
    #[builder(into)]
    #[serde(rename = "backupConfigurations")]
    pub r#backup_configurations: Box<Vec<super::super::types::sql::GetDatabaseInstancesInstanceSettingBackupConfiguration>>,
    /// The name of server instance collation.
    #[builder(into)]
    #[serde(rename = "collation")]
    pub r#collation: Box<String>,
    /// Enables the enforcement of Cloud SQL Auth Proxy or Cloud SQL connectors for all the connections. If enabled, all the direct connections are rejected.
    #[builder(into)]
    #[serde(rename = "connectorEnforcement")]
    pub r#connector_enforcement: Box<String>,
    /// Data cache configurations.
    #[builder(into)]
    #[serde(rename = "dataCacheConfigs")]
    pub r#data_cache_configs: Box<Vec<super::super::types::sql::GetDatabaseInstancesInstanceSettingDataCacheConfig>>,
    #[builder(into)]
    #[serde(rename = "databaseFlags")]
    pub r#database_flags: Box<Vec<super::super::types::sql::GetDatabaseInstancesInstanceSettingDatabaseFlag>>,
    /// Configuration to protect against accidental instance deletion.
    #[builder(into)]
    #[serde(rename = "deletionProtectionEnabled")]
    pub r#deletion_protection_enabled: Box<bool>,
    #[builder(into)]
    #[serde(rename = "denyMaintenancePeriods")]
    pub r#deny_maintenance_periods: Box<Vec<super::super::types::sql::GetDatabaseInstancesInstanceSettingDenyMaintenancePeriod>>,
    /// Enables auto-resizing of the storage size. Defaults to true.
    #[builder(into)]
    #[serde(rename = "diskAutoresize")]
    pub r#disk_autoresize: Box<bool>,
    /// The maximum size, in GB, to which storage capacity can be automatically increased. The default value is 0, which specifies that there is no limit.
    #[builder(into)]
    #[serde(rename = "diskAutoresizeLimit")]
    pub r#disk_autoresize_limit: Box<i32>,
    /// The size of data disk, in GB. Size of a running instance cannot be reduced but can be increased. The minimum value is 10GB.
    #[builder(into)]
    #[serde(rename = "diskSize")]
    pub r#disk_size: Box<i32>,
    /// The type of data disk: PD_SSD or PD_HDD. Defaults to PD_SSD.
    #[builder(into)]
    #[serde(rename = "diskType")]
    pub r#disk_type: Box<String>,
    /// The edition of the instance, can be ENTERPRISE or ENTERPRISE_PLUS.
    #[builder(into)]
    #[serde(rename = "edition")]
    pub r#edition: Box<String>,
    /// Enables Dataplex Integration.
    #[builder(into)]
    #[serde(rename = "enableDataplexIntegration")]
    pub r#enable_dataplex_integration: Box<bool>,
    /// Enables Vertex AI Integration.
    #[builder(into)]
    #[serde(rename = "enableGoogleMlIntegration")]
    pub r#enable_google_ml_integration: Box<bool>,
    /// Configuration of Query Insights.
    #[builder(into)]
    #[serde(rename = "insightsConfigs")]
    pub r#insights_configs: Box<Vec<super::super::types::sql::GetDatabaseInstancesInstanceSettingInsightsConfig>>,
    #[builder(into)]
    #[serde(rename = "ipConfigurations")]
    pub r#ip_configurations: Box<Vec<super::super::types::sql::GetDatabaseInstancesInstanceSettingIpConfiguration>>,
    #[builder(into)]
    #[serde(rename = "locationPreferences")]
    pub r#location_preferences: Box<Vec<super::super::types::sql::GetDatabaseInstancesInstanceSettingLocationPreference>>,
    /// Declares a one-hour maintenance window when an Instance can automatically restart to apply updates. The maintenance window is specified in UTC time.
    #[builder(into)]
    #[serde(rename = "maintenanceWindows")]
    pub r#maintenance_windows: Box<Vec<super::super::types::sql::GetDatabaseInstancesInstanceSettingMaintenanceWindow>>,
    #[builder(into)]
    #[serde(rename = "passwordValidationPolicies")]
    pub r#password_validation_policies: Box<Vec<super::super::types::sql::GetDatabaseInstancesInstanceSettingPasswordValidationPolicy>>,
    /// Pricing plan for this instance, can only be PER_USE.
    #[builder(into)]
    #[serde(rename = "pricingPlan")]
    pub r#pricing_plan: Box<String>,
    #[builder(into)]
    #[serde(rename = "sqlServerAuditConfigs")]
    pub r#sql_server_audit_configs: Box<Vec<super::super::types::sql::GetDatabaseInstancesInstanceSettingSqlServerAuditConfig>>,
    /// To filter out the Cloud SQL instances based on the tier(or machine type) of the database instances.
    #[builder(into)]
    #[serde(rename = "tier")]
    pub r#tier: Box<String>,
    /// The time_zone to be used by the database engine (supported only for SQL Server), in SQL Server timezone format.
    #[builder(into)]
    #[serde(rename = "timeZone")]
    pub r#time_zone: Box<String>,
    /// A set of key/value user label pairs to assign to the instance.
    #[builder(into)]
    #[serde(rename = "userLabels")]
    pub r#user_labels: Box<std::collections::HashMap<String, String>>,
    /// Used to make sure changes to the settings block are atomic.
    #[builder(into)]
    #[serde(rename = "version")]
    pub r#version: Box<i32>,
}
