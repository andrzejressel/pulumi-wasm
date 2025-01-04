#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct FleetRuntimeConfiguration {
    /// Maximum amount of time (in seconds) that a game session can remain in status `ACTIVATING`.
    #[builder(into, default)]
    #[serde(rename = "gameSessionActivationTimeoutSeconds")]
    pub r#game_session_activation_timeout_seconds: Box<Option<i32>>,
    /// Maximum number of game sessions with status `ACTIVATING` to allow on an instance simultaneously.
    #[builder(into, default)]
    #[serde(rename = "maxConcurrentGameSessionActivations")]
    pub r#max_concurrent_game_session_activations: Box<Option<i32>>,
    /// Collection of server process configurations that describe which server processes to run on each instance in a fleet. See below.
    #[builder(into, default)]
    #[serde(rename = "serverProcesses")]
    pub r#server_processes: Box<Option<Vec<super::super::types::gamelift::FleetRuntimeConfigurationServerProcess>>>,
}
