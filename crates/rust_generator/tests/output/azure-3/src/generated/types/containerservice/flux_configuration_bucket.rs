#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct FluxConfigurationBucket {
    /// Specifies the plaintext access key used to securely access the S3 bucket.
    #[builder(into, default)]
    #[serde(rename = "accessKey")]
    pub r#access_key: Box<Option<String>>,
    /// Specifies the bucket name to sync from the url endpoint for the flux configuration.
    #[builder(into)]
    #[serde(rename = "bucketName")]
    pub r#bucket_name: Box<String>,
    /// Specifies the name of a local secret on the Kubernetes cluster to use as the authentication secret rather than the managed or user-provided configuration secrets. It must be between 1 and 63 characters. It can contain only lowercase letters, numbers, and hyphens (-). It must start and end with a lowercase letter or number.
    #[builder(into, default)]
    #[serde(rename = "localAuthReference")]
    pub r#local_auth_reference: Box<Option<String>>,
    /// Specifies the Base64-encoded secret key used to authenticate with the bucket source.
    #[builder(into, default)]
    #[serde(rename = "secretKeyBase64")]
    pub r#secret_key_base_64: Box<Option<String>>,
    /// Specifies the interval at which to re-reconcile the cluster git repository source with the remote. Defaults to `600`.
    #[builder(into, default)]
    #[serde(rename = "syncIntervalInSeconds")]
    pub r#sync_interval_in_seconds: Box<Option<i32>>,
    /// Specifies the maximum time to attempt to reconcile the cluster git repository source with the remote. Defaults to `600`.
    #[builder(into, default)]
    #[serde(rename = "timeoutInSeconds")]
    pub r#timeout_in_seconds: Box<Option<i32>>,
    /// Specify whether to communicate with a bucket using TLS is enabled. Defaults to `true`.
    #[builder(into, default)]
    #[serde(rename = "tlsEnabled")]
    pub r#tls_enabled: Box<Option<bool>>,
    /// Specifies the URL to sync for the flux configuration S3 bucket. It must start with `http://` or `https://`.
    #[builder(into)]
    #[serde(rename = "url")]
    pub r#url: Box<String>,
}
