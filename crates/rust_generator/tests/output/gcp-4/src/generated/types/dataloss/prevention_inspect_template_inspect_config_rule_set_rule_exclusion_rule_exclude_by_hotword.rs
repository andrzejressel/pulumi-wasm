#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct PreventionInspectTemplateInspectConfigRuleSetRuleExclusionRuleExcludeByHotword {
    /// Regular expression pattern defining what qualifies as a hotword.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "hotwordRegex")]
    pub r#hotword_regex: Box<super::super::types::dataloss::PreventionInspectTemplateInspectConfigRuleSetRuleExclusionRuleExcludeByHotwordHotwordRegex>,
    /// Proximity of the finding within which the entire hotword must reside. The total length of the window cannot
    /// exceed 1000 characters. Note that the finding itself will be included in the window, so that hotwords may be
    /// used to match substrings of the finding itself. For example, the certainty of a phone number regex
    /// `(\d{3}) \d{3}-\d{4}` could be adjusted upwards if the area code is known to be the local area code of a company
    /// office using the hotword regex `(xxx)`, where `xxx` is the area code in question.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "proximity")]
    pub r#proximity: Box<super::super::types::dataloss::PreventionInspectTemplateInspectConfigRuleSetRuleExclusionRuleExcludeByHotwordProximity>,
}
