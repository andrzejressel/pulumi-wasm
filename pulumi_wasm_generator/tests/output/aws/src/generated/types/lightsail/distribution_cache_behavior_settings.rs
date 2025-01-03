#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct DistributionCacheBehaviorSettings {
    /// The HTTP methods that are processed and forwarded to the distribution's origin.
    #[builder(into, default)]
    #[serde(rename = "allowedHttpMethods")]
    pub r#allowed_http_methods: Box<Option<String>>,
    /// The HTTP method responses that are cached by your distribution.
    #[builder(into, default)]
    #[serde(rename = "cachedHttpMethods")]
    pub r#cached_http_methods: Box<Option<String>>,
    /// The default amount of time that objects stay in the distribution's cache before the distribution forwards another request to the origin to determine whether the content has been updated.
    #[builder(into, default)]
    #[serde(rename = "defaultTtl")]
    pub r#default_ttl: Box<Option<i32>>,
    /// An object that describes the cookies that are forwarded to the origin. Your content is cached based on the cookies that are forwarded. Detailed below
    #[builder(into, default)]
    #[serde(rename = "forwardedCookies")]
    pub r#forwarded_cookies: Box<Option<super::super::types::lightsail::DistributionCacheBehaviorSettingsForwardedCookies>>,
    /// An object that describes the headers that are forwarded to the origin. Your content is cached based on the headers that are forwarded. Detailed below
    #[builder(into, default)]
    #[serde(rename = "forwardedHeaders")]
    pub r#forwarded_headers: Box<Option<super::super::types::lightsail::DistributionCacheBehaviorSettingsForwardedHeaders>>,
    /// An object that describes the query strings that are forwarded to the origin. Your content is cached based on the query strings that are forwarded. Detailed below
    #[builder(into, default)]
    #[serde(rename = "forwardedQueryStrings")]
    pub r#forwarded_query_strings: Box<Option<super::super::types::lightsail::DistributionCacheBehaviorSettingsForwardedQueryStrings>>,
    /// The maximum amount of time that objects stay in the distribution's cache before the distribution forwards another request to the origin to determine whether the object has been updated.
    #[builder(into, default)]
    #[serde(rename = "maximumTtl")]
    pub r#maximum_ttl: Box<Option<i32>>,
    /// The minimum amount of time that objects stay in the distribution's cache before the distribution forwards another request to the origin to determine whether the object has been updated.
    #[builder(into, default)]
    #[serde(rename = "minimumTtl")]
    pub r#minimum_ttl: Box<Option<i32>>,
}
