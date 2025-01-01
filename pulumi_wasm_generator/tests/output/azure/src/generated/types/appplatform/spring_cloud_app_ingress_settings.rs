#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct SpringCloudAppIngressSettings {
    /// Specifies how ingress should communicate with this app backend service. Allowed values are `GRPC` and `Default`. Defaults to `Default`.
    #[builder(into, default)]
    #[serde(rename = "backendProtocol")]
    pub r#backend_protocol: Box<Option<String>>,
    /// Specifies the ingress read time out in seconds. Defaults to `300`.
    #[builder(into, default)]
    #[serde(rename = "readTimeoutInSeconds")]
    pub r#read_timeout_in_seconds: Box<Option<i32>>,
    /// Specifies the ingress send time out in seconds. Defaults to `60`.
    #[builder(into, default)]
    #[serde(rename = "sendTimeoutInSeconds")]
    pub r#send_timeout_in_seconds: Box<Option<i32>>,
    /// Specifies the type of the affinity, set this to `Cookie` to enable session affinity. Allowed values are `Cookie` and `None`. Defaults to `None`.
    #[builder(into, default)]
    #[serde(rename = "sessionAffinity")]
    pub r#session_affinity: Box<Option<String>>,
    /// Specifies the time in seconds until the cookie expires.
    #[builder(into, default)]
    #[serde(rename = "sessionCookieMaxAge")]
    pub r#session_cookie_max_age: Box<Option<i32>>,
}
