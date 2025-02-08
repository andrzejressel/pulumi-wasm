#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ListenerRuleActionForward {
    /// The target groups. Traffic matching the rule is forwarded to the specified target groups. With forward actions, you can assign a weight that controls the prioritization and selection of each target group. This means that requests are distributed to individual target groups based on their weights. For example, if two target groups have the same weight, each target group receives half of the traffic.
    /// 
    /// The default value is 1 with maximum number of 2. If only one target group is provided, there is no need to set the weight; 100% of traffic will go to that target group.
    #[builder(into)]
    #[serde(rename = "targetGroups")]
    pub r#target_groups: Box<Vec<super::super::types::vpclattice::ListenerRuleActionForwardTargetGroup>>,
}
