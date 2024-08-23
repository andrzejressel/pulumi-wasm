#[derive(serde::Serialize)]
pub struct LoadBalancerSessionAffinityAttribute {
    #[serde(rename = "drainDuration")]
    pub r#drain_duration: Box<Option<i32>>,
    #[serde(rename = "headers")]
    pub r#headers: Box<Option<Vec<String>>>,
    #[serde(rename = "requireAllHeaders")]
    pub r#require_all_headers: Box<Option<bool>>,
    #[serde(rename = "samesite")]
    pub r#samesite: Box<Option<String>>,
    #[serde(rename = "secure")]
    pub r#secure: Box<Option<String>>,
    #[serde(rename = "zeroDowntimeFailover")]
    pub r#zero_downtime_failover: Box<Option<String>>,
}
