#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct NfsFileShareNfsFileShareDefaults {
    /// The Unix directory mode in the string form "nnnn". Defaults to `"0777"`.
    #[builder(into, default)]
    #[serde(rename = "directoryMode")]
    pub r#directory_mode: Box<Option<String>>,
    /// The Unix file mode in the string form "nnnn". Defaults to `"0666"`.
    #[builder(into, default)]
    #[serde(rename = "fileMode")]
    pub r#file_mode: Box<Option<String>>,
    /// The default group ID for the file share (unless the files have another group ID specified). Defaults to `65534` (`nfsnobody`). Valid values: `0` through `4294967294`.
    #[builder(into, default)]
    #[serde(rename = "groupId")]
    pub r#group_id: Box<Option<String>>,
    /// The default owner ID for the file share (unless the files have another owner ID specified). Defaults to `65534` (`nfsnobody`). Valid values: `0` through `4294967294`.
    #[builder(into, default)]
    #[serde(rename = "ownerId")]
    pub r#owner_id: Box<Option<String>>,
}
