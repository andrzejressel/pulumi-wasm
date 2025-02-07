#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct HttpRouteRuleActionFaultInjectionPolicyAbort {
    /// The HTTP status code used to abort the request.
    #[builder(into, default)]
    #[serde(rename = "httpStatus")]
    pub r#http_status: Box<Option<i32>>,
    /// The percentage of traffic which will be aborted.
    #[builder(into, default)]
    #[serde(rename = "percentage")]
    pub r#percentage: Box<Option<i32>>,
}
