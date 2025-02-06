#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ClusterDatabaseEncryption {
    /// the key to use to encrypt/decrypt secrets.  See the [DatabaseEncryption definition](https://cloud.google.com/kubernetes-engine/docs/reference/rest/v1beta1/projects.locations.clusters#Cluster.DatabaseEncryption) for more information.
    /// 
    /// <a name="nested_enable_k8s_beta_apis"></a>The `enable_k8s_beta_apis` block supports:
    #[builder(into, default)]
    #[serde(rename = "keyName")]
    pub r#key_name: Box<Option<String>>,
    /// `ENCRYPTED` or `DECRYPTED`
    #[builder(into)]
    #[serde(rename = "state")]
    pub r#state: Box<String>,
}
