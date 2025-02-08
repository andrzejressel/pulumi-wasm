#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ScriptStorageLocation {
    /// Name of your S3 bucket.
    #[builder(into)]
    #[serde(rename = "bucket")]
    pub r#bucket: Box<String>,
    /// Name of the zip file containing your script files.
    #[builder(into)]
    #[serde(rename = "key")]
    pub r#key: Box<String>,
    /// A specific version of the file. If not set, the latest version of the file is retrieved.
    #[builder(into, default)]
    #[serde(rename = "objectVersion")]
    pub r#object_version: Box<Option<String>>,
    /// ARN of the access role that allows Amazon GameLift to access your S3 bucket.
    #[builder(into)]
    #[serde(rename = "roleArn")]
    pub r#role_arn: Box<String>,
}
