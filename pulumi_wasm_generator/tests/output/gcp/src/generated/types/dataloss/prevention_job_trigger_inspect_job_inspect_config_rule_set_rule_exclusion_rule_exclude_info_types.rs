#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct PreventionJobTriggerInspectJobInspectConfigRuleSetRuleExclusionRuleExcludeInfoTypes {
    /// If a finding is matched by any of the infoType detectors listed here, the finding will be excluded from the scan results.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "infoTypes")]
    pub r#info_types: Box<Vec<super::super::types::dataloss::PreventionJobTriggerInspectJobInspectConfigRuleSetRuleExclusionRuleExcludeInfoTypesInfoType>>,
}
