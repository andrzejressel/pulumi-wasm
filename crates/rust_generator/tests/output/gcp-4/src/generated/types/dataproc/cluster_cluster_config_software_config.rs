#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ClusterClusterConfigSoftwareConfig {
    /// The Cloud Dataproc image version to use
    /// for the cluster - this controls the sets of software versions
    /// installed onto the nodes when you create clusters. If not specified, defaults to the
    /// latest version. For a list of valid versions see
    /// [Cloud Dataproc versions](https://cloud.google.com/dataproc/docs/concepts/dataproc-versions)
    #[builder(into, default)]
    #[serde(rename = "imageVersion")]
    pub r#image_version: Box<Option<String>>,
    /// The set of optional components to activate on the cluster. See [Available Optional Components](https://cloud.google.com/dataproc/docs/concepts/components/overview#available_optional_components).
    /// 
    /// - - -
    #[builder(into, default)]
    #[serde(rename = "optionalComponents")]
    pub r#optional_components: Box<Option<Vec<String>>>,
    /// A list of override and additional properties (key/value pairs)
    /// used to modify various aspects of the common configuration files used when creating
    /// a cluster. For a list of valid properties please see
    /// [Cluster properties](https://cloud.google.com/dataproc/docs/concepts/cluster-properties)
    #[builder(into, default)]
    #[serde(rename = "overrideProperties")]
    pub r#override_properties: Box<Option<std::collections::HashMap<String, String>>>,
    /// A list of the properties used to set the daemon config files.
    /// This will include any values supplied by the user via `cluster_config.software_config.override_properties`
    #[builder(into, default)]
    #[serde(rename = "properties")]
    pub r#properties: Box<Option<std::collections::HashMap<String, String>>>,
}
