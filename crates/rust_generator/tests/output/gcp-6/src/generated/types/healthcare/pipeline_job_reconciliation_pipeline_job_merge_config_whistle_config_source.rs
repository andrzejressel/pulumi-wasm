#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct PipelineJobReconciliationPipelineJobMergeConfigWhistleConfigSource {
    /// Directory path where all the Whistle files are located.
    /// Example: gs://{bucket-id}/{path/to/import-root/dir}
    #[builder(into)]
    #[serde(rename = "importUriPrefix")]
    pub r#import_uri_prefix: Box<String>,
    /// Main configuration file which has the entrypoint or the root function.
    /// Example: gs://{bucket-id}/{path/to/import-root/dir}/entrypoint-file-name.wstl.
    #[builder(into)]
    #[serde(rename = "uri")]
    pub r#uri: Box<String>,
}
