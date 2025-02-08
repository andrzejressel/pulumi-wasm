#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct RoutingProfileMediaConcurrency {
    /// Specifies the channels that agents can handle in the Contact Control Panel (CCP). Valid values are `VOICE`, `CHAT`, `TASK`.
    #[builder(into)]
    #[serde(rename = "channel")]
    pub r#channel: Box<String>,
    /// Specifies the number of contacts an agent can have on a channel simultaneously. Valid Range for `VOICE`: Minimum value of 1. Maximum value of 1. Valid Range for `CHAT`: Minimum value of 1. Maximum value of 10. Valid Range for `TASK`: Minimum value of 1. Maximum value of 10.
    #[builder(into)]
    #[serde(rename = "concurrency")]
    pub r#concurrency: Box<i32>,
}
