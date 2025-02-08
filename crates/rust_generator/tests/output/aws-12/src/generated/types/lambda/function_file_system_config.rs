#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct FunctionFileSystemConfig {
    /// Amazon Resource Name (ARN) of the Amazon EFS Access Point that provides access to the file system.
    #[builder(into)]
    #[serde(rename = "arn")]
    pub r#arn: Box<String>,
    /// Path where the function can access the file system, starting with /mnt/.
    #[builder(into)]
    #[serde(rename = "localMountPath")]
    pub r#local_mount_path: Box<String>,
}
