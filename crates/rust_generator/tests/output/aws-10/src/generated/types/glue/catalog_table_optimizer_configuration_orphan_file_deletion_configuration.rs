#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct CatalogTableOptimizerConfigurationOrphanFileDeletionConfiguration {
    /// The configuration for an Iceberg orphan file deletion optimizer.
    #[builder(into, default)]
    #[serde(rename = "icebergConfiguration")]
    pub r#iceberg_configuration: Box<Option<super::super::types::glue::CatalogTableOptimizerConfigurationOrphanFileDeletionConfigurationIcebergConfiguration>>,
}
