#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct PreventionJobTriggerInspectJobActionDeidentifyTransformationConfig {
    /// If this template is specified, it will serve as the default de-identify template.
    #[builder(into, default)]
    #[serde(rename = "deidentifyTemplate")]
    pub r#deidentify_template: Box<Option<String>>,
    /// If this template is specified, it will serve as the de-identify template for images.
    #[builder(into, default)]
    #[serde(rename = "imageRedactTemplate")]
    pub r#image_redact_template: Box<Option<String>>,
    /// If this template is specified, it will serve as the de-identify template for structured content such as delimited files and tables.
    #[builder(into, default)]
    #[serde(rename = "structuredDeidentifyTemplate")]
    pub r#structured_deidentify_template: Box<Option<String>>,
}
