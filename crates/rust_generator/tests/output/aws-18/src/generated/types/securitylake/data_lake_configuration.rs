#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct DataLakeConfiguration {
    /// Provides encryption details of Amazon Security Lake object.
    #[builder(into, default)]
    #[serde(rename = "encryptionConfigurations")]
    pub r#encryption_configurations: Box<Option<Vec<super::super::types::securitylake::DataLakeConfigurationEncryptionConfiguration>>>,
    /// Provides lifecycle details of Amazon Security Lake object.
    #[builder(into, default)]
    #[serde(rename = "lifecycleConfiguration")]
    pub r#lifecycle_configuration: Box<Option<super::super::types::securitylake::DataLakeConfigurationLifecycleConfiguration>>,
    /// The AWS Regions where Security Lake is automatically enabled.
    #[builder(into)]
    #[serde(rename = "region")]
    pub r#region: Box<String>,
    /// Provides replication details of Amazon Security Lake object.
    #[builder(into, default)]
    #[serde(rename = "replicationConfiguration")]
    pub r#replication_configuration: Box<Option<super::super::types::securitylake::DataLakeConfigurationReplicationConfiguration>>,
}
