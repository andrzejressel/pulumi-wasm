#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct DataSourceConfigurationS3ConfigurationAccessControlListConfiguration {
    /// Path to the AWS S3 bucket that contains the ACL files.
    #[builder(into, default)]
    #[serde(rename = "keyPath")]
    pub r#key_path: Box<Option<String>>,
}
