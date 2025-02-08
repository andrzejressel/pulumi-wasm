#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetServiceTemplateSpecVolumeCsi {
    /// Unique name representing the type of file system to be created. Cloud Run supports the following values:
    ///   * gcsfuse.run.googleapis.com: Mount a Google Cloud Storage bucket using GCSFuse. This driver requires the
    ///     run.googleapis.com/execution-environment annotation to be unset or set to "gen2"
    #[builder(into)]
    #[serde(rename = "driver")]
    pub r#driver: Box<String>,
    /// If true, all mounts created from this volume will be read-only.
    #[builder(into)]
    #[serde(rename = "readOnly")]
    pub r#read_only: Box<bool>,
    /// Driver-specific attributes. The following options are supported for available drivers:
    ///   * gcsfuse.run.googleapis.com
    ///     * bucketName: The name of the Cloud Storage Bucket that backs this volume. The Cloud Run Service identity must have access to this bucket.
    #[builder(into)]
    #[serde(rename = "volumeAttributes")]
    pub r#volume_attributes: Box<std::collections::HashMap<String, String>>,
}
