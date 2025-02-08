#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetDatabaseInstancesInstanceSettingIpConfiguration {
    /// The name of the allocated ip range for the private ip CloudSQL instance. For example: "google-managed-services-default". If set, the instance ip will be created in the allocated range. The range name must comply with RFC 1035. Specifically, the name must be 1-63 characters long and match the regular expression a-z?.
    #[builder(into)]
    #[serde(rename = "allocatedIpRange")]
    pub r#allocated_ip_range: Box<String>,
    #[builder(into)]
    #[serde(rename = "authorizedNetworks")]
    pub r#authorized_networks: Box<Vec<super::super::types::sql::GetDatabaseInstancesInstanceSettingIpConfigurationAuthorizedNetwork>>,
    /// Whether Google Cloud services such as BigQuery are allowed to access data in this Cloud SQL instance over a private IP connection. SQLSERVER database type is not supported.
    #[builder(into)]
    #[serde(rename = "enablePrivatePathForGoogleCloudServices")]
    pub r#enable_private_path_for_google_cloud_services: Box<bool>,
    /// Whether this Cloud SQL instance should be assigned a public IPV4 address. At least ipv4_enabled must be enabled or a private_network must be configured.
    #[builder(into)]
    #[serde(rename = "ipv4Enabled")]
    pub r#ipv_4_enabled: Box<bool>,
    /// The VPC network from which the Cloud SQL instance is accessible for private IP. For example, projects/myProject/global/networks/default. Specifying a network enables private IP. At least ipv4_enabled must be enabled or a private_network must be configured. This setting can be updated, but it cannot be removed after it is set.
    #[builder(into)]
    #[serde(rename = "privateNetwork")]
    pub r#private_network: Box<String>,
    /// PSC settings for a Cloud SQL instance.
    #[builder(into)]
    #[serde(rename = "pscConfigs")]
    pub r#psc_configs: Box<Vec<super::super::types::sql::GetDatabaseInstancesInstanceSettingIpConfigurationPscConfig>>,
    /// Specify how the server certificate's Certificate Authority is hosted.
    #[builder(into)]
    #[serde(rename = "serverCaMode")]
    pub r#server_ca_mode: Box<String>,
    /// Specify how SSL connection should be enforced in DB connections.
    #[builder(into)]
    #[serde(rename = "sslMode")]
    pub r#ssl_mode: Box<String>,
}
