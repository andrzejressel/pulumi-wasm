#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct InstanceCluster {
    /// [Autoscaling](https://cloud.google.com/bigtable/docs/autoscaling#parameters) config for the cluster, contains the following arguments:
    #[builder(into, default)]
    #[serde(rename = "autoscalingConfig")]
    pub r#autoscaling_config: Box<Option<super::super::types::bigtable::InstanceClusterAutoscalingConfig>>,
    /// The ID of the Cloud Bigtable cluster. Must be 6-30 characters and must only contain hyphens, lowercase letters and numbers.
    #[builder(into)]
    #[serde(rename = "clusterId")]
    pub r#cluster_id: Box<String>,
    /// Describes the Cloud KMS encryption key that will be used to protect the destination Bigtable cluster. The requirements for this key are: 1) The Cloud Bigtable service account associated with the project that contains this cluster must be granted the `cloudkms.cryptoKeyEncrypterDecrypter` role on the CMEK key. 2) Only regional keys can be used and the region of the CMEK key must match the region of the cluster.
    /// 
    /// > **Note**: Removing the field entirely from the config will cause the provider to default to the backend value.
    /// 
    /// !> **Warning**: Modifying this field will cause the provider to delete/recreate the entire resource.
    /// 
    /// !> **Warning:** Modifying the `storage_type`, `zone` or `kms_key_name` of an existing cluster (by
    /// `cluster_id`) will cause the provider to delete/recreate the entire
    /// `gcp.bigtable.Instance` resource. If these values are changing, use a new
    /// `cluster_id`.
    #[builder(into, default)]
    #[serde(rename = "kmsKeyName")]
    pub r#kms_key_name: Box<Option<String>>,
    /// The number of nodes in the cluster.
    /// If no value is set, Cloud Bigtable automatically allocates nodes based on your data footprint and optimized for 50% storage utilization.
    #[builder(into, default)]
    #[serde(rename = "numNodes")]
    pub r#num_nodes: Box<Option<i32>>,
    /// describes the current state of the cluster.
    #[builder(into, default)]
    #[serde(rename = "state")]
    pub r#state: Box<Option<String>>,
    /// The storage type to use. One of `"SSD"` or
    /// `"HDD"`. Defaults to `"SSD"`.
    #[builder(into, default)]
    #[serde(rename = "storageType")]
    pub r#storage_type: Box<Option<String>>,
    /// The zone to create the Cloud Bigtable cluster in. If it not
    /// specified, the provider zone is used. Each cluster must have a different zone in the same region. Zones that support
    /// Bigtable instances are noted on the [Cloud Bigtable locations page](https://cloud.google.com/bigtable/docs/locations).
    #[builder(into, default)]
    #[serde(rename = "zone")]
    pub r#zone: Box<Option<String>>,
}
