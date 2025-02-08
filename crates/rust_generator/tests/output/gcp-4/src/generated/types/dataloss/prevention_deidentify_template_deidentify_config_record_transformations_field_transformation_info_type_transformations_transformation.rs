#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct PreventionDeidentifyTemplateDeidentifyConfigRecordTransformationsFieldTransformationInfoTypeTransformationsTransformation {
    /// InfoTypes to apply the transformation to. Leaving this empty will apply the transformation to apply to
    /// all findings that correspond to infoTypes that were requested in InspectConfig.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "infoTypes")]
    pub r#info_types: Box<Option<Vec<super::super::types::dataloss::PreventionDeidentifyTemplateDeidentifyConfigRecordTransformationsFieldTransformationInfoTypeTransformationsTransformationInfoType>>>,
    /// Apply the transformation to the entire field.
    /// The `primitive_transformation` block must only contain one argument, corresponding to the type of transformation.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "primitiveTransformation")]
    pub r#primitive_transformation: Box<super::super::types::dataloss::PreventionDeidentifyTemplateDeidentifyConfigRecordTransformationsFieldTransformationInfoTypeTransformationsTransformationPrimitiveTransformation>,
}
