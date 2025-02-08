#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct PipelineArtifactStore {
    /// The encryption key block AWS CodePipeline uses to encrypt the data in the artifact store, such as an AWS Key Management Service (AWS KMS) key. If you don't specify a key, AWS CodePipeline uses the default key for Amazon Simple Storage Service (Amazon S3). An `encryption_key` block is documented below.
    #[builder(into, default)]
    #[serde(rename = "encryptionKey")]
    pub r#encryption_key: Box<Option<super::super::types::codepipeline::PipelineArtifactStoreEncryptionKey>>,
    /// The location where AWS CodePipeline stores artifacts for a pipeline; currently only `S3` is supported.
    #[builder(into)]
    #[serde(rename = "location")]
    pub r#location: Box<String>,
    /// The region where the artifact store is located. Required for a cross-region CodePipeline, do not provide for a single-region CodePipeline.
    #[builder(into, default)]
    #[serde(rename = "region")]
    pub r#region: Box<Option<String>>,
    /// The type of the artifact store, such as Amazon S3
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: Box<String>,
}
