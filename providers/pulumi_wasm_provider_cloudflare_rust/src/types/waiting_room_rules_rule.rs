#[derive(serde::Deserialize, serde::Serialize, bon::Builder, Debug)]
#[builder(finish_fn = build_struct)]
pub struct WaitingRoomRulesRule {
    /// Action to perform in the ruleset rule. Available values: `bypass_waiting_room`.
    #[builder(into)]
    #[serde(rename = "action")]
    pub r#action: Box<String>,
    /// Brief summary of the waiting room rule and its intended use.
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "description")]
    pub r#description: Box<Option<String>>,
    /// Criteria for an HTTP request to trigger the waiting room rule action. Uses the Firewall Rules expression language based on Wireshark display filters. Refer to the [Waiting Room Rules Docs](https://developers.cloudflare.com/waiting-room/additional-options/waiting-room-rules/bypass-rules/).
    #[builder(into)]
    #[serde(rename = "expression")]
    pub r#expression: Box<String>,
    /// Unique rule identifier.
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "id")]
    pub r#id: Box<Option<String>>,
    /// Whether the rule is enabled or disabled. Available values: `enabled`, `disabled`.
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "status")]
    pub r#status: Box<Option<String>>,
    /// Version of the waiting room rule.
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "version")]
    pub r#version: Box<Option<String>>,
}
