#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetHostPoolScheduledAgentUpdate {
    /// Are scheduled updates of the AVD agent components (RDAgent, Geneva Monitoring agent, and side-by-side stack) enabled on session hosts.
    #[builder(into)]
    #[serde(rename = "enabled")]
    pub r#enabled: Box<bool>,
    /// A `schedule` block as defined below.
    #[builder(into)]
    #[serde(rename = "schedules")]
    pub r#schedules: Box<Vec<super::super::types::desktopvirtualization::GetHostPoolScheduledAgentUpdateSchedule>>,
    /// The time zone in which the agent update schedule will apply.
    #[builder(into)]
    #[serde(rename = "timezone")]
    pub r#timezone: Box<String>,
    /// Specifies whether scheduled agent updates should be applied based on the timezone of the affected session host.
    #[builder(into)]
    #[serde(rename = "useSessionHostTimezone")]
    pub r#use_session_host_timezone: Box<bool>,
}
