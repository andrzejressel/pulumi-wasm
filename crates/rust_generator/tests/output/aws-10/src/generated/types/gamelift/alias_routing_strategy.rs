#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct AliasRoutingStrategy {
    /// ID of the GameLift Fleet to point the alias to.
    #[builder(into, default)]
    #[serde(rename = "fleetId")]
    pub r#fleet_id: Box<Option<String>>,
    /// Message text to be used with the `TERMINAL` routing strategy.
    #[builder(into, default)]
    #[serde(rename = "message")]
    pub r#message: Box<Option<String>>,
    /// Type of routing strategyE.g., `SIMPLE` or `TERMINAL`
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: Box<String>,
}
