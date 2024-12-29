#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ProjectFileSystemLocation {
    /// The name used to access a file system created by Amazon EFS. CodeBuild creates an environment variable by appending the identifier in all capital letters to CODEBUILD\_. For example, if you specify my-efs for identifier, a new environment variable is create named CODEBUILD_MY-EFS.
    #[builder(into, default)]
    #[serde(rename = "identifier")]
    pub r#identifier: Box<Option<String>>,
    /// A string that specifies the location of the file system created by Amazon EFS. Its format is `efs-dns-name:/directory-path`.
    #[builder(into, default)]
    #[serde(rename = "location")]
    pub r#location: Box<Option<String>>,
    /// The mount options for a file system created by AWS EFS.
    #[builder(into, default)]
    #[serde(rename = "mountOptions")]
    pub r#mount_options: Box<Option<String>>,
    /// The location in the container where you mount the file system.
    #[builder(into, default)]
    #[serde(rename = "mountPoint")]
    pub r#mount_point: Box<Option<String>>,
    /// The type of the file system. The one supported type is `EFS`.
    #[builder(into, default)]
    #[serde(rename = "type")]
    pub r#type_: Box<Option<String>>,
}
