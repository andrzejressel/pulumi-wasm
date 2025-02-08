#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct DistributionLoggingConfig {
    /// Amazon S3 bucket to store the access logs in, for example, `myawslogbucket.s3.amazonaws.com`.
    #[builder(into)]
    #[serde(rename = "bucket")]
    pub r#bucket: Box<String>,
    /// Whether to include cookies in access logs (default: `false`).
    #[builder(into, default)]
    #[serde(rename = "includeCookies")]
    pub r#include_cookies: Box<Option<bool>>,
    /// Prefix to the access log filenames for this distribution, for example, `myprefix/`.
    #[builder(into, default)]
    #[serde(rename = "prefix")]
    pub r#prefix: Box<Option<String>>,
}
