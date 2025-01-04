#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ListenerDefaultActionForward {
    /// Configuration block for target group stickiness for the rule. See below.
    #[builder(into, default)]
    #[serde(rename = "stickiness")]
    pub r#stickiness: Box<Option<super::super::types::lb::ListenerDefaultActionForwardStickiness>>,
    /// Set of 1-5 target group blocks. See below.
    /// 
    /// The following arguments are optional:
    #[builder(into)]
    #[serde(rename = "targetGroups")]
    pub r#target_groups: Box<Vec<super::super::types::lb::ListenerDefaultActionForwardTargetGroup>>,
}
