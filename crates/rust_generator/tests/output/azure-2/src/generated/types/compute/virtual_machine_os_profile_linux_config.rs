#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct VirtualMachineOsProfileLinuxConfig {
    /// Specifies whether password authentication should be disabled. If set to `false`, an `admin_password` must be specified.
    #[builder(into)]
    #[serde(rename = "disablePasswordAuthentication")]
    pub r#disable_password_authentication: Box<bool>,
    /// One or more `ssh_keys` blocks as defined below. This field is required if `disable_password_authentication` is set to `true`.
    #[builder(into, default)]
    #[serde(rename = "sshKeys")]
    pub r#ssh_keys: Box<Option<Vec<super::super::types::compute::VirtualMachineOsProfileLinuxConfigSshKey>>>,
}
