#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct TcpRouteRuleAction {
    /// The destination services to which traffic should be forwarded. At least one destination service is required.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "destinations")]
    pub r#destinations: Box<Option<Vec<super::super::types::networkservices::TcpRouteRuleActionDestination>>>,
    /// Specifies the idle timeout for the selected route. The idle timeout is defined as the period in which there are no bytes sent or received on either the upstream or downstream connection. If not set, the default idle timeout is 30 seconds. If set to 0s, the timeout will be disabled.
    /// A duration in seconds with up to nine fractional digits, ending with 's'. Example: "3.5s".
    #[builder(into, default)]
    #[serde(rename = "idleTimeout")]
    pub r#idle_timeout: Box<Option<String>>,
    /// If true, Router will use the destination IP and port of the original connection as the destination of the request.
    #[builder(into, default)]
    #[serde(rename = "originalDestination")]
    pub r#original_destination: Box<Option<bool>>,
}
