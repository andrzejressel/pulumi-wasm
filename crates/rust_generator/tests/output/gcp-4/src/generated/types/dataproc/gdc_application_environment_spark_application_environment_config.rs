#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GdcApplicationEnvironmentSparkApplicationEnvironmentConfig {
    /// A map of default Spark properties to apply to workloads in this application environment. These defaults may be overridden by per-application properties.
    #[builder(into, default)]
    #[serde(rename = "defaultProperties")]
    pub r#default_properties: Box<Option<std::collections::HashMap<String, String>>>,
    /// The default Dataproc version to use for applications submitted to this application environment
    #[builder(into, default)]
    #[serde(rename = "defaultVersion")]
    pub r#default_version: Box<Option<String>>,
}
