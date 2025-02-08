#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct CertificateCertificate {
    /// The base64-encoded certificate contents.
    #[builder(into)]
    #[serde(rename = "contents")]
    pub r#contents: Box<String>,
    /// The password associated with the certificate.
    /// 
    /// > **NOTE:** A PEM certificate is already base64 encoded. To successfully import, the `contents` property should include a PEM encoded X509 certificate and a private_key in pkcs8 format. There should only be linux style `\n` line endings and the whole block should have the PEM begin/end blocks around the certificate data and the private key data.
    /// 
    /// To convert a private key to pkcs8 format with openssl use:
    /// ```shell
    /// openssl pkcs8 -topk8 -nocrypt -in private_key.pem > private_key_pk8.pem
    /// ```
    /// 
    /// The PEM content should look something like:
    /// ```text
    /// -----BEGIN CERTIFICATE-----
    /// aGVsbG8KaGVsbG8KaGVsbG8KaGVsbG8KaGVsbG8KaGVsbG8KaGVsbG8KaGVsbG8K
    /// :
    /// aGVsbG8KaGVsbG8KaGVsbG8KaGVsbG8KaGVsbG8KaGVsbG8KaGVsbG8KaGVsbG8K
    /// -----END CERTIFICATE-----
    /// -----BEGIN PRIVATE KEY-----
    /// d29ybGQKd29ybGQKd29ybGQKd29ybGQKd29ybGQKd29ybGQKd29ybGQKd29ybGQK
    /// :
    /// d29ybGQKd29ybGQKd29ybGQKd29ybGQKd29ybGQKd29ybGQKd29ybGQKd29ybGQK
    /// -----END PRIVATE KEY-----
    /// ```
    #[builder(into, default)]
    #[serde(rename = "password")]
    pub r#password: Box<Option<String>>,
}
