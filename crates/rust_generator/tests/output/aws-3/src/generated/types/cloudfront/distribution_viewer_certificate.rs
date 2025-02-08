#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct DistributionViewerCertificate {
    /// ARN of the [AWS Certificate Manager](https://aws.amazon.com/certificate-manager/) certificate that you wish to use with this distribution. Specify this, `cloudfront_default_certificate`, or `iam_certificate_id`.  The ACM certificate must be in  US-EAST-1.
    #[builder(into, default)]
    #[serde(rename = "acmCertificateArn")]
    pub r#acm_certificate_arn: Box<Option<String>>,
    /// `true` if you want viewers to use HTTPS to request your objects and you're using the CloudFront domain name for your distribution. Specify this, `acm_certificate_arn`, or `iam_certificate_id`.
    #[builder(into, default)]
    #[serde(rename = "cloudfrontDefaultCertificate")]
    pub r#cloudfront_default_certificate: Box<Option<bool>>,
    /// IAM certificate identifier of the custom viewer certificate for this distribution if you are using a custom domain. Specify this, `acm_certificate_arn`, or `cloudfront_default_certificate`.
    #[builder(into, default)]
    #[serde(rename = "iamCertificateId")]
    pub r#iam_certificate_id: Box<Option<String>>,
    /// Minimum version of the SSL protocol that you want CloudFront to use for HTTPS connections. Can only be set if `cloudfront_default_certificate = false`. See all possible values in [this](https://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/secure-connections-supported-viewer-protocols-ciphers.html) table under "Security policy." Some examples include: `TLSv1.2_2019` and `TLSv1.2_2021`. Default: `TLSv1`. **NOTE**: If you are using a custom certificate (specified with `acm_certificate_arn` or `iam_certificate_id`), and have specified `sni-only` in `ssl_support_method`, `TLSv1` or later must be specified. If you have specified `vip` in `ssl_support_method`, only `SSLv3` or `TLSv1` can be specified. If you have specified `cloudfront_default_certificate`, `TLSv1` must be specified.
    #[builder(into, default)]
    #[serde(rename = "minimumProtocolVersion")]
    pub r#minimum_protocol_version: Box<Option<String>>,
    /// How you want CloudFront to serve HTTPS requests. One of `vip`, `sni-only`, or `static-ip`. Required if you specify `acm_certificate_arn` or `iam_certificate_id`. **NOTE:** `vip` causes CloudFront to use a dedicated IP address and may incur extra charges.
    #[builder(into, default)]
    #[serde(rename = "sslSupportMethod")]
    pub r#ssl_support_method: Box<Option<String>>,
}
