#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ListenerDefaultActionForwardTargetGroup {
    /// ID or Amazon Resource Name (ARN) of the target group.
    #[builder(into, default)]
    #[serde(rename = "targetGroupIdentifier")]
    pub r#target_group_identifier: Box<Option<String>>,
    /// Determines how requests are distributed to the target group. Only required if you specify multiple target groups for a forward action. For example, if you specify two target groups, one with a
    /// weight of 10 and the other with a weight of 20, the target group with a weight of 20 receives twice as many requests as the other target group. See [Listener rules](https://docs.aws.amazon.com/vpc-lattice/latest/ug/listeners.html#listener-rules) in the AWS documentation for additional examples. Default: `100`.
    #[builder(into, default)]
    #[serde(rename = "weight")]
    pub r#weight: Box<Option<i32>>,
}
