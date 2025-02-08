#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct FleetRuntimeConfigurationServerProcess {
    /// Number of server processes using this configuration to run concurrently on an instance.
    #[builder(into)]
    #[serde(rename = "concurrentExecutions")]
    pub r#concurrent_executions: Box<i32>,
    /// Location of the server executable in a game build. All game builds are installed on instances at the root : for Windows instances `C:\game`, and for Linux instances `/local/game`.
    #[builder(into)]
    #[serde(rename = "launchPath")]
    pub r#launch_path: Box<String>,
    /// Optional list of parameters to pass to the server executable on launch.
    #[builder(into, default)]
    #[serde(rename = "parameters")]
    pub r#parameters: Box<Option<String>>,
}
