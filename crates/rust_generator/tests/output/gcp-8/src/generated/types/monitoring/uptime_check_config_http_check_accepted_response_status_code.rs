#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct UptimeCheckConfigHttpCheckAcceptedResponseStatusCode {
    /// A class of status codes to accept.
    /// Possible values are: `STATUS_CLASS_1XX`, `STATUS_CLASS_2XX`, `STATUS_CLASS_3XX`, `STATUS_CLASS_4XX`, `STATUS_CLASS_5XX`, `STATUS_CLASS_ANY`.
    #[builder(into, default)]
    #[serde(rename = "statusClass")]
    pub r#status_class: Box<Option<String>>,
    /// A status code to accept.
    #[builder(into, default)]
    #[serde(rename = "statusValue")]
    pub r#status_value: Box<Option<i32>>,
}
