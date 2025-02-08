#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct PreventionDeidentifyTemplateDeidentifyConfigImageTransformations {
    /// For determination of how redaction of images should occur.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "transforms")]
    pub r#transforms: Box<Vec<super::super::types::dataloss::PreventionDeidentifyTemplateDeidentifyConfigImageTransformationsTransform>>,
}
