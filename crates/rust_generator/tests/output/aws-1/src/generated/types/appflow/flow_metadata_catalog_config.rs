#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct FlowMetadataCatalogConfig {
    #[builder(into, default)]
    #[serde(rename = "glueDataCatalog")]
    pub r#glue_data_catalog: Box<Option<super::super::types::appflow::FlowMetadataCatalogConfigGlueDataCatalog>>,
}
