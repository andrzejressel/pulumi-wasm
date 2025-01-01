#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct PreventionDeidentifyTemplateDeidentifyConfigRecordTransformationsFieldTransformationPrimitiveTransformationBucketingConfigBucket {
    /// Upper bound of the range, exclusive; type must match min.
    /// The `max` block must only contain one argument. See the `bucketing_config` block description for more information about choosing a data type.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "max")]
    pub r#max: Box<Option<super::super::types::dataloss::PreventionDeidentifyTemplateDeidentifyConfigRecordTransformationsFieldTransformationPrimitiveTransformationBucketingConfigBucketMax>>,
    /// Lower bound of the range, inclusive. Type should be the same as max if used.
    /// The `min` block must only contain one argument. See the `bucketing_config` block description for more information about choosing a data type.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "min")]
    pub r#min: Box<Option<super::super::types::dataloss::PreventionDeidentifyTemplateDeidentifyConfigRecordTransformationsFieldTransformationPrimitiveTransformationBucketingConfigBucketMin>>,
    /// Replacement value for this bucket.
    /// The `replacement_value` block must only contain one argument.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "replacementValue")]
    pub r#replacement_value: Box<super::super::types::dataloss::PreventionDeidentifyTemplateDeidentifyConfigRecordTransformationsFieldTransformationPrimitiveTransformationBucketingConfigBucketReplacementValue>,
}
