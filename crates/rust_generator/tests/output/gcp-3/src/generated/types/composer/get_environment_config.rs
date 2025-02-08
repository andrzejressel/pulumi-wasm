#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct GetEnvironmentConfig {
    /// The URI of the Apache Airflow Web UI hosted within the
    /// environment.
    #[builder(into)]
    #[serde(rename = "airflowUri")]
    pub r#airflow_uri: Box<String>,
    /// The Cloud Storage prefix of the DAGs for the environment.
    #[builder(into)]
    #[serde(rename = "dagGcsPrefix")]
    pub r#dag_gcs_prefix: Box<String>,
    /// The configuration setting for Airflow data retention mechanism. This field is supported for Cloud Composer environments in versions composer-2.0.32-airflow-2.1.4. or newer
    #[builder(into)]
    #[serde(rename = "dataRetentionConfigs")]
    pub r#data_retention_configs: Box<Vec<super::super::types::composer::GetEnvironmentConfigDataRetentionConfig>>,
    /// The configuration of Cloud SQL instance that is used by the Apache Airflow software. This field is supported for Cloud Composer environments in versions composer-1.*.*-airflow-*.*.*.
    #[builder(into)]
    #[serde(rename = "databaseConfigs")]
    pub r#database_configs: Box<Vec<super::super::types::composer::GetEnvironmentConfigDatabaseConfig>>,
    /// Optional. If true, builds performed during operations that install Python packages have only private connectivity to Google services. If false, the builds also have access to the internet.
    #[builder(into)]
    #[serde(rename = "enablePrivateBuildsOnly")]
    pub r#enable_private_builds_only: Box<bool>,
    /// Optional. If true, a private Composer environment will be created.
    #[builder(into)]
    #[serde(rename = "enablePrivateEnvironment")]
    pub r#enable_private_environment: Box<bool>,
    /// The encryption options for the Composer environment and its dependencies.
    #[builder(into)]
    #[serde(rename = "encryptionConfigs")]
    pub r#encryption_configs: Box<Vec<super::super::types::composer::GetEnvironmentConfigEncryptionConfig>>,
    /// The size of the Cloud Composer environment. This field is supported for Cloud Composer environments in versions composer-2.*.*-airflow-*.*.* and newer.
    #[builder(into)]
    #[serde(rename = "environmentSize")]
    pub r#environment_size: Box<String>,
    /// The Kubernetes Engine cluster used to run the environment.
    #[builder(into)]
    #[serde(rename = "gkeCluster")]
    pub r#gke_cluster: Box<String>,
    /// The configuration for Cloud Composer maintenance window.
    #[builder(into)]
    #[serde(rename = "maintenanceWindows")]
    pub r#maintenance_windows: Box<Vec<super::super::types::composer::GetEnvironmentConfigMaintenanceWindow>>,
    /// Configuration options for the master authorized networks feature. Enabled master authorized networks will disallow all external traffic to access Kubernetes master through HTTPS except traffic from the given CIDR blocks, Google Compute Engine Public IPs and Google Prod IPs.
    #[builder(into)]
    #[serde(rename = "masterAuthorizedNetworksConfigs")]
    pub r#master_authorized_networks_configs: Box<Vec<super::super::types::composer::GetEnvironmentConfigMasterAuthorizedNetworksConfig>>,
    /// The configuration used for the Kubernetes Engine cluster.
    #[builder(into)]
    #[serde(rename = "nodeConfigs")]
    pub r#node_configs: Box<Vec<super::super::types::composer::GetEnvironmentConfigNodeConfig>>,
    /// The number of nodes in the Kubernetes Engine cluster that will be used to run this environment. This field is supported for Cloud Composer environments in versions composer-1.*.*-airflow-*.*.*.
    #[builder(into)]
    #[serde(rename = "nodeCount")]
    pub r#node_count: Box<i32>,
    /// The configuration used for the Private IP Cloud Composer environment.
    #[builder(into)]
    #[serde(rename = "privateEnvironmentConfigs")]
    pub r#private_environment_configs: Box<Vec<super::super::types::composer::GetEnvironmentConfigPrivateEnvironmentConfig>>,
    /// The recovery configuration settings for the Cloud Composer environment
    #[builder(into)]
    #[serde(rename = "recoveryConfigs")]
    pub r#recovery_configs: Box<Vec<super::super::types::composer::GetEnvironmentConfigRecoveryConfig>>,
    /// Whether high resilience is enabled or not. This field is supported for Cloud Composer environments in versions composer-2.1.15-airflow-*.*.* and newer.
    #[builder(into)]
    #[serde(rename = "resilienceMode")]
    pub r#resilience_mode: Box<String>,
    /// The configuration settings for software inside the environment.
    #[builder(into)]
    #[serde(rename = "softwareConfigs")]
    pub r#software_configs: Box<Vec<super::super::types::composer::GetEnvironmentConfigSoftwareConfig>>,
    /// The configuration settings for the Airflow web server App Engine instance. This field is supported for Cloud Composer environments in versions composer-1.*.*-airflow-*.*.*.
    #[builder(into)]
    #[serde(rename = "webServerConfigs")]
    pub r#web_server_configs: Box<Vec<super::super::types::composer::GetEnvironmentConfigWebServerConfig>>,
    /// Network-level access control policy for the Airflow web server.
    #[builder(into)]
    #[serde(rename = "webServerNetworkAccessControls")]
    pub r#web_server_network_access_controls: Box<Vec<super::super::types::composer::GetEnvironmentConfigWebServerNetworkAccessControl>>,
    /// The workloads configuration settings for the GKE cluster associated with the Cloud Composer environment. Supported for Cloud Composer environments in versions composer-2.*.*-airflow-*.*.* and newer.
    #[builder(into)]
    #[serde(rename = "workloadsConfigs")]
    pub r#workloads_configs: Box<Vec<super::super::types::composer::GetEnvironmentConfigWorkloadsConfig>>,
}
