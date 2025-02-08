#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct PreventionDeidentifyTemplateDeidentifyConfigRecordTransformations {
    /// Transform the record by applying various field transformations.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "fieldTransformations")]
    pub r#field_transformations: Box<Option<Vec<super::super::types::dataloss::PreventionDeidentifyTemplateDeidentifyConfigRecordTransformationsFieldTransformation>>>,
    /// Configuration defining which records get suppressed entirely. Records that match any suppression rule are omitted from the output.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "recordSuppressions")]
    pub r#record_suppressions: Box<Option<Vec<super::super::types::dataloss::PreventionDeidentifyTemplateDeidentifyConfigRecordTransformationsRecordSuppression>>>,
}
