#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct VirtualMachineGroupWsfcDomainProfile {
    /// The account name used for creating cluster. Changing this forces a new resource to be created.
    #[builder(into, default)]
    #[serde(rename = "clusterBootstrapAccountName")]
    pub r#cluster_bootstrap_account_name: Box<Option<String>>,
    /// The account name used for operating cluster. Changing this forces a new resource to be created.
    #[builder(into, default)]
    #[serde(rename = "clusterOperatorAccountName")]
    pub r#cluster_operator_account_name: Box<Option<String>>,
    /// The subnet type of the SQL Virtual Machine cluster. Possible values are `MultiSubnet` and `SingleSubnet`. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "clusterSubnetType")]
    pub r#cluster_subnet_type: Box<String>,
    /// The fully qualified name of the domain. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "fqdn")]
    pub r#fqdn: Box<String>,
    /// The organizational Unit path in which the nodes and cluster will be present. Changing this forces a new resource to be created.
    #[builder(into, default)]
    #[serde(rename = "organizationalUnitPath")]
    pub r#organizational_unit_path: Box<Option<String>>,
    /// The account name under which SQL service will run on all participating SQL virtual machines in the cluster. Changing this forces a new resource to be created.
    #[builder(into, default)]
    #[serde(rename = "sqlServiceAccountName")]
    pub r#sql_service_account_name: Box<Option<String>>,
    /// The primary key of the Storage Account.
    #[builder(into, default)]
    #[serde(rename = "storageAccountPrimaryKey")]
    pub r#storage_account_primary_key: Box<Option<String>>,
    /// The SAS URL to the Storage Container of the witness storage account. Changing this forces a new resource to be created.
    #[builder(into, default)]
    #[serde(rename = "storageAccountUrl")]
    pub r#storage_account_url: Box<Option<String>>,
}
