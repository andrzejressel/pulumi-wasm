#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ClusterStorageConfig {
    #[builder(into, default)]
    #[serde(rename = "blockStorage")]
    pub r#block_storage: Box<Option<super::super::types::eks::ClusterStorageConfigBlockStorage>>,
}
