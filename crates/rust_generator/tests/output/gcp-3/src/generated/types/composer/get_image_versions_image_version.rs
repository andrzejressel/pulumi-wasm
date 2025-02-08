#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct GetImageVersionsImageVersion {
    /// The string identifier of the image version, in the form: "composer-x.y.z-airflow-a.b.c"
    #[builder(into)]
    #[serde(rename = "imageVersionId")]
    pub r#image_version_id: Box<String>,
    /// Supported python versions for this image version
    #[builder(into)]
    #[serde(rename = "supportedPythonVersions")]
    pub r#supported_python_versions: Box<Vec<String>>,
}
