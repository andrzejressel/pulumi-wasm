#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct PreventionDeidentifyTemplateDeidentifyConfig {
    /// Treat the dataset as an image and redact.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "imageTransformations")]
    pub r#image_transformations: Box<Option<super::super::types::dataloss::PreventionDeidentifyTemplateDeidentifyConfigImageTransformations>>,
    /// Treat the dataset as free-form text and apply the same free text transformation everywhere
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "infoTypeTransformations")]
    pub r#info_type_transformations: Box<Option<super::super::types::dataloss::PreventionDeidentifyTemplateDeidentifyConfigInfoTypeTransformations>>,
    /// Treat the dataset as structured. Transformations can be applied to specific locations within structured datasets, such as transforming a column within a table.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "recordTransformations")]
    pub r#record_transformations: Box<Option<super::super::types::dataloss::PreventionDeidentifyTemplateDeidentifyConfigRecordTransformations>>,
}
