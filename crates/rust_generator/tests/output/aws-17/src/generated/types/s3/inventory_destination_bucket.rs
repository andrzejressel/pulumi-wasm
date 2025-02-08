#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct InventoryDestinationBucket {
    /// ID of the account that owns the destination bucket. Recommended to be set to prevent problems if the destination bucket ownership changes.
    #[builder(into, default)]
    #[serde(rename = "accountId")]
    pub r#account_id: Box<Option<String>>,
    /// Amazon S3 bucket ARN of the destination.
    #[builder(into)]
    #[serde(rename = "bucketArn")]
    pub r#bucket_arn: Box<String>,
    /// Contains the type of server-side encryption to use to encrypt the inventory (documented below).
    #[builder(into, default)]
    #[serde(rename = "encryption")]
    pub r#encryption: Box<Option<super::super::types::s3::InventoryDestinationBucketEncryption>>,
    /// Specifies the output format of the inventory results. Can be `CSV`, [`ORC`](https://orc.apache.org/) or [`Parquet`](https://parquet.apache.org/).
    #[builder(into)]
    #[serde(rename = "format")]
    pub r#format: Box<String>,
    /// Prefix that is prepended to all inventory results.
    #[builder(into, default)]
    #[serde(rename = "prefix")]
    pub r#prefix: Box<Option<String>>,
}
