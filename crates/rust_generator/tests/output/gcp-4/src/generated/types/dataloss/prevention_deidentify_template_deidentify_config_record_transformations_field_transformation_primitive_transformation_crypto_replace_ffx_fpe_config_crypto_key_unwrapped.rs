#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct PreventionDeidentifyTemplateDeidentifyConfigRecordTransformationsFieldTransformationPrimitiveTransformationCryptoReplaceFfxFpeConfigCryptoKeyUnwrapped {
    /// A 128/192/256 bit key.
    /// A base64-encoded string.
    /// **Note**: This property is sensitive and will not be displayed in the plan.
    #[builder(into)]
    #[serde(rename = "key")]
    pub r#key: Box<String>,
}
