#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct RegionUrlMapDefaultRouteActionFaultInjectionPolicyDelayFixedDelay {
    /// Span of time that's a fraction of a second at nanosecond resolution. Durations less than one second are
    /// represented with a 0 seconds field and a positive nanos field. Must be from 0 to 999,999,999 inclusive.
    #[builder(into, default)]
    #[serde(rename = "nanos")]
    pub r#nanos: Box<Option<i32>>,
    /// Span of time at a resolution of a second. Must be from 0 to 315,576,000,000 inclusive.
    /// Note: these bounds are computed from: 60 sec/min * 60 min/hr * 24 hr/day * 365.25 days/year * 10000 years
    #[builder(into, default)]
    #[serde(rename = "seconds")]
    pub r#seconds: Box<Option<String>>,
}
