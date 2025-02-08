#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct AssetTypeFormsInput {
    #[builder(into)]
    #[serde(rename = "mapBlockKey")]
    pub r#map_block_key: Box<String>,
    #[builder(into, default)]
    #[serde(rename = "required")]
    pub r#required: Box<Option<bool>>,
    #[builder(into)]
    #[serde(rename = "typeIdentifier")]
    pub r#type_identifier: Box<String>,
    #[builder(into)]
    #[serde(rename = "typeRevision")]
    pub r#type_revision: Box<String>,
}
