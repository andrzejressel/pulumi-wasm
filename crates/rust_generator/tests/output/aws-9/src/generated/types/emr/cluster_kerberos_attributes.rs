#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct ClusterKerberosAttributes {
    /// Active Directory password for `ad_domain_join_user`. This provider cannot perform drift detection of this configuration.
    #[builder(into, default)]
    #[serde(rename = "adDomainJoinPassword")]
    pub r#ad_domain_join_password: Box<Option<String>>,
    /// Required only when establishing a cross-realm trust with an Active Directory domain. A user with sufficient privileges to join resources to the domain. This provider cannot perform drift detection of this configuration.
    #[builder(into, default)]
    #[serde(rename = "adDomainJoinUser")]
    pub r#ad_domain_join_user: Box<Option<String>>,
    /// Required only when establishing a cross-realm trust with a KDC in a different realm. The cross-realm principal password, which must be identical across realms. This provider cannot perform drift detection of this configuration.
    #[builder(into, default)]
    #[serde(rename = "crossRealmTrustPrincipalPassword")]
    pub r#cross_realm_trust_principal_password: Box<Option<String>>,
    /// Password used within the cluster for the kadmin service on the cluster-dedicated KDC, which maintains Kerberos principals, password policies, and keytabs for the cluster. This provider cannot perform drift detection of this configuration.
    #[builder(into)]
    #[serde(rename = "kdcAdminPassword")]
    pub r#kdc_admin_password: Box<String>,
    /// Name of the Kerberos realm to which all nodes in a cluster belong. For example, `EC2.INTERNAL`
    #[builder(into)]
    #[serde(rename = "realm")]
    pub r#realm: Box<String>,
}
