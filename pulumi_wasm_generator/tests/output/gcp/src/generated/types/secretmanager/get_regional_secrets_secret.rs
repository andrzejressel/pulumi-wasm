#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetRegionalSecretsSecret {
    /// Custom metadata about the regional secret.
    #[builder(into)]
    #[serde(rename = "annotations")]
    pub r#annotations: Box<std::collections::HashMap<String, String>>,
    /// The time at which the regional secret was created.
    #[builder(into)]
    #[serde(rename = "createTime")]
    pub r#create_time: Box<String>,
    /// Customer Managed Encryption for the regional secret.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "customerManagedEncryptions")]
    pub r#customer_managed_encryptions: Box<Vec<super::super::types::secretmanager::GetRegionalSecretsSecretCustomerManagedEncryption>>,
    #[builder(into)]
    #[serde(rename = "effectiveAnnotations")]
    pub r#effective_annotations: Box<std::collections::HashMap<String, String>>,
    #[builder(into)]
    #[serde(rename = "effectiveLabels")]
    pub r#effective_labels: Box<std::collections::HashMap<String, String>>,
    /// Timestamp in UTC when the regional secret is scheduled to expire.
    #[builder(into)]
    #[serde(rename = "expireTime")]
    pub r#expire_time: Box<String>,
    /// The labels assigned to this regional secret.
    #[builder(into)]
    #[serde(rename = "labels")]
    pub r#labels: Box<std::collections::HashMap<String, String>>,
    /// The location of the regional secret.
    #[builder(into)]
    #[serde(rename = "location")]
    pub r#location: Box<String>,
    /// The resource name of the Pub/Sub topic that will be published to.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// The ID of the project.
    #[builder(into)]
    #[serde(rename = "project")]
    pub r#project: Box<String>,
    /// The combination of labels configured directly on the resource
    ///  and default labels configured on the provider.
    #[builder(into)]
    #[serde(rename = "pulumiLabels")]
    pub r#pulumi_labels: Box<std::collections::HashMap<String, String>>,
    /// The rotation time and period for a regional secret.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "rotations")]
    pub r#rotations: Box<Vec<super::super::types::secretmanager::GetRegionalSecretsSecretRotation>>,
    /// The unique name of the resource.
    #[builder(into)]
    #[serde(rename = "secretId")]
    pub r#secret_id: Box<String>,
    /// A list of up to 10 Pub/Sub topics to which messages are published when control plane operations are called on the regional secret or its versions.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "topics")]
    pub r#topics: Box<Vec<super::super::types::secretmanager::GetRegionalSecretsSecretTopic>>,
    /// The TTL for the regional secret. A duration in seconds with up to nine fractional digits,
    /// terminated by 's'. Example: "3.5s". Only one of 'ttl' or 'expire_time' can be provided.
    #[builder(into)]
    #[serde(rename = "ttl")]
    pub r#ttl: Box<String>,
    /// Mapping from version alias to version name.
    #[builder(into)]
    #[serde(rename = "versionAliases")]
    pub r#version_aliases: Box<std::collections::HashMap<String, String>>,
    /// The version destroy ttl for the regional secret version.
    #[builder(into)]
    #[serde(rename = "versionDestroyTtl")]
    pub r#version_destroy_ttl: Box<String>,
}
