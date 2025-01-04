#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct LbRouteExtensionExtensionChainExtension {
    /// The :authority header in the gRPC request sent from Envoy to the extension service.
    #[builder(into, default)]
    #[serde(rename = "authority")]
    pub r#authority: Box<Option<String>>,
    /// Determines how the proxy behaves if the call to the extension fails or times out.
    /// When set to TRUE, request or response processing continues without error.
    /// Any subsequent extensions in the extension chain are also executed.
    /// When set to FALSE: * If response headers have not been delivered to the downstream client,
    /// a generic 500 error is returned to the client. The error response can be tailored by
    /// configuring a custom error response in the load balancer.
    #[builder(into, default)]
    #[serde(rename = "failOpen")]
    pub r#fail_open: Box<Option<bool>>,
    /// List of the HTTP headers to forward to the extension (from the client or backend).
    /// If omitted, all headers are sent. Each element is a string indicating the header name.
    /// 
    /// - - -
    #[builder(into, default)]
    #[serde(rename = "forwardHeaders")]
    pub r#forward_headers: Box<Option<Vec<String>>>,
    /// The name for this extension. The name is logged as part of the HTTP request logs.
    /// The name must conform with RFC-1034, is restricted to lower-cased letters, numbers and hyphens,
    /// and can have a maximum length of 63 characters. Additionally, the first character must be a letter
    /// and the last a letter or a number.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// The reference to the service that runs the extension. Must be a reference to a backend service
    #[builder(into)]
    #[serde(rename = "service")]
    pub r#service: Box<String>,
    /// Specifies the timeout for each individual message on the stream. The timeout must be between 10-1000 milliseconds.
    /// A duration in seconds with up to nine fractional digits, ending with 's'. Example: "3.5s".
    #[builder(into, default)]
    #[serde(rename = "timeout")]
    pub r#timeout: Box<Option<String>>,
}
