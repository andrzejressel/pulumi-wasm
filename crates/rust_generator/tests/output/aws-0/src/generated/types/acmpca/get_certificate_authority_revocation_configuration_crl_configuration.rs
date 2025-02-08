#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetCertificateAuthorityRevocationConfigurationCrlConfiguration {
    /// Name inserted into the certificate CRL Distribution Points extension that enables the use of an alias for the CRL distribution point.
    #[builder(into)]
    #[serde(rename = "customCname")]
    pub r#custom_cname: Box<String>,
    /// Boolean value that specifies whether certificate revocation lists (CRLs) are enabled.
    #[builder(into)]
    #[serde(rename = "enabled")]
    pub r#enabled: Box<bool>,
    /// Number of days until a certificate expires.
    #[builder(into)]
    #[serde(rename = "expirationInDays")]
    pub r#expiration_in_days: Box<i32>,
    /// Name of the S3 bucket that contains the CRL.
    #[builder(into)]
    #[serde(rename = "s3BucketName")]
    pub r#s_3_bucket_name: Box<String>,
    /// Whether the CRL is publicly readable or privately held in the CRL Amazon S3 bucket.
    #[builder(into)]
    #[serde(rename = "s3ObjectAcl")]
    pub r#s_3_object_acl: Box<String>,
}
