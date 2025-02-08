#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct LinkedServiceAzureDatabricksNewClusterConfig {
    /// Spark version of a the cluster.
    #[builder(into)]
    #[serde(rename = "clusterVersion")]
    pub r#cluster_version: Box<String>,
    /// Tags for the cluster resource.
    #[builder(into, default)]
    #[serde(rename = "customTags")]
    pub r#custom_tags: Box<Option<std::collections::HashMap<String, String>>>,
    /// Driver node type for the cluster.
    #[builder(into, default)]
    #[serde(rename = "driverNodeType")]
    pub r#driver_node_type: Box<Option<String>>,
    /// User defined initialization scripts for the cluster.
    #[builder(into, default)]
    #[serde(rename = "initScripts")]
    pub r#init_scripts: Box<Option<Vec<String>>>,
    /// Location to deliver Spark driver, worker, and event logs.
    #[builder(into, default)]
    #[serde(rename = "logDestination")]
    pub r#log_destination: Box<Option<String>>,
    /// Specifies the maximum number of worker nodes. It should be between 1 and 25000.
    #[builder(into, default)]
    #[serde(rename = "maxNumberOfWorkers")]
    pub r#max_number_of_workers: Box<Option<i32>>,
    /// Specifies the minimum number of worker nodes. It should be between 1 and 25000. It defaults to `1`.
    #[builder(into, default)]
    #[serde(rename = "minNumberOfWorkers")]
    pub r#min_number_of_workers: Box<Option<i32>>,
    /// Node type for the new cluster.
    #[builder(into)]
    #[serde(rename = "nodeType")]
    pub r#node_type: Box<String>,
    /// User-specified Spark configuration variables key-value pairs.
    #[builder(into, default)]
    #[serde(rename = "sparkConfig")]
    pub r#spark_config: Box<Option<std::collections::HashMap<String, String>>>,
    /// User-specified Spark environment variables key-value pairs.
    #[builder(into, default)]
    #[serde(rename = "sparkEnvironmentVariables")]
    pub r#spark_environment_variables: Box<Option<std::collections::HashMap<String, String>>>,
}
