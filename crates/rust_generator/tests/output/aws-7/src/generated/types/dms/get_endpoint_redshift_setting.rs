#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetEndpointRedshiftSetting {
    #[builder(into)]
    #[serde(rename = "bucketFolder")]
    pub r#bucket_folder: Box<String>,
    #[builder(into)]
    #[serde(rename = "bucketName")]
    pub r#bucket_name: Box<String>,
    #[builder(into)]
    #[serde(rename = "encryptionMode")]
    pub r#encryption_mode: Box<String>,
    #[builder(into)]
    #[serde(rename = "serverSideEncryptionKmsKeyId")]
    pub r#server_side_encryption_kms_key_id: Box<String>,
    #[builder(into)]
    #[serde(rename = "serviceAccessRoleArn")]
    pub r#service_access_role_arn: Box<String>,
}
