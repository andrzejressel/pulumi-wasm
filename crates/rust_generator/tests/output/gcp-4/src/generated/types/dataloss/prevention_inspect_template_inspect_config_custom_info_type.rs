#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct PreventionInspectTemplateInspectConfigCustomInfoType {
    /// Dictionary which defines the rule.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "dictionary")]
    pub r#dictionary: Box<Option<super::super::types::dataloss::PreventionInspectTemplateInspectConfigCustomInfoTypeDictionary>>,
    /// If set to EXCLUSION_TYPE_EXCLUDE this infoType will not cause a finding to be returned. It still can be used for rules matching.
    /// Possible values are: `EXCLUSION_TYPE_EXCLUDE`.
    #[builder(into, default)]
    #[serde(rename = "exclusionType")]
    pub r#exclusion_type: Box<Option<String>>,
    /// CustomInfoType can either be a new infoType, or an extension of built-in infoType, when the name matches one of existing
    /// infoTypes and that infoType is specified in `info_types` field. Specifying the latter adds findings to the
    /// one detected by the system. If built-in info type is not specified in `info_types` list then the name is
    /// treated as a custom info type.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "infoType")]
    pub r#info_type: Box<super::super::types::dataloss::PreventionInspectTemplateInspectConfigCustomInfoTypeInfoType>,
    /// Likelihood to return for this CustomInfoType. This base value can be altered by a detection rule if the finding meets the criteria
    /// specified by the rule.
    /// Default value is `VERY_LIKELY`.
    /// Possible values are: `VERY_UNLIKELY`, `UNLIKELY`, `POSSIBLE`, `LIKELY`, `VERY_LIKELY`.
    #[builder(into, default)]
    #[serde(rename = "likelihood")]
    pub r#likelihood: Box<Option<String>>,
    /// Regular expression which defines the rule.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "regex")]
    pub r#regex: Box<Option<super::super::types::dataloss::PreventionInspectTemplateInspectConfigCustomInfoTypeRegex>>,
    /// Optional custom sensitivity for this InfoType. This only applies to data profiling.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "sensitivityScore")]
    pub r#sensitivity_score: Box<Option<super::super::types::dataloss::PreventionInspectTemplateInspectConfigCustomInfoTypeSensitivityScore>>,
    /// A reference to a StoredInfoType to use with scanning.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "storedType")]
    pub r#stored_type: Box<Option<super::super::types::dataloss::PreventionInspectTemplateInspectConfigCustomInfoTypeStoredType>>,
    /// Message for detecting output from deidentification transformations that support reversing.
    #[builder(into, default)]
    #[serde(rename = "surrogateType")]
    pub r#surrogate_type: Box<Option<super::super::types::dataloss::PreventionInspectTemplateInspectConfigCustomInfoTypeSurrogateType>>,
}
