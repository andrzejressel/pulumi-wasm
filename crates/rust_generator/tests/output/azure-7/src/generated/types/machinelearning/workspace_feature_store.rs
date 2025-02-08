#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct WorkspaceFeatureStore {
    /// The version of Spark runtime.
    #[builder(into, default)]
    #[serde(rename = "computerSparkRuntimeVersion")]
    pub r#computer_spark_runtime_version: Box<Option<String>>,
    /// The name of offline store connection.
    #[builder(into, default)]
    #[serde(rename = "offlineConnectionName")]
    pub r#offline_connection_name: Box<Option<String>>,
    /// The name of online store connection.
    /// 
    /// > **Note:** `feature_store` must be set when`kind` is `FeatureStore`
    #[builder(into, default)]
    #[serde(rename = "onlineConnectionName")]
    pub r#online_connection_name: Box<Option<String>>,
}
