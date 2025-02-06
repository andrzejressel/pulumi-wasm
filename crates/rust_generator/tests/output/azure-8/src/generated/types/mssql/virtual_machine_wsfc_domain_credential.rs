#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct VirtualMachineWsfcDomainCredential {
    /// The account password used for creating cluster.
    #[builder(into)]
    #[serde(rename = "clusterBootstrapAccountPassword")]
    pub r#cluster_bootstrap_account_password: Box<String>,
    /// The account password used for operating cluster.
    #[builder(into)]
    #[serde(rename = "clusterOperatorAccountPassword")]
    pub r#cluster_operator_account_password: Box<String>,
    /// The account password under which SQL service will run on all participating SQL virtual machines in the cluster.
    #[builder(into)]
    #[serde(rename = "sqlServiceAccountPassword")]
    pub r#sql_service_account_password: Box<String>,
}
