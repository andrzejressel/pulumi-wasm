#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ContainerHealthcheck {
    /// Time between running the check (ms|s|m|h). Defaults to `0s`.
    #[builder(into, default)]
    #[serde(rename = "interval")]
    pub r#interval: Box<Option<String>>,
    /// Consecutive failures needed to report unhealthy. Defaults to `0`.
    #[builder(into, default)]
    #[serde(rename = "retries")]
    pub r#retries: Box<Option<i32>>,
    /// Start period for the container to initialize before counting retries towards unstable (ms|s|m|h). Defaults to `0s`.
    #[builder(into, default)]
    #[serde(rename = "startPeriod")]
    pub r#start_period: Box<Option<String>>,
    /// Command to run to check health. For example, to run `curl -f localhost/health` set the command to be `["CMD", "curl", "-f", "localhost/health"]`.
    #[builder(into)]
    #[serde(rename = "tests")]
    pub r#tests: Box<Vec<String>>,
    /// Maximum time to allow one check to run (ms|s|m|h). Defaults to `0s`.
    #[builder(into, default)]
    #[serde(rename = "timeout")]
    pub r#timeout: Box<Option<String>>,
}
