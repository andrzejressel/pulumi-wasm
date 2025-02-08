#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct CrawlerLineageConfiguration {
    /// Specifies whether data lineage is enabled for the crawler. Valid values are: `ENABLE` and `DISABLE`. Default value is `DISABLE`.
    #[builder(into, default)]
    #[serde(rename = "crawlerLineageSettings")]
    pub r#crawler_lineage_settings: Box<Option<String>>,
}
