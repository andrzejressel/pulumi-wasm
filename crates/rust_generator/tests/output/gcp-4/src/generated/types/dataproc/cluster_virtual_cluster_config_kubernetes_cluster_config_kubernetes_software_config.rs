#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ClusterVirtualClusterConfigKubernetesClusterConfigKubernetesSoftwareConfig {
    /// The components that should be installed in this Dataproc cluster. The key must be a string from the   
    /// KubernetesComponent enumeration. The value is the version of the software to be installed. At least one entry must be specified.
    /// * **NOTE** : `component_version[SPARK]` is mandatory to set, or the creation of the cluster will fail.
    #[builder(into)]
    #[serde(rename = "componentVersion")]
    pub r#component_version: Box<std::collections::HashMap<String, String>>,
    /// The properties to set on daemon config files. Property keys are specified in prefix:property format, 
    /// for example spark:spark.kubernetes.container.image.
    #[builder(into, default)]
    #[serde(rename = "properties")]
    pub r#properties: Box<Option<std::collections::HashMap<String, String>>>,
}
