#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ConnectionProfileCloudsqlSettings {
    /// The activation policy specifies when the instance is activated; it is applicable only when the instance state is 'RUNNABLE'.
    /// Possible values are: `ALWAYS`, `NEVER`.
    #[builder(into, default)]
    #[serde(rename = "activationPolicy")]
    pub r#activation_policy: Box<Option<String>>,
    /// If you enable this setting, Cloud SQL checks your available storage every 30 seconds. If the available storage falls below a threshold size, Cloud SQL automatically adds additional storage capacity.
    /// If the available storage repeatedly falls below the threshold size, Cloud SQL continues to add storage until it reaches the maximum of 30 TB.
    #[builder(into, default)]
    #[serde(rename = "autoStorageIncrease")]
    pub r#auto_storage_increase: Box<Option<bool>>,
    /// The KMS key name used for the csql instance.
    #[builder(into, default)]
    #[serde(rename = "cmekKeyName")]
    pub r#cmek_key_name: Box<Option<String>>,
    /// The Cloud SQL default instance level collation.
    #[builder(into, default)]
    #[serde(rename = "collation")]
    pub r#collation: Box<Option<String>>,
    /// The storage capacity available to the database, in GB. The minimum (and default) size is 10GB.
    #[builder(into, default)]
    #[serde(rename = "dataDiskSizeGb")]
    pub r#data_disk_size_gb: Box<Option<String>>,
    /// The type of storage.
    /// Possible values are: `PD_SSD`, `PD_HDD`.
    #[builder(into, default)]
    #[serde(rename = "dataDiskType")]
    pub r#data_disk_type: Box<Option<String>>,
    /// The database flags passed to the Cloud SQL instance at startup.
    #[builder(into, default)]
    #[serde(rename = "databaseFlags")]
    pub r#database_flags: Box<Option<std::collections::HashMap<String, String>>>,
    /// The database engine type and version.
    /// Currently supported values located at https://cloud.google.com/database-migration/docs/reference/rest/v1/projects.locations.connectionProfiles#sqldatabaseversion
    #[builder(into, default)]
    #[serde(rename = "databaseVersion")]
    pub r#database_version: Box<Option<String>>,
    /// The edition of the given Cloud SQL instance.
    /// Possible values are: `ENTERPRISE`, `ENTERPRISE_PLUS`.
    #[builder(into, default)]
    #[serde(rename = "edition")]
    pub r#edition: Box<Option<String>>,
    /// The settings for IP Management. This allows to enable or disable the instance IP and manage which external networks can connect to the instance. The IPv4 address cannot be disabled.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "ipConfig")]
    pub r#ip_config: Box<Option<super::super::types::databasemigrationservice::ConnectionProfileCloudsqlSettingsIpConfig>>,
    /// Input only. Initial root password.
    /// **Note**: This property is sensitive and will not be displayed in the plan.
    #[builder(into, default)]
    #[serde(rename = "rootPassword")]
    pub r#root_password: Box<Option<String>>,
    /// (Output)
    /// Output only. Indicates If this connection profile root password is stored.
    #[builder(into, default)]
    #[serde(rename = "rootPasswordSet")]
    pub r#root_password_set: Box<Option<bool>>,
    /// The Database Migration Service source connection profile ID, in the format: projects/my_project_name/locations/us-central1/connectionProfiles/connection_profile_ID
    #[builder(into)]
    #[serde(rename = "sourceId")]
    pub r#source_id: Box<String>,
    /// The maximum size to which storage capacity can be automatically increased. The default value is 0, which specifies that there is no limit.
    #[builder(into, default)]
    #[serde(rename = "storageAutoResizeLimit")]
    pub r#storage_auto_resize_limit: Box<Option<String>>,
    /// The tier (or machine type) for this instance, for example: db-n1-standard-1 (MySQL instances) or db-custom-1-3840 (PostgreSQL instances).
    /// For more information, see https://cloud.google.com/sql/docs/mysql/instance-settings
    #[builder(into, default)]
    #[serde(rename = "tier")]
    pub r#tier: Box<Option<String>>,
    /// The resource labels for a Cloud SQL instance to use to annotate any related underlying resources such as Compute Engine VMs.
    #[builder(into, default)]
    #[serde(rename = "userLabels")]
    pub r#user_labels: Box<Option<std::collections::HashMap<String, String>>>,
    /// The Google Cloud Platform zone where your Cloud SQL datdabse instance is located.
    #[builder(into, default)]
    #[serde(rename = "zone")]
    pub r#zone: Box<Option<String>>,
}
