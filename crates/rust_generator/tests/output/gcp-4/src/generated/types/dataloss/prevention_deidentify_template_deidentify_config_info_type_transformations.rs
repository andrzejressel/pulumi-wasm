#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct PreventionDeidentifyTemplateDeidentifyConfigInfoTypeTransformations {
    /// Transformation for each infoType. Cannot specify more than one for a given infoType.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "transformations")]
    pub r#transformations: Box<Vec<super::super::types::dataloss::PreventionDeidentifyTemplateDeidentifyConfigInfoTypeTransformationsTransformation>>,
}
