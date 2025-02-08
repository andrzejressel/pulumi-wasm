#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct GetNetworkServicePccRuleServiceDataFlowTemplate {
    /// The direction of this flow. Possible values are `Uplink`, `Downlink` and `Bidirectional`.
    #[builder(into)]
    #[serde(rename = "direction")]
    pub r#direction: Box<String>,
    /// Specifies the name which should be used for this Mobile Network Service.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// The port(s) to which UEs will connect for this flow. You can specify zero or more ports or port ranges.
    #[builder(into)]
    #[serde(rename = "ports")]
    pub r#ports: Box<Vec<String>>,
    /// A list of the allowed protocol(s) for this flow.
    #[builder(into)]
    #[serde(rename = "protocols")]
    pub r#protocols: Box<Vec<String>>,
    /// The remote IP address(es) to which UEs will connect for this flow.
    #[builder(into)]
    #[serde(rename = "remoteIpLists")]
    pub r#remote_ip_lists: Box<Vec<String>>,
}
