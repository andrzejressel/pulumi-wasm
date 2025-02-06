#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetKeyMultiRegionConfiguration {
    /// Indicates whether the KMS key is a `PRIMARY` or `REPLICA` key.
    #[builder(into)]
    #[serde(rename = "multiRegionKeyType")]
    pub r#multi_region_key_type: Box<String>,
    /// The key ARN and Region of the primary key. This is the current KMS key if it is the primary key.
    #[builder(into)]
    #[serde(rename = "primaryKeys")]
    pub r#primary_keys: Box<Vec<super::super::types::kms::GetKeyMultiRegionConfigurationPrimaryKey>>,
    /// The key ARNs and Regions of all replica keys. Includes the current KMS key if it is a replica key.
    #[builder(into)]
    #[serde(rename = "replicaKeys")]
    pub r#replica_keys: Box<Vec<super::super::types::kms::GetKeyMultiRegionConfigurationReplicaKey>>,
}
