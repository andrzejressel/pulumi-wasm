#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct AccessPointRootDirectory {
    /// POSIX IDs and permissions to apply to the access point's Root Directory. See Creation Info below.
    #[builder(into, default)]
    #[serde(rename = "creationInfo")]
    pub r#creation_info: Box<Option<super::super::types::efs::AccessPointRootDirectoryCreationInfo>>,
    /// Path on the EFS file system to expose as the root directory to NFS clients using the access point to access the EFS file system. A path can have up to four subdirectories. If the specified path does not exist, you are required to provide `creation_info`.
    #[builder(into, default)]
    #[serde(rename = "path")]
    pub r#path: Box<Option<String>>,
}
