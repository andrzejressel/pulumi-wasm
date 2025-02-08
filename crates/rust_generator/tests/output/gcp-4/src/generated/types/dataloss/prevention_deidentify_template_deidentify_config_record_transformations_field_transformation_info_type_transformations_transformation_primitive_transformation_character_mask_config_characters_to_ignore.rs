#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct PreventionDeidentifyTemplateDeidentifyConfigRecordTransformationsFieldTransformationInfoTypeTransformationsTransformationPrimitiveTransformationCharacterMaskConfigCharactersToIgnore {
    /// Characters to not transform when masking. Only one of this or `common_characters_to_ignore` must be specified.
    #[builder(into, default)]
    #[serde(rename = "charactersToSkip")]
    pub r#characters_to_skip: Box<Option<String>>,
    /// Common characters to not transform when masking. Useful to avoid removing punctuation. Only one of this or `characters_to_skip` must be specified.
    /// Possible values are: `NUMERIC`, `ALPHA_UPPER_CASE`, `ALPHA_LOWER_CASE`, `PUNCTUATION`, `WHITESPACE`.
    #[builder(into, default)]
    #[serde(rename = "commonCharactersToIgnore")]
    pub r#common_characters_to_ignore: Box<Option<String>>,
}
