#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct GalleryApplicationVersionSource {
    /// The Storage Blob URI of the default configuration. Changing this forces a new resource to be created.
    #[builder(into, default)]
    #[serde(rename = "defaultConfigurationLink")]
    pub r#default_configuration_link: Box<Option<String>>,
    /// The Storage Blob URI of the source application package. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "mediaLink")]
    pub r#media_link: Box<String>,
}
