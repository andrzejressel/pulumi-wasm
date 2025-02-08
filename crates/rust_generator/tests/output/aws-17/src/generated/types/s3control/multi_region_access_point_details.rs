#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct MultiRegionAccessPointDetails {
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    #[builder(into, default)]
    #[serde(rename = "publicAccessBlock")]
    pub r#public_access_block: Box<Option<super::super::types::s3control::MultiRegionAccessPointDetailsPublicAccessBlock>>,
    #[builder(into)]
    #[serde(rename = "regions")]
    pub r#regions: Box<Vec<super::super::types::s3control::MultiRegionAccessPointDetailsRegion>>,
}
