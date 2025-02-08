#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct SourceApiAssociationSourceApiAssociationConfig {
    /// Merge type. Valid values: `MANUAL_MERGE`, `AUTO_MERGE`
    #[builder(into)]
    #[serde(rename = "mergeType")]
    pub r#merge_type: Box<String>,
}
