#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
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
