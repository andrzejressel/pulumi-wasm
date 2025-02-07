#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetReplicationSetRegion {
    /// The ARN of the AWS Key Management Service (AWS KMS) encryption key.
    #[builder(into)]
    #[serde(rename = "kmsKeyArn")]
    pub r#kms_key_arn: Box<String>,
    /// The name of the Region.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// The current status of the Region.
    /// * Valid Values: `ACTIVE` | `CREATING` | `UPDATING` | `DELETING` | `FAILED`
    #[builder(into)]
    #[serde(rename = "status")]
    pub r#status: Box<String>,
    /// More information about the status of a Region.
    #[builder(into)]
    #[serde(rename = "statusMessage")]
    pub r#status_message: Box<String>,
}
