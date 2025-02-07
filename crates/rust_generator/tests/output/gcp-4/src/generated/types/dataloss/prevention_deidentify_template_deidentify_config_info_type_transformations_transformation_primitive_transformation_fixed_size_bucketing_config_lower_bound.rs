#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct PreventionDeidentifyTemplateDeidentifyConfigInfoTypeTransformationsTransformationPrimitiveTransformationFixedSizeBucketingConfigLowerBound {
    /// A float value.
    #[builder(into, default)]
    #[serde(rename = "floatValue")]
    pub r#float_value: Box<Option<f64>>,
    /// An integer value (int64 format)
    #[builder(into, default)]
    #[serde(rename = "integerValue")]
    pub r#integer_value: Box<Option<String>>,
}
