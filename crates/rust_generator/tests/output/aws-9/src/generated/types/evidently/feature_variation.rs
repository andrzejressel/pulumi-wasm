#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct FeatureVariation {
    /// The name of the variation. Minimum length of `1`. Maximum length of `127`.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// A block that specifies the value assigned to this variation. Detailed below
    #[builder(into)]
    #[serde(rename = "value")]
    pub r#value: Box<super::super::types::evidently::FeatureVariationValue>,
}
