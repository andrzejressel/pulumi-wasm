#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct PreventionDeidentifyTemplateDeidentifyConfigRecordTransformationsFieldTransformationPrimitiveTransformationBucketingConfigBucketReplacementValueDateValue {
    /// Day of a month. Must be from 1 to 31 and valid for the year and month, or 0 to specify a year by itself or a year and month where the day isn't significant.
    /// 
    /// - - -
    #[builder(into, default)]
    #[serde(rename = "day")]
    pub r#day: Box<Option<i32>>,
    /// Month of a year. Must be from 1 to 12, or 0 to specify a year without a month and day.
    #[builder(into, default)]
    #[serde(rename = "month")]
    pub r#month: Box<Option<i32>>,
    /// Year of the date. Must be from 1 to 9999, or 0 to specify a date without a year.
    #[builder(into, default)]
    #[serde(rename = "year")]
    pub r#year: Box<Option<i32>>,
}
