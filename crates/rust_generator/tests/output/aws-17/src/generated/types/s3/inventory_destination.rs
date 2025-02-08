#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct InventoryDestination {
    /// S3 bucket configuration where inventory results are published (documented below).
    #[builder(into)]
    #[serde(rename = "bucket")]
    pub r#bucket: Box<super::super::types::s3::InventoryDestinationBucket>,
}
