#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct DatabaseInstanceSettings {
    /// This specifies when the instance should be
    /// active. Can be either `ALWAYS`, `NEVER` or `ON_DEMAND`.
    #[builder(into, default)]
    #[serde(rename = "activationPolicy")]
    pub r#activation_policy: Box<Option<String>>,
    #[builder(into, default)]
    #[serde(rename = "activeDirectoryConfig")]
    pub r#active_directory_config: Box<Option<super::super::types::sql::DatabaseInstanceSettingsActiveDirectoryConfig>>,
    #[builder(into, default)]
    #[serde(rename = "advancedMachineFeatures")]
    pub r#advanced_machine_features: Box<Option<super::super::types::sql::DatabaseInstanceSettingsAdvancedMachineFeatures>>,
    /// The availability type of the Cloud SQL
    /// instance, high availability (`REGIONAL`) or single zone (`ZONAL`).' For all instances, ensure that
    /// `settings.backup_configuration.enabled` is set to `true`.
    /// For MySQL instances, ensure that `settings.backup_configuration.binary_log_enabled` is set to `true`.
    /// For Postgres and SQL Server instances, ensure that `settings.backup_configuration.point_in_time_recovery_enabled`
    /// is set to `true`. Defaults to `ZONAL`.
    #[builder(into, default)]
    #[serde(rename = "availabilityType")]
    pub r#availability_type: Box<Option<String>>,
    #[builder(into, default)]
    #[serde(rename = "backupConfiguration")]
    pub r#backup_configuration: Box<Option<super::super::types::sql::DatabaseInstanceSettingsBackupConfiguration>>,
    /// The name of server instance collation.
    #[builder(into, default)]
    #[serde(rename = "collation")]
    pub r#collation: Box<Option<String>>,
    /// Control the enforcement of Cloud SQL Auth Proxy or Cloud SQL connectors for all the connections, can be `REQUIRED` or `NOT_REQUIRED`. If enabled, all the direct connections are rejected.
    #[builder(into, default)]
    #[serde(rename = "connectorEnforcement")]
    pub r#connector_enforcement: Box<Option<String>>,
    /// Data cache configurations.
    #[builder(into, default)]
    #[serde(rename = "dataCacheConfig")]
    pub r#data_cache_config: Box<Option<super::super::types::sql::DatabaseInstanceSettingsDataCacheConfig>>,
    #[builder(into, default)]
    #[serde(rename = "databaseFlags")]
    pub r#database_flags: Box<Option<Vec<super::super::types::sql::DatabaseInstanceSettingsDatabaseFlag>>>,
    /// Configuration to protect against accidental instance deletion.
    #[builder(into, default)]
    #[serde(rename = "deletionProtectionEnabled")]
    pub r#deletion_protection_enabled: Box<Option<bool>>,
    #[builder(into, default)]
    #[serde(rename = "denyMaintenancePeriod")]
    pub r#deny_maintenance_period: Box<Option<super::super::types::sql::DatabaseInstanceSettingsDenyMaintenancePeriod>>,
    /// Enables auto-resizing of the storage size. Defaults to `true`. Note that if `disk_size` is set, future `pulumi up` calls will attempt to delete the instance in order to resize the disk to the value specified in disk_size if it has been resized. To avoid this, ensure that `lifecycle.ignore_changes` is applied to `disk_size`.
    #[builder(into, default)]
    #[serde(rename = "diskAutoresize")]
    pub r#disk_autoresize: Box<Option<bool>>,
    /// The maximum size to which storage capacity can be automatically increased. The default value is 0, which specifies that there is no limit.
    #[builder(into, default)]
    #[serde(rename = "diskAutoresizeLimit")]
    pub r#disk_autoresize_limit: Box<Option<i32>>,
    /// The size of data disk, in GB. Size of a running instance cannot be reduced but can be increased. The minimum value is 10GB. Note that this value will override the resizing from `disk_autoresize` if that feature is enabled. To avoid this, set `lifecycle.ignore_changes` on this field.
    #[builder(into, default)]
    #[serde(rename = "diskSize")]
    pub r#disk_size: Box<Option<i32>>,
    /// The type of data disk: PD_SSD or PD_HDD. Defaults to `PD_SSD`.
    #[builder(into, default)]
    #[serde(rename = "diskType")]
    pub r#disk_type: Box<Option<String>>,
    /// The edition of the instance, can be `ENTERPRISE` or `ENTERPRISE_PLUS`.
    #[builder(into, default)]
    #[serde(rename = "edition")]
    pub r#edition: Box<Option<String>>,
    /// Enables [Cloud SQL instance integration with Dataplex](https://cloud.google.com/sql/docs/mysql/dataplex-catalog-integration). MySQL, Postgres and SQL Server instances are supported for this feature. Defaults to `false`.
    #[builder(into, default)]
    #[serde(rename = "enableDataplexIntegration")]
    pub r#enable_dataplex_integration: Box<Option<bool>>,
    /// Enables [Cloud SQL instances to connect to Vertex AI](https://cloud.google.com/sql/docs/postgres/integrate-cloud-sql-with-vertex-ai) and pass requests for real-time predictions and insights. Defaults to `false`.
    #[builder(into, default)]
    #[serde(rename = "enableGoogleMlIntegration")]
    pub r#enable_google_ml_integration: Box<Option<bool>>,
    /// Configuration of Query Insights.
    #[builder(into, default)]
    #[serde(rename = "insightsConfig")]
    pub r#insights_config: Box<Option<super::super::types::sql::DatabaseInstanceSettingsInsightsConfig>>,
    #[builder(into, default)]
    #[serde(rename = "ipConfiguration")]
    pub r#ip_configuration: Box<Option<super::super::types::sql::DatabaseInstanceSettingsIpConfiguration>>,
    #[builder(into, default)]
    #[serde(rename = "locationPreference")]
    pub r#location_preference: Box<Option<super::super::types::sql::DatabaseInstanceSettingsLocationPreference>>,
    /// Declares a one-hour maintenance window when an Instance can automatically restart to apply updates. The maintenance window is specified in UTC time.
    #[builder(into, default)]
    #[serde(rename = "maintenanceWindow")]
    pub r#maintenance_window: Box<Option<super::super::types::sql::DatabaseInstanceSettingsMaintenanceWindow>>,
    #[builder(into, default)]
    #[serde(rename = "passwordValidationPolicy")]
    pub r#password_validation_policy: Box<Option<super::super::types::sql::DatabaseInstanceSettingsPasswordValidationPolicy>>,
    /// Pricing plan for this instance, can only be `PER_USE`.
    #[builder(into, default)]
    #[serde(rename = "pricingPlan")]
    pub r#pricing_plan: Box<Option<String>>,
    #[builder(into, default)]
    #[serde(rename = "sqlServerAuditConfig")]
    pub r#sql_server_audit_config: Box<Option<super::super::types::sql::DatabaseInstanceSettingsSqlServerAuditConfig>>,
    /// The machine type to use. See [tiers](https://cloud.google.com/sql/docs/admin-api/v1beta4/tiers)
    /// for more details and supported versions. Postgres supports only shared-core machine types,
    /// and custom machine types such as `db-custom-2-13312`. See the [Custom Machine Type Documentation](https://cloud.google.com/compute/docs/instances/creating-instance-with-custom-machine-type#create) to learn about specifying custom machine types.
    #[builder(into)]
    #[serde(rename = "tier")]
    pub r#tier: Box<String>,
    /// The time_zone to be used by the database engine (supported only for SQL Server), in SQL Server timezone format.
    #[builder(into, default)]
    #[serde(rename = "timeZone")]
    pub r#time_zone: Box<Option<String>>,
    /// A set of key/value user label pairs to assign to the instance.
    #[builder(into, default)]
    #[serde(rename = "userLabels")]
    pub r#user_labels: Box<Option<std::collections::HashMap<String, String>>>,
    /// Used to make sure changes to the `settings` block are
    /// atomic.
    #[builder(into, default)]
    #[serde(rename = "version")]
    pub r#version: Box<Option<i32>>,
}
