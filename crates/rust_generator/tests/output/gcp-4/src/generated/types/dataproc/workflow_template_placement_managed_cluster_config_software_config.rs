#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct WorkflowTemplatePlacementManagedClusterConfigSoftwareConfig {
    /// The version of software inside the cluster. It must be one of the supported [Dataproc Versions](https://cloud.google.com/dataproc/docs/concepts/versioning/dataproc-versions#supported_dataproc_versions), such as "1.2" (including a subminor version, such as "1.2.29"), or the ["preview" version](https://cloud.google.com/dataproc/docs/concepts/versioning/dataproc-versions#other_versions). If unspecified, it defaults to the latest Debian version.
    #[builder(into, default)]
    #[serde(rename = "imageVersion")]
    pub r#image_version: Box<Option<String>>,
    /// The set of components to activate on the cluster.
    #[builder(into, default)]
    #[serde(rename = "optionalComponents")]
    pub r#optional_components: Box<Option<Vec<String>>>,
    /// The properties to set on daemon config files.
    /// 
    /// Property keys are specified in `prefix:property` format, for example `core:hadoop.tmp.dir`. The following are supported prefixes and their mappings:
    /// 
    /// * capacity-scheduler: `capacity-scheduler.xml`
    /// * core: `core-site.xml`
    /// * distcp: `distcp-default.xml`
    /// * hdfs: `hdfs-site.xml`
    /// * hive: `hive-site.xml`
    /// * mapred: `mapred-site.xml`
    /// * pig: `pig.properties`
    /// * spark: `spark-defaults.conf`
    /// * yarn: `yarn-site.xml`
    /// 
    /// 
    /// For more information, see [Cluster properties](https://cloud.google.com/dataproc/docs/concepts/cluster-properties).
    #[builder(into, default)]
    #[serde(rename = "properties")]
    pub r#properties: Box<Option<std::collections::HashMap<String, String>>>,
}
