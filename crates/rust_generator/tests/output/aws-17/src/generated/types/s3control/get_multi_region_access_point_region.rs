#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct GetMultiRegionAccessPointRegion {
    /// The name of the bucket.
    #[builder(into)]
    #[serde(rename = "bucket")]
    pub r#bucket: Box<String>,
    /// The AWS account ID that owns the bucket.
    #[builder(into)]
    #[serde(rename = "bucketAccountId")]
    pub r#bucket_account_id: Box<String>,
    /// The name of the region.
    #[builder(into)]
    #[serde(rename = "region")]
    pub r#region: Box<String>,
}
