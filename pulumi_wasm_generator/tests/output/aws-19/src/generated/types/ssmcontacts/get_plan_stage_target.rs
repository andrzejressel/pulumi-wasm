#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetPlanStageTarget {
    #[builder(into)]
    #[serde(rename = "channelTargetInfos")]
    pub r#channel_target_infos: Box<Vec<super::super::types::ssmcontacts::GetPlanStageTargetChannelTargetInfo>>,
    #[builder(into)]
    #[serde(rename = "contactTargetInfos")]
    pub r#contact_target_infos: Box<Vec<super::super::types::ssmcontacts::GetPlanStageTargetContactTargetInfo>>,
}
