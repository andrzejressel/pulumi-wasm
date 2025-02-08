#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct RegionPerInstanceConfigPreservedStateExternalIp {
    /// These stateful IPs will never be released during autohealing, update or VM instance recreate operations. This flag is used to configure if the IP reservation should be deleted after it is no longer used by the group, e.g. when the given instance or the whole group is deleted.
    /// Default value is `NEVER`.
    /// Possible values are: `NEVER`, `ON_PERMANENT_INSTANCE_DELETION`.
    #[builder(into, default)]
    #[serde(rename = "autoDelete")]
    pub r#auto_delete: Box<Option<String>>,
    /// The identifier for this object. Format specified above.
    #[builder(into)]
    #[serde(rename = "interfaceName")]
    pub r#interface_name: Box<String>,
    /// Ip address representation
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "ipAddress")]
    pub r#ip_address: Box<Option<super::super::types::compute::RegionPerInstanceConfigPreservedStateExternalIpIpAddress>>,
}
