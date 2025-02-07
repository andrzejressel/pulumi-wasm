#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct HostPoolScheduledAgentUpdates {
    /// Enables or disables scheduled updates of the AVD agent components (RDAgent, Geneva Monitoring agent, and side-by-side stack) on session hosts. If this is enabled then up to two `schedule` blocks must be defined. Default is `false`.
    /// 
    /// > **NOTE:** if `enabled` is set to `true` then at least one and a maximum of two `schedule` blocks must be provided.
    #[builder(into, default)]
    #[serde(rename = "enabled")]
    pub r#enabled: Box<Option<bool>>,
    /// A `schedule` block as defined below. A maximum of two blocks can be added.
    #[builder(into, default)]
    #[serde(rename = "schedules")]
    pub r#schedules: Box<Option<Vec<super::super::types::desktopvirtualization::HostPoolScheduledAgentUpdatesSchedule>>>,
    /// Specifies the time zone in which the agent update schedule will apply, [the possible values are defined here](https://jackstromberg.com/2017/01/list-of-time-zones-consumed-by-azure/). If `use_session_host_timezone` is enabled then it will override this setting. Default is `UTC`
    #[builder(into, default)]
    #[serde(rename = "timezone")]
    pub r#timezone: Box<Option<String>>,
    /// Specifies whether scheduled agent updates should be applied based on the timezone of the affected session host. If configured then this setting overrides `timezone`. Default is `false`.
    #[builder(into, default)]
    #[serde(rename = "useSessionHostTimezone")]
    pub r#use_session_host_timezone: Box<Option<bool>>,
}
