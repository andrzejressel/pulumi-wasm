#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ImageRawDisk {
    /// The format used to encode and transmit the block device, which
    /// should be TAR. This is just a container and transmission format
    /// and not a runtime format. Provided by the client when the disk
    /// image is created.
    /// Default value is `TAR`.
    /// Possible values are: `TAR`.
    #[builder(into, default)]
    #[serde(rename = "containerType")]
    pub r#container_type: Box<Option<String>>,
    /// An optional SHA1 checksum of the disk image before unpackaging.
    /// This is provided by the client when the disk image is created.
    #[builder(into, default)]
    #[serde(rename = "sha1")]
    pub r#sha_1: Box<Option<String>>,
    /// The full Google Cloud Storage URL where disk storage is stored
    /// You must provide either this property or the sourceDisk property
    /// but not both.
    #[builder(into)]
    #[serde(rename = "source")]
    pub r#source: Box<String>,
}
