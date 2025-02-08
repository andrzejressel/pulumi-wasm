#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ComputeClusterSsh {
    /// Password of the administrator user account. Changing this forces a new Machine Learning Compute Cluster to be created.
    #[builder(into, default)]
    #[serde(rename = "adminPassword")]
    pub r#admin_password: Box<Option<String>>,
    /// Name of the administrator user account which can be used to SSH to nodes. Changing this forces a new Machine Learning Compute Cluster to be created.
    #[builder(into)]
    #[serde(rename = "adminUsername")]
    pub r#admin_username: Box<String>,
    /// SSH public key of the administrator user account. Changing this forces a new Machine Learning Compute Cluster to be created.
    /// 
    /// > **NOTE:** At least one of `admin_password` and `key_value` shoud be specified.
    #[builder(into, default)]
    #[serde(rename = "keyValue")]
    pub r#key_value: Box<Option<String>>,
}
