#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct VMwareClusterVcenter {
    /// (Output)
    /// The vCenter IP address.
    #[builder(into, default)]
    #[serde(rename = "address")]
    pub r#address: Box<Option<String>>,
    /// Contains the vCenter CA certificate public key for SSL verification.
    #[builder(into, default)]
    #[serde(rename = "caCertData")]
    pub r#ca_cert_data: Box<Option<String>>,
    /// The name of the vCenter cluster for the user cluster.
    #[builder(into, default)]
    #[serde(rename = "cluster")]
    pub r#cluster: Box<Option<String>>,
    /// The name of the vCenter datacenter for the user cluster.
    #[builder(into, default)]
    #[serde(rename = "datacenter")]
    pub r#datacenter: Box<Option<String>>,
    /// The name of the vCenter datastore for the user cluster.
    #[builder(into, default)]
    #[serde(rename = "datastore")]
    pub r#datastore: Box<Option<String>>,
    /// The name of the vCenter folder for the user cluster.
    #[builder(into, default)]
    #[serde(rename = "folder")]
    pub r#folder: Box<Option<String>>,
    /// The name of the vCenter resource pool for the user cluster.
    #[builder(into, default)]
    #[serde(rename = "resourcePool")]
    pub r#resource_pool: Box<Option<String>>,
    /// The name of the vCenter storage policy for the user cluster.
    #[builder(into, default)]
    #[serde(rename = "storagePolicyName")]
    pub r#storage_policy_name: Box<Option<String>>,
}
