#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct V2ModelsSlotTypeExternalSourceSettingGrammarSlotTypeSettingSource {
    /// KMS key required to decrypt the contents of the grammar, if any.
    #[builder(into)]
    #[serde(rename = "kmsKeyArn")]
    pub r#kms_key_arn: Box<String>,
    /// Name of the Amazon S3 bucket that contains the grammar source.
    #[builder(into)]
    #[serde(rename = "s3BucketName")]
    pub r#s_3_bucket_name: Box<String>,
    /// Path to the grammar in the Amazon S3 bucket.
    #[builder(into)]
    #[serde(rename = "s3ObjectKey")]
    pub r#s_3_object_key: Box<String>,
}
