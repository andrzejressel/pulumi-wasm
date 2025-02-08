#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct GatewaySmbActiveDirectorySettings {
    #[builder(into, default)]
    #[serde(rename = "activeDirectoryStatus")]
    pub r#active_directory_status: Box<Option<String>>,
    /// List of IPv4 addresses, NetBIOS names, or host names of your domain server.
    /// If you need to specify the port number include it after the colon (“:”). For example, `mydc.mydomain.com:389`.
    #[builder(into, default)]
    #[serde(rename = "domainControllers")]
    pub r#domain_controllers: Box<Option<Vec<String>>>,
    /// The name of the domain that you want the gateway to join.
    #[builder(into)]
    #[serde(rename = "domainName")]
    pub r#domain_name: Box<String>,
    /// The organizational unit (OU) is a container in an Active Directory that can hold users, groups,
    /// computers, and other OUs and this parameter specifies the OU that the gateway will join within the AD domain.
    #[builder(into, default)]
    #[serde(rename = "organizationalUnit")]
    pub r#organizational_unit: Box<Option<String>>,
    /// The password of the user who has permission to add the gateway to the Active Directory domain.
    #[builder(into)]
    #[serde(rename = "password")]
    pub r#password: Box<String>,
    /// Specifies the time in seconds, in which the JoinDomain operation must complete. The default is `20` seconds.
    #[builder(into, default)]
    #[serde(rename = "timeoutInSeconds")]
    pub r#timeout_in_seconds: Box<Option<i32>>,
    /// The user name of user who has permission to add the gateway to the Active Directory domain.
    #[builder(into)]
    #[serde(rename = "username")]
    pub r#username: Box<String>,
}
