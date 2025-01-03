#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetRegionNetworkEndpointGroupAppEngine {
    /// Optional serving service.
    /// The service name must be 1-63 characters long, and comply with RFC1035.
    /// Example value: "default", "my-service".
    #[builder(into)]
    #[serde(rename = "service")]
    pub r#service: Box<String>,
    /// A template to parse service and version fields from a request URL.
    /// URL mask allows for routing to multiple App Engine services without
    /// having to create multiple Network Endpoint Groups and backend services.
    /// 
    /// For example, the request URLs "foo1-dot-appname.appspot.com/v1" and
    /// "foo1-dot-appname.appspot.com/v2" can be backed by the same Serverless NEG with
    /// URL mask "-dot-appname.appspot.com/". The URL mask will parse
    /// them to { service = "foo1", version = "v1" } and { service = "foo1", version = "v2" } respectively.
    #[builder(into)]
    #[serde(rename = "urlMask")]
    pub r#url_mask: Box<String>,
    /// Optional serving version.
    /// The version must be 1-63 characters long, and comply with RFC1035.
    /// Example value: "v1", "v2".
    #[builder(into)]
    #[serde(rename = "version")]
    pub r#version: Box<String>,
}
