#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ServerS3StorageOptions {
    /// Specifies whether or not performance for your Amazon S3 directories is optimized. Valid values are `DISABLED`, `ENABLED`.
    /// 
    /// By default, home directory mappings have a `TYPE` of `DIRECTORY`. If you enable this option, you would then need to explicitly set the `HomeDirectoryMapEntry` Type to `FILE` if you want a mapping to have a file target. See [Using logical directories to simplify your Transfer Family directory structures](https://docs.aws.amazon.com/transfer/latest/userguide/logical-dir-mappings.html) for details.
    #[builder(into, default)]
    #[serde(rename = "directoryListingOptimization")]
    pub r#directory_listing_optimization: Box<Option<String>>,
}
