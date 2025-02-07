#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct TriggerTrigger {
    /// The branches that will be included in the trigger configuration. If no branches   are specified, the trigger will apply to all branches.
    #[builder(into, default)]
    #[serde(rename = "branches")]
    pub r#branches: Box<Option<Vec<String>>>,
    /// Any custom data associated with the trigger that will be included in the information sent to the target of the trigger.
    #[builder(into, default)]
    #[serde(rename = "customData")]
    pub r#custom_data: Box<Option<String>>,
    /// The ARN of the resource that is the target for a trigger. For example, the ARN of a topic in Amazon Simple Notification Service (SNS).
    #[builder(into)]
    #[serde(rename = "destinationArn")]
    pub r#destination_arn: Box<String>,
    /// The repository events that will cause the trigger to run actions in another service, such as sending a notification through Amazon Simple Notification Service (SNS). If no events are specified, the trigger will run for all repository events. Event types include: `all`, `updateReference`, `createReference`, `deleteReference`.
    #[builder(into)]
    #[serde(rename = "events")]
    pub r#events: Box<Vec<String>>,
    /// The name of the trigger.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
}
