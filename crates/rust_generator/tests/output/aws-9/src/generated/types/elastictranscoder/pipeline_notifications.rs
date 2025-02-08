#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct PipelineNotifications {
    /// The topic ARN for the Amazon SNS topic that you want to notify when Elastic Transcoder has finished processing a job in this pipeline.
    #[builder(into, default)]
    #[serde(rename = "completed")]
    pub r#completed: Box<Option<String>>,
    /// The topic ARN for the Amazon SNS topic that you want to notify when Elastic Transcoder encounters an error condition while processing a job in this pipeline.
    #[builder(into, default)]
    #[serde(rename = "error")]
    pub r#error: Box<Option<String>>,
    /// The topic ARN for the Amazon Simple Notification Service (Amazon SNS) topic that you want to notify when Elastic Transcoder has started to process a job in this pipeline.
    #[builder(into, default)]
    #[serde(rename = "progressing")]
    pub r#progressing: Box<Option<String>>,
    /// The topic ARN for the Amazon SNS topic that you want to notify when Elastic Transcoder encounters a warning condition while processing a job in this pipeline.
    /// 
    /// The `thumbnail_config` object specifies information about the Amazon S3 bucket in
    /// which you want Elastic Transcoder to save thumbnail files: which bucket to use,
    /// which users you want to have access to the files, the type of access you want
    /// users to have, and the storage class that you want to assign to the files. If
    /// you specify values for `content_config`, you must also specify values for
    /// `thumbnail_config` even if you don't want to create thumbnails. (You control
    /// whether to create thumbnails when you create a job. For more information, see
    /// ThumbnailPattern in the topic Create Job.) If you specify values for
    /// `content_config` and `thumbnail_config`, omit the OutputBucket object.
    #[builder(into, default)]
    #[serde(rename = "warning")]
    pub r#warning: Box<Option<String>>,
}
