#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct LinkedServiceAzureDatabricksInstancePool {
    /// Spark version of a the cluster.
    #[builder(into)]
    #[serde(rename = "clusterVersion")]
    pub r#cluster_version: Box<String>,
    /// Identifier of the instance pool within the linked ADB instance.
    #[builder(into)]
    #[serde(rename = "instancePoolId")]
    pub r#instance_pool_id: Box<String>,
    /// The max number of worker nodes. Set this value if you want to enable autoscaling between the `min_number_of_workers` and this value. Omit this value to use a fixed number of workers defined in the `min_number_of_workers` property.
    #[builder(into, default)]
    #[serde(rename = "maxNumberOfWorkers")]
    pub r#max_number_of_workers: Box<Option<i32>>,
    /// The minimum number of worker nodes. Defaults to `1`.
    #[builder(into, default)]
    #[serde(rename = "minNumberOfWorkers")]
    pub r#min_number_of_workers: Box<Option<i32>>,
}
