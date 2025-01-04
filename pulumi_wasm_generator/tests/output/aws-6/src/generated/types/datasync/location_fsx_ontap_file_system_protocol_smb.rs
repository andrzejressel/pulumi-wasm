#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct LocationFsxOntapFileSystemProtocolSmb {
    /// Fully qualified domain name of the Microsoft Active Directory (AD) that your storage virtual machine belongs to.
    #[builder(into, default)]
    #[serde(rename = "domain")]
    pub r#domain: Box<Option<String>>,
    /// Mount options that are available for DataSync to access an SMB location. See SMB Mount Options below.
    #[builder(into)]
    #[serde(rename = "mountOptions")]
    pub r#mount_options: Box<super::super::types::datasync::LocationFsxOntapFileSystemProtocolSmbMountOptions>,
    /// Password of a user who has permission to access your SVM.
    #[builder(into)]
    #[serde(rename = "password")]
    pub r#password: Box<String>,
    /// Username that can mount the location and access the files, folders, and metadata that you need in the SVM.
    #[builder(into)]
    #[serde(rename = "user")]
    pub r#user: Box<String>,
}
