#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetServiceCondition {
    /// A reason for the execution condition.
    #[builder(into)]
    #[serde(rename = "executionReason")]
    pub r#execution_reason: Box<String>,
    /// Last time the condition transitioned from one status to another.
    /// 
    /// A timestamp in RFC3339 UTC "Zulu" format, with nanosecond resolution and up to nine fractional digits. Examples: "2014-10-02T15:01:23Z" and "2014-10-02T15:01:23.045123456Z".
    #[builder(into)]
    #[serde(rename = "lastTransitionTime")]
    pub r#last_transition_time: Box<String>,
    /// Human readable message indicating details about the current status.
    #[builder(into)]
    #[serde(rename = "message")]
    pub r#message: Box<String>,
    /// A common (service-level) reason for this condition.
    #[builder(into)]
    #[serde(rename = "reason")]
    pub r#reason: Box<String>,
    /// A reason for the revision condition.
    #[builder(into)]
    #[serde(rename = "revisionReason")]
    pub r#revision_reason: Box<String>,
    /// How to interpret failures of this condition, one of Error, Warning, Info
    #[builder(into)]
    #[serde(rename = "severity")]
    pub r#severity: Box<String>,
    /// State of the condition.
    #[builder(into)]
    #[serde(rename = "state")]
    pub r#state: Box<String>,
    /// type is used to communicate the status of the reconciliation process. See also: https://github.com/knative/serving/blob/main/docs/spec/errors.md#error-conditions-and-reporting Types common to all resources include: * "Ready": True when the Resource is ready.
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: Box<String>,
}
