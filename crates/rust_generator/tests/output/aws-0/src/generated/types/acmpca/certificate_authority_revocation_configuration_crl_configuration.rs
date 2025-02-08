#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct CertificateAuthorityRevocationConfigurationCrlConfiguration {
    /// Name inserted into the certificate CRL Distribution Points extension that enables the use of an alias for the CRL distribution point. Use this value if you don't want the name of your S3 bucket to be public. Must be less than or equal to 253 characters in length.
    #[builder(into, default)]
    #[serde(rename = "customCname")]
    pub r#custom_cname: Box<Option<String>>,
    /// Boolean value that specifies whether certificate revocation lists (CRLs) are enabled. Defaults to `false`.
    #[builder(into, default)]
    #[serde(rename = "enabled")]
    pub r#enabled: Box<Option<bool>>,
    /// Number of days until a certificate expires. Must be between 1 and 5000.
    #[builder(into, default)]
    #[serde(rename = "expirationInDays")]
    pub r#expiration_in_days: Box<Option<i32>>,
    /// Name of the S3 bucket that contains the CRL. If you do not provide a value for the `custom_cname` argument, the name of your S3 bucket is placed into the CRL Distribution Points extension of the issued certificate. You must specify a bucket policy that allows ACM PCA to write the CRL to your bucket. Must be between 3 and 255 characters in length.
    #[builder(into, default)]
    #[serde(rename = "s3BucketName")]
    pub r#s_3_bucket_name: Box<Option<String>>,
    /// Determines whether the CRL will be publicly readable or privately held in the CRL Amazon S3 bucket. Defaults to `PUBLIC_READ`.
    #[builder(into, default)]
    #[serde(rename = "s3ObjectAcl")]
    pub r#s_3_object_acl: Box<Option<String>>,
}
