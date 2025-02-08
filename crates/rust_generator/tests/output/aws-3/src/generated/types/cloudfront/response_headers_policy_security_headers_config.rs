#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ResponseHeadersPolicySecurityHeadersConfig {
    /// The policy directives and their values that CloudFront includes as values for the `Content-Security-Policy` HTTP response header. See Content Security Policy for more information.
    #[builder(into, default)]
    #[serde(rename = "contentSecurityPolicy")]
    pub r#content_security_policy: Box<Option<super::super::types::cloudfront::ResponseHeadersPolicySecurityHeadersConfigContentSecurityPolicy>>,
    /// Determines whether CloudFront includes the `X-Content-Type-Options` HTTP response header with its value set to `nosniff`. See Content Type Options for more information.
    #[builder(into, default)]
    #[serde(rename = "contentTypeOptions")]
    pub r#content_type_options: Box<Option<super::super::types::cloudfront::ResponseHeadersPolicySecurityHeadersConfigContentTypeOptions>>,
    /// Determines whether CloudFront includes the `X-Frame-Options` HTTP response header and the header’s value. See Frame Options for more information.
    #[builder(into, default)]
    #[serde(rename = "frameOptions")]
    pub r#frame_options: Box<Option<super::super::types::cloudfront::ResponseHeadersPolicySecurityHeadersConfigFrameOptions>>,
    /// Determines whether CloudFront includes the `Referrer-Policy` HTTP response header and the header’s value. See Referrer Policy for more information.
    #[builder(into, default)]
    #[serde(rename = "referrerPolicy")]
    pub r#referrer_policy: Box<Option<super::super::types::cloudfront::ResponseHeadersPolicySecurityHeadersConfigReferrerPolicy>>,
    /// Determines whether CloudFront includes the `Strict-Transport-Security` HTTP response header and the header’s value. See Strict Transport Security for more information.
    #[builder(into, default)]
    #[serde(rename = "strictTransportSecurity")]
    pub r#strict_transport_security: Box<Option<super::super::types::cloudfront::ResponseHeadersPolicySecurityHeadersConfigStrictTransportSecurity>>,
    /// Determine whether CloudFront includes the `X-XSS-Protection` HTTP response header and the header’s value. See XSS Protection for more information.
    #[builder(into, default)]
    #[serde(rename = "xssProtection")]
    pub r#xss_protection: Box<Option<super::super::types::cloudfront::ResponseHeadersPolicySecurityHeadersConfigXssProtection>>,
}
