#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct InstanceFileSharesNfsExportOption {
    /// Either READ_ONLY, for allowing only read requests on the exported directory,
    /// or READ_WRITE, for allowing both read and write requests. The default is READ_WRITE.
    /// Default value is `READ_WRITE`.
    /// Possible values are: `READ_ONLY`, `READ_WRITE`.
    #[builder(into, default)]
    #[serde(rename = "accessMode")]
    pub r#access_mode: Box<Option<String>>,
    /// An integer representing the anonymous group id with a default value of 65534.
    /// Anon_gid may only be set with squashMode of ROOT_SQUASH. An error will be returned
    /// if this field is specified for other squashMode settings.
    #[builder(into, default)]
    #[serde(rename = "anonGid")]
    pub r#anon_gid: Box<Option<i32>>,
    /// An integer representing the anonymous user id with a default value of 65534.
    /// Anon_uid may only be set with squashMode of ROOT_SQUASH. An error will be returned
    /// if this field is specified for other squashMode settings.
    #[builder(into, default)]
    #[serde(rename = "anonUid")]
    pub r#anon_uid: Box<Option<i32>>,
    /// List of either IPv4 addresses, or ranges in CIDR notation which may mount the file share.
    /// Overlapping IP ranges are not allowed, both within and across NfsExportOptions. An error will be returned.
    /// The limit is 64 IP ranges/addresses for each FileShareConfig among all NfsExportOptions.
    #[builder(into, default)]
    #[serde(rename = "ipRanges")]
    pub r#ip_ranges: Box<Option<Vec<String>>>,
    /// Either NO_ROOT_SQUASH, for allowing root access on the exported directory, or ROOT_SQUASH,
    /// for not allowing root access. The default is NO_ROOT_SQUASH.
    /// Default value is `NO_ROOT_SQUASH`.
    /// Possible values are: `NO_ROOT_SQUASH`, `ROOT_SQUASH`.
    #[builder(into, default)]
    #[serde(rename = "squashMode")]
    pub r#squash_mode: Box<Option<String>>,
}
