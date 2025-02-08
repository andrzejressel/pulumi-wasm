#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetTrafficManagerProfileMonitorConfigCustomHeader {
    /// Specifies the name of the Traffic Manager Profile.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// The value of custom header. Applicable for HTTP and HTTPS protocol.
    #[builder(into)]
    #[serde(rename = "value")]
    pub r#value: Box<String>,
}
