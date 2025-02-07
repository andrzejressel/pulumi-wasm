#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetResponseHeadersPolicySecurityHeadersConfig {
    /// The policy directives and their values that CloudFront includes as values for the Content-Security-Policy HTTP response header.
    #[builder(into)]
    #[serde(rename = "contentSecurityPolicies")]
    pub r#content_security_policies: Box<Vec<super::super::types::cloudfront::GetResponseHeadersPolicySecurityHeadersConfigContentSecurityPolicy>>,
    /// A setting that determines whether CloudFront includes the X-Content-Type-Options HTTP response header with its value set to nosniff. See Content Type Options for more information.
    #[builder(into)]
    #[serde(rename = "contentTypeOptions")]
    pub r#content_type_options: Box<Vec<super::super::types::cloudfront::GetResponseHeadersPolicySecurityHeadersConfigContentTypeOption>>,
    /// Setting that determines whether CloudFront includes the X-Frame-Options HTTP response header and the header’s value. See Frame Options for more information.
    #[builder(into)]
    #[serde(rename = "frameOptions")]
    pub r#frame_options: Box<Vec<super::super::types::cloudfront::GetResponseHeadersPolicySecurityHeadersConfigFrameOption>>,
    /// Value of the Referrer-Policy HTTP response header. Valid Values: `no-referrer` | `no-referrer-when-downgrade` | `origin` | `origin-when-cross-origin` | `same-origin` | `strict-origin` | `strict-origin-when-cross-origin` | `unsafe-url`
    #[builder(into)]
    #[serde(rename = "referrerPolicies")]
    pub r#referrer_policies: Box<Vec<super::super::types::cloudfront::GetResponseHeadersPolicySecurityHeadersConfigReferrerPolicy>>,
    /// Settings that determine whether CloudFront includes the Strict-Transport-Security HTTP response header and the header’s value. See Strict Transport Security for more information.
    #[builder(into)]
    #[serde(rename = "strictTransportSecurities")]
    pub r#strict_transport_securities: Box<Vec<super::super::types::cloudfront::GetResponseHeadersPolicySecurityHeadersConfigStrictTransportSecurity>>,
    /// Settings that determine whether CloudFront includes the X-XSS-Protection HTTP response header and the header’s value. See XSS Protection for more information.
    #[builder(into)]
    #[serde(rename = "xssProtections")]
    pub r#xss_protections: Box<Vec<super::super::types::cloudfront::GetResponseHeadersPolicySecurityHeadersConfigXssProtection>>,
}
