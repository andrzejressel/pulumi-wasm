#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetRegionNetworkEndpointGroupCloudRun {
    /// Cloud Run service is the main resource of Cloud Run.
    /// The service must be 1-63 characters long, and comply with RFC1035.
    /// Example value: "run-service".
    #[builder(into)]
    #[serde(rename = "service")]
    pub r#service: Box<String>,
    /// Cloud Run tag represents the "named-revision" to provide
    /// additional fine-grained traffic routing information.
    /// The tag must be 1-63 characters long, and comply with RFC1035.
    /// Example value: "revision-0010".
    #[builder(into)]
    #[serde(rename = "tag")]
    pub r#tag: Box<String>,
    /// A template to parse service and tag fields from a request URL.
    /// URL mask allows for routing to multiple Run services without having
    /// to create multiple network endpoint groups and backend services.
    /// 
    /// For example, request URLs "foo1.domain.com/bar1" and "foo1.domain.com/bar2"
    /// an be backed by the same Serverless Network Endpoint Group (NEG) with
    /// URL mask ".domain.com/". The URL mask will parse them to { service="bar1", tag="foo1" }
    /// and { service="bar2", tag="foo2" } respectively.
    #[builder(into)]
    #[serde(rename = "urlMask")]
    pub r#url_mask: Box<String>,
}
