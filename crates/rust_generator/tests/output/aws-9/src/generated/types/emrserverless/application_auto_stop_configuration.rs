#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ApplicationAutoStopConfiguration {
    /// Enables the application to automatically stop after a certain amount of time being idle. Defaults to `true`.
    #[builder(into, default)]
    #[serde(rename = "enabled")]
    pub r#enabled: Box<Option<bool>>,
    /// The amount of idle time in minutes after which your application will automatically stop. Defaults to `15` minutes.
    #[builder(into, default)]
    #[serde(rename = "idleTimeoutMinutes")]
    pub r#idle_timeout_minutes: Box<Option<i32>>,
}
