#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct OsPolicyAssignmentOsPolicyResourceGroupResourceFile {
    /// A a file with this content. The size of the content
    /// is limited to 1024 characters.
    #[builder(into, default)]
    #[serde(rename = "content")]
    pub r#content: Box<Option<String>>,
    /// A remote or local source. Structure is
    /// documented below.
    #[builder(into, default)]
    #[serde(rename = "file")]
    pub r#file: Box<Option<super::super::types::osconfig::OsPolicyAssignmentOsPolicyResourceGroupResourceFileFile>>,
    /// The absolute path of the file within the VM.
    #[builder(into)]
    #[serde(rename = "path")]
    pub r#path: Box<String>,
    /// Consists of three octal digits which represent, in
    /// order, the permissions of the owner, group, and other users for the file
    /// (similarly to the numeric mode used in the linux chmod utility). Each digit
    /// represents a three bit number with the 4 bit corresponding to the read
    /// permissions, the 2 bit corresponds to the write bit, and the one bit
    /// corresponds to the execute permission. Default behavior is 755. Below are
    /// some examples of permissions and their associated values: read, write, and
    /// execute: 7 read and execute: 5 read and write: 6 read only: 4
    #[builder(into, default)]
    #[serde(rename = "permissions")]
    pub r#permissions: Box<Option<String>>,
    /// Desired state of the file. Possible values are:
    /// `DESIRED_STATE_UNSPECIFIED`, `PRESENT`, `ABSENT`, `CONTENTS_MATCH`.
    #[builder(into)]
    #[serde(rename = "state")]
    pub r#state: Box<String>,
}
