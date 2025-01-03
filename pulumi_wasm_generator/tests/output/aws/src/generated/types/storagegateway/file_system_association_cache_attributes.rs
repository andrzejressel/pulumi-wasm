#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct FileSystemAssociationCacheAttributes {
    /// Refreshes a file share's cache by using Time To Live (TTL).
    /// TTL is the length of time since the last refresh after which access to the directory would cause the file gateway
    /// to first refresh that directory's contents from the Amazon S3 bucket. Valid Values: `0` or `300` to `2592000` seconds (5 minutes to 30 days). Defaults to `0`
    #[builder(into, default)]
    #[serde(rename = "cacheStaleTimeoutInSeconds")]
    pub r#cache_stale_timeout_in_seconds: Box<Option<i32>>,
}
