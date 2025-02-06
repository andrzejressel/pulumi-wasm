#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct CacheAccessPolicyAccessRule {
    /// The access level for this rule. Possible values are: `rw`, `ro`, `no`.
    #[builder(into)]
    #[serde(rename = "access")]
    pub r#access: Box<String>,
    /// The anonymous GID used when `root_squash_enabled` is `true`.
    #[builder(into, default)]
    #[serde(rename = "anonymousGid")]
    pub r#anonymous_gid: Box<Option<i32>>,
    /// The anonymous UID used when `root_squash_enabled` is `true`.
    #[builder(into, default)]
    #[serde(rename = "anonymousUid")]
    pub r#anonymous_uid: Box<Option<i32>>,
    /// The filter applied to the `scope` for this rule. The filter's format depends on its scope: `default` scope matches all clients and has no filter value; `network` scope takes a CIDR format; `host` takes an IP address or fully qualified domain name. If a client does not match any filter rule and there is no default rule, access is denied.
    #[builder(into, default)]
    #[serde(rename = "filter")]
    pub r#filter: Box<Option<String>>,
    /// Whether to enable [root squash](https://docs.microsoft.com/azure/hpc-cache/access-policies#root-squash)?
    #[builder(into, default)]
    #[serde(rename = "rootSquashEnabled")]
    pub r#root_squash_enabled: Box<Option<bool>>,
    /// The scope of this rule. The `scope` and (potentially) the `filter` determine which clients match the rule. Possible values are: `default`, `network`, `host`.
    /// 
    /// > **NOTE:** Each `access_rule` should set a unique `scope`.
    #[builder(into)]
    #[serde(rename = "scope")]
    pub r#scope: Box<String>,
    /// Whether allow access to subdirectories under the root export?
    #[builder(into, default)]
    #[serde(rename = "submountAccessEnabled")]
    pub r#submount_access_enabled: Box<Option<bool>>,
    /// Whether [SUID](https://docs.microsoft.com/azure/hpc-cache/access-policies#suid) is allowed?
    #[builder(into, default)]
    #[serde(rename = "suidEnabled")]
    pub r#suid_enabled: Box<Option<bool>>,
}
