#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ReplicationConfigurationReplicationConfiguration {
    /// The replication rules for a replication configuration. A maximum of 10 are allowed per `replication_configuration`. See Rule
    #[builder(into)]
    #[serde(rename = "rules")]
    pub r#rules: Box<Vec<super::super::types::ecr::ReplicationConfigurationReplicationConfigurationRule>>,
}
