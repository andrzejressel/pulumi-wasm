#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ScalingPlanSchedule {
    /// A list of Days of the Week on which this schedule will be used. Possible values are `Monday`, `Tuesday`, `Wednesday`, `Thursday`, `Friday`, `Saturday`, and `Sunday`
    #[builder(into)]
    #[serde(rename = "daysOfWeeks")]
    pub r#days_of_weeks: Box<Vec<String>>,
    /// The name of the schedule.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// The load Balancing Algorithm to use during Off-Peak Hours. Possible values are `DepthFirst` and `BreadthFirst`.
    #[builder(into)]
    #[serde(rename = "offPeakLoadBalancingAlgorithm")]
    pub r#off_peak_load_balancing_algorithm: Box<String>,
    /// The time at which Off-Peak scaling will begin. This is also the end-time for the Ramp-Down period. The time must be specified in "HH:MM" format.
    #[builder(into)]
    #[serde(rename = "offPeakStartTime")]
    pub r#off_peak_start_time: Box<String>,
    /// The load Balancing Algorithm to use during Peak Hours. Possible values are `DepthFirst` and `BreadthFirst`.
    #[builder(into)]
    #[serde(rename = "peakLoadBalancingAlgorithm")]
    pub r#peak_load_balancing_algorithm: Box<String>,
    /// The time at which Peak scaling will begin. This is also the end-time for the Ramp-Up period. The time must be specified in "HH:MM" format.
    #[builder(into)]
    #[serde(rename = "peakStartTime")]
    pub r#peak_start_time: Box<String>,
    /// This is the value in percentage of used host pool capacity that will be considered to evaluate whether to turn on/off virtual machines during the ramp-down and off-peak hours. For example, if capacity threshold is specified as 60% and your total host pool capacity is 100 sessions, autoscale will turn on additional session hosts once the host pool exceeds a load of 60 sessions.
    #[builder(into)]
    #[serde(rename = "rampDownCapacityThresholdPercent")]
    pub r#ramp_down_capacity_threshold_percent: Box<i32>,
    /// Whether users will be forced to log-off session hosts once the `ramp_down_wait_time_minutes` value has been exceeded during the Ramp-Down period. Possible values are `true` and `false`.
    #[builder(into)]
    #[serde(rename = "rampDownForceLogoffUsers")]
    pub r#ramp_down_force_logoff_users: Box<bool>,
    /// The load Balancing Algorithm to use during the Ramp-Down period. Possible values are `DepthFirst` and `BreadthFirst`.
    #[builder(into)]
    #[serde(rename = "rampDownLoadBalancingAlgorithm")]
    pub r#ramp_down_load_balancing_algorithm: Box<String>,
    /// The minimum percentage of session host virtual machines that you would like to get to for ramp-down and off-peak hours. For example, if Minimum percentage of hosts is specified as 10% and total number of session hosts in your host pool is 10, autoscale will ensure a minimum of 1 session host is available to take user connections.
    #[builder(into)]
    #[serde(rename = "rampDownMinimumHostsPercent")]
    pub r#ramp_down_minimum_hosts_percent: Box<i32>,
    /// The notification message to send to users during Ramp-Down period when they are required to log-off.
    #[builder(into)]
    #[serde(rename = "rampDownNotificationMessage")]
    pub r#ramp_down_notification_message: Box<String>,
    /// The time at which Ramp-Down scaling will begin. This is also the end-time for the Ramp-Up period. The time must be specified in "HH:MM" format.
    #[builder(into)]
    #[serde(rename = "rampDownStartTime")]
    pub r#ramp_down_start_time: Box<String>,
    /// Controls Session Host shutdown behaviour during Ramp-Down period. Session Hosts can either be shutdown when all sessions on the Session Host have ended, or when there are no Active sessions left on the Session Host. Possible values are `ZeroSessions` and `ZeroActiveSessions`.
    #[builder(into)]
    #[serde(rename = "rampDownStopHostsWhen")]
    pub r#ramp_down_stop_hosts_when: Box<String>,
    /// The number of minutes during Ramp-Down period that autoscale will wait after setting the session host VMs to drain mode, notifying any currently signed in users to save their work before forcing the users to logoff. Once all user sessions on the session host VM have been logged off, Autoscale will shut down the VM.
    #[builder(into)]
    #[serde(rename = "rampDownWaitTimeMinutes")]
    pub r#ramp_down_wait_time_minutes: Box<i32>,
    /// This is the value of percentage of used host pool capacity that will be considered to evaluate whether to turn on/off virtual machines during the ramp-up and peak hours. For example, if capacity threshold is specified as `60%` and your total host pool capacity is `100` sessions, autoscale will turn on additional session hosts once the host pool exceeds a load of `60` sessions.
    #[builder(into, default)]
    #[serde(rename = "rampUpCapacityThresholdPercent")]
    pub r#ramp_up_capacity_threshold_percent: Box<Option<i32>>,
    /// The load Balancing Algorithm to use during the Ramp-Up period. Possible values are `DepthFirst` and `BreadthFirst`.
    #[builder(into)]
    #[serde(rename = "rampUpLoadBalancingAlgorithm")]
    pub r#ramp_up_load_balancing_algorithm: Box<String>,
    /// Specifies the minimum percentage of session host virtual machines to start during ramp-up for peak hours. For example, if Minimum percentage of hosts is specified as `10%` and total number of session hosts in your host pool is `10`, autoscale will ensure a minimum of `1` session host is available to take user connections.
    #[builder(into, default)]
    #[serde(rename = "rampUpMinimumHostsPercent")]
    pub r#ramp_up_minimum_hosts_percent: Box<Option<i32>>,
    /// The time at which Ramp-Up scaling will begin. This is also the end-time for the Ramp-Up period. The time must be specified in "HH:MM" format.
    #[builder(into)]
    #[serde(rename = "rampUpStartTime")]
    pub r#ramp_up_start_time: Box<String>,
}
