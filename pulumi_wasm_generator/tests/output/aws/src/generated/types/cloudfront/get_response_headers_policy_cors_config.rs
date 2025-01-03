#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetResponseHeadersPolicyCorsConfig {
    /// A Boolean value that CloudFront uses as the value for the Access-Control-Allow-Credentials HTTP response header.
    #[builder(into)]
    #[serde(rename = "accessControlAllowCredentials")]
    pub r#access_control_allow_credentials: Box<bool>,
    /// Object that contains an attribute `items` that contains a list of HTTP header names that CloudFront includes as values for the Access-Control-Allow-Headers HTTP response header.
    #[builder(into)]
    #[serde(rename = "accessControlAllowHeaders")]
    pub r#access_control_allow_headers: Box<Vec<super::super::types::cloudfront::GetResponseHeadersPolicyCorsConfigAccessControlAllowHeader>>,
    /// Object that contains an attribute `items` that contains a list of HTTP methods that CloudFront includes as values for the Access-Control-Allow-Methods HTTP response header. Valid values: `GET` | `POST` | `OPTIONS` | `PUT` | `DELETE` | `HEAD` | `ALL`
    #[builder(into)]
    #[serde(rename = "accessControlAllowMethods")]
    pub r#access_control_allow_methods: Box<Vec<super::super::types::cloudfront::GetResponseHeadersPolicyCorsConfigAccessControlAllowMethod>>,
    /// Object that contains an attribute `items` that contains a list of origins that CloudFront can use as the value for the Access-Control-Allow-Origin HTTP response header.
    #[builder(into)]
    #[serde(rename = "accessControlAllowOrigins")]
    pub r#access_control_allow_origins: Box<Vec<super::super::types::cloudfront::GetResponseHeadersPolicyCorsConfigAccessControlAllowOrigin>>,
    /// Object that contains an attribute `items` that contains a list of HTTP headers that CloudFront includes as values for the Access-Control-Expose-Headers HTTP response header.
    #[builder(into)]
    #[serde(rename = "accessControlExposeHeaders")]
    pub r#access_control_expose_headers: Box<Vec<super::super::types::cloudfront::GetResponseHeadersPolicyCorsConfigAccessControlExposeHeader>>,
    /// A number that CloudFront uses as the value for the max-age directive in the Strict-Transport-Security HTTP response header.
    #[builder(into)]
    #[serde(rename = "accessControlMaxAgeSec")]
    pub r#access_control_max_age_sec: Box<i32>,
    #[builder(into)]
    #[serde(rename = "originOverride")]
    pub r#origin_override: Box<bool>,
}
