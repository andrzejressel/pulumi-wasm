#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct PreventionDeidentifyTemplateDeidentifyConfigRecordTransformationsFieldTransformationInfoTypeTransformationsTransformationPrimitiveTransformationCharacterMaskConfig {
    /// Characters to skip when doing de-identification of a value. These will be left alone and skipped.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "charactersToIgnores")]
    pub r#characters_to_ignores: Box<Option<Vec<super::super::types::dataloss::PreventionDeidentifyTemplateDeidentifyConfigRecordTransformationsFieldTransformationInfoTypeTransformationsTransformationPrimitiveTransformationCharacterMaskConfigCharactersToIgnore>>>,
    /// Character to use to mask the sensitive values—for example, * for an alphabetic string such as a name, or 0 for a numeric string
    /// such as ZIP code or credit card number. This string must have a length of 1. If not supplied, this value defaults to * for
    /// strings, and 0 for digits.
    #[builder(into, default)]
    #[serde(rename = "maskingCharacter")]
    pub r#masking_character: Box<Option<String>>,
    /// Number of characters to mask. If not set, all matching chars will be masked. Skipped characters do not count towards this tally.
    /// If number_to_mask is negative, this denotes inverse masking. Cloud DLP masks all but a number of characters. For example, suppose you have the following values:
    #[builder(into, default)]
    #[serde(rename = "numberToMask")]
    pub r#number_to_mask: Box<Option<i32>>,
    /// Mask characters in reverse order. For example, if masking_character is 0, number_to_mask is 14, and reverse_order is `false`, then the
    /// input string `1234-5678-9012-3456` is masked as `00000000000000-3456`.
    #[builder(into, default)]
    #[serde(rename = "reverseOrder")]
    pub r#reverse_order: Box<Option<bool>>,
}
