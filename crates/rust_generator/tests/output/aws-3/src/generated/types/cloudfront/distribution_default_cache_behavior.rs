#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct DistributionDefaultCacheBehavior {
    /// Controls which HTTP methods CloudFront processes and forwards to your Amazon S3 bucket or your custom origin.
    #[builder(into)]
    #[serde(rename = "allowedMethods")]
    pub r#allowed_methods: Box<Vec<String>>,
    /// Unique identifier of the cache policy that is attached to the cache behavior. If configuring the `default_cache_behavior` either `cache_policy_id` or `forwarded_values` must be set.
    #[builder(into, default)]
    #[serde(rename = "cachePolicyId")]
    pub r#cache_policy_id: Box<Option<String>>,
    /// Controls whether CloudFront caches the response to requests using the specified HTTP methods.
    #[builder(into)]
    #[serde(rename = "cachedMethods")]
    pub r#cached_methods: Box<Vec<String>>,
    /// Whether you want CloudFront to automatically compress content for web requests that include `Accept-Encoding: gzip` in the request header (default: `false`).
    #[builder(into, default)]
    #[serde(rename = "compress")]
    pub r#compress: Box<Option<bool>>,
    /// Default amount of time (in seconds) that an object is in a CloudFront cache before CloudFront forwards another request in the absence of an `Cache-Control max-age` or `Expires` header.
    #[builder(into, default)]
    #[serde(rename = "defaultTtl")]
    pub r#default_ttl: Box<Option<i32>>,
    /// Field level encryption configuration ID.
    #[builder(into, default)]
    #[serde(rename = "fieldLevelEncryptionId")]
    pub r#field_level_encryption_id: Box<Option<String>>,
    /// The forwarded values configuration that specifies how CloudFront handles query strings, cookies and headers (maximum one).
    #[builder(into, default)]
    #[serde(rename = "forwardedValues")]
    pub r#forwarded_values: Box<Option<super::super::types::cloudfront::DistributionDefaultCacheBehaviorForwardedValues>>,
    /// A config block that triggers a cloudfront function with specific actions (maximum 2).
    #[builder(into, default)]
    #[serde(rename = "functionAssociations")]
    pub r#function_associations: Box<Option<Vec<super::super::types::cloudfront::DistributionDefaultCacheBehaviorFunctionAssociation>>>,
    /// A config block that triggers a lambda function with specific actions (maximum 4).
    #[builder(into, default)]
    #[serde(rename = "lambdaFunctionAssociations")]
    pub r#lambda_function_associations: Box<Option<Vec<super::super::types::cloudfront::DistributionDefaultCacheBehaviorLambdaFunctionAssociation>>>,
    /// Maximum amount of time (in seconds) that an object is in a CloudFront cache before CloudFront forwards another request to your origin to determine whether the object has been updated. Only effective in the presence of `Cache-Control max-age`, `Cache-Control s-maxage`, and `Expires` headers.
    #[builder(into, default)]
    #[serde(rename = "maxTtl")]
    pub r#max_ttl: Box<Option<i32>>,
    /// Minimum amount of time that you want objects to stay in CloudFront caches before CloudFront queries your origin to see whether the object has been updated. Defaults to 0 seconds.
    #[builder(into, default)]
    #[serde(rename = "minTtl")]
    pub r#min_ttl: Box<Option<i32>>,
    /// Unique identifier of the origin request policy that is attached to the behavior.
    #[builder(into, default)]
    #[serde(rename = "originRequestPolicyId")]
    pub r#origin_request_policy_id: Box<Option<String>>,
    /// ARN of the real-time log configuration that is attached to this cache behavior.
    #[builder(into, default)]
    #[serde(rename = "realtimeLogConfigArn")]
    pub r#realtime_log_config_arn: Box<Option<String>>,
    /// Identifier for a response headers policy.
    #[builder(into, default)]
    #[serde(rename = "responseHeadersPolicyId")]
    pub r#response_headers_policy_id: Box<Option<String>>,
    /// Indicates whether you want to distribute media files in Microsoft Smooth Streaming format using the origin that is associated with this cache behavior.
    #[builder(into, default)]
    #[serde(rename = "smoothStreaming")]
    pub r#smooth_streaming: Box<Option<bool>>,
    /// Value of ID for the origin that you want CloudFront to route requests to when a request matches the path pattern either for a cache behavior or for the default cache behavior.
    #[builder(into)]
    #[serde(rename = "targetOriginId")]
    pub r#target_origin_id: Box<String>,
    /// List of nested attributes for active trusted key groups, if the distribution is set up to serve private content with signed URLs.
    #[builder(into, default)]
    #[serde(rename = "trustedKeyGroups")]
    pub r#trusted_key_groups: Box<Option<Vec<String>>>,
    /// List of nested attributes for active trusted signers, if the distribution is set up to serve private content with signed URLs.
    #[builder(into, default)]
    #[serde(rename = "trustedSigners")]
    pub r#trusted_signers: Box<Option<Vec<String>>>,
    /// Use this element to specify the protocol that users can use to access the files in the origin specified by TargetOriginId when a request matches the path pattern in PathPattern. One of `allow-all`, `https-only`, or `redirect-to-https`.
    #[builder(into)]
    #[serde(rename = "viewerProtocolPolicy")]
    pub r#viewer_protocol_policy: Box<String>,
}
