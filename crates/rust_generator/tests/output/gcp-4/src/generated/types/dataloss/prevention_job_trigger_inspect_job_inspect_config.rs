#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct PreventionJobTriggerInspectJobInspectConfig {
    /// Custom info types to be used. See https://cloud.google.com/dlp/docs/creating-custom-infotypes to learn more.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "customInfoTypes")]
    pub r#custom_info_types: Box<Option<Vec<super::super::types::dataloss::PreventionJobTriggerInspectJobInspectConfigCustomInfoType>>>,
    /// When true, excludes type information of the findings.
    #[builder(into, default)]
    #[serde(rename = "excludeInfoTypes")]
    pub r#exclude_info_types: Box<Option<bool>>,
    /// When true, a contextual quote from the data that triggered a finding is included in the response.
    #[builder(into, default)]
    #[serde(rename = "includeQuote")]
    pub r#include_quote: Box<Option<bool>>,
    /// Restricts what infoTypes to look for. The values must correspond to InfoType values returned by infoTypes.list
    /// or listed at https://cloud.google.com/dlp/docs/infotypes-reference.
    /// When no InfoTypes or CustomInfoTypes are specified in a request, the system may automatically choose what detectors to run.
    /// By default this may be all types, but may change over time as detectors are updated.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "infoTypes")]
    pub r#info_types: Box<Option<Vec<super::super::types::dataloss::PreventionJobTriggerInspectJobInspectConfigInfoType>>>,
    /// Configuration to control the number of findings returned.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "limits")]
    pub r#limits: Box<Option<super::super::types::dataloss::PreventionJobTriggerInspectJobInspectConfigLimits>>,
    /// Only returns findings equal or above this threshold. See https://cloud.google.com/dlp/docs/likelihood for more info
    /// Default value is `POSSIBLE`.
    /// Possible values are: `VERY_UNLIKELY`, `UNLIKELY`, `POSSIBLE`, `LIKELY`, `VERY_LIKELY`.
    #[builder(into, default)]
    #[serde(rename = "minLikelihood")]
    pub r#min_likelihood: Box<Option<String>>,
    /// Set of rules to apply to the findings for this InspectConfig. Exclusion rules, contained in the set are executed in the end,
    /// other rules are executed in the order they are specified for each info type.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "ruleSets")]
    pub r#rule_sets: Box<Option<Vec<super::super::types::dataloss::PreventionJobTriggerInspectJobInspectConfigRuleSet>>>,
}
