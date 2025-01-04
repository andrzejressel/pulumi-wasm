#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct SnapshotImportDiskContainerUserBucket {
    /// The name of the Amazon S3 bucket where the disk image is located.
    #[builder(into)]
    #[serde(rename = "s3Bucket")]
    pub r#s_3_bucket: Box<String>,
    /// The file name of the disk image.
    #[builder(into)]
    #[serde(rename = "s3Key")]
    pub r#s_3_key: Box<String>,
}
