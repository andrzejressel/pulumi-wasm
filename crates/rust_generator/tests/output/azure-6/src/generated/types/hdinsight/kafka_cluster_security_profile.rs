#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct KafkaClusterSecurityProfile {
    /// The resource ID of the Azure Active Directory Domain Service. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "aaddsResourceId")]
    pub r#aadds_resource_id: Box<String>,
    /// A list of the distinguished names for the cluster user groups. Changing this forces a new resource to be created.
    #[builder(into, default)]
    #[serde(rename = "clusterUsersGroupDns")]
    pub r#cluster_users_group_dns: Box<Option<Vec<String>>>,
    /// The name of the Azure Active Directory Domain. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "domainName")]
    pub r#domain_name: Box<String>,
    /// The user password of the Azure Active Directory Domain. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "domainUserPassword")]
    pub r#domain_user_password: Box<String>,
    /// The username of the Azure Active Directory Domain. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "domainUsername")]
    pub r#domain_username: Box<String>,
    /// A list of the LDAPS URLs to communicate with the Azure Active Directory. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "ldapsUrls")]
    pub r#ldaps_urls: Box<Vec<String>>,
    /// The User Assigned Identity for the HDInsight Cluster. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "msiResourceId")]
    pub r#msi_resource_id: Box<String>,
}
