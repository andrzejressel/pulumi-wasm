#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct JobDefinitionEksPropertiesPodPropertiesVolume {
    #[builder(into, default)]
    #[serde(rename = "emptyDir")]
    pub r#empty_dir: Box<Option<super::super::types::batch::JobDefinitionEksPropertiesPodPropertiesVolumeEmptyDir>>,
    #[builder(into, default)]
    #[serde(rename = "hostPath")]
    pub r#host_path: Box<Option<super::super::types::batch::JobDefinitionEksPropertiesPodPropertiesVolumeHostPath>>,
    /// Name of the job definition.
    #[builder(into, default)]
    #[serde(rename = "name")]
    pub r#name: Box<Option<String>>,
    #[builder(into, default)]
    #[serde(rename = "secret")]
    pub r#secret: Box<Option<super::super::types::batch::JobDefinitionEksPropertiesPodPropertiesVolumeSecret>>,
}
