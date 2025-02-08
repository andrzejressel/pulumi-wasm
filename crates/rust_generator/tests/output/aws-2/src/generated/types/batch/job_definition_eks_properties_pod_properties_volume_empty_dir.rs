#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct JobDefinitionEksPropertiesPodPropertiesVolumeEmptyDir {
    /// Medium to store the volume. The default value is an empty string, which uses the storage of the node.
    #[builder(into, default)]
    #[serde(rename = "medium")]
    pub r#medium: Box<Option<String>>,
    /// Maximum size of the volume. By default, there's no maximum size defined.
    #[builder(into)]
    #[serde(rename = "sizeLimit")]
    pub r#size_limit: Box<String>,
}
