#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct DomainNameMutualTlsAuthentication {
    /// Amazon S3 URL that specifies the truststore for mutual TLS authentication, for example, `s3://bucket-name/key-name`. The truststore can contain certificates from public or private certificate authorities. To update the truststore, upload a new version to S3, and then update your custom domain name to use the new version.
    #[builder(into)]
    #[serde(rename = "truststoreUri")]
    pub r#truststore_uri: Box<String>,
    /// Version of the S3 object that contains the truststore. To specify a version, you must have versioning enabled for the S3 bucket.
    #[builder(into, default)]
    #[serde(rename = "truststoreVersion")]
    pub r#truststore_version: Box<Option<String>>,
}
