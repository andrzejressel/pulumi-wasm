#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct InstanceTemplateServiceAccount {
    /// The service account e-mail address. If not given, the
    /// default Google Compute Engine service account is used.
    #[builder(into, default)]
    #[serde(rename = "email")]
    pub r#email: Box<Option<String>>,
    /// A list of service scopes. Both OAuth2 URLs and gcloud
    /// short names are supported. To allow full access to all Cloud APIs, use the
    /// `cloud-platform` scope. See a complete list of scopes [here](https://cloud.google.com/sdk/gcloud/reference/alpha/compute/instances/set-scopes#--scopes).
    /// 
    /// The [service accounts documentation](https://cloud.google.com/compute/docs/access/service-accounts#accesscopesiam)
    /// explains that access scopes are the legacy method of specifying permissions for your instance.
    /// To follow best practices you should create a dedicated service account with the minimum permissions the VM requires.
    /// To use a dedicated service account this field should be configured as a list containing the `cloud-platform` scope.
    /// See [Authenticate workloads using service accounts best practices](https://cloud.google.com/compute/docs/access/create-enable-service-accounts-for-instances#best_practices)
    /// and [Best practices for using service accounts](https://cloud.google.com/iam/docs/best-practices-service-accounts#single-purpose).
    #[builder(into)]
    #[serde(rename = "scopes")]
    pub r#scopes: Box<Vec<String>>,
}
