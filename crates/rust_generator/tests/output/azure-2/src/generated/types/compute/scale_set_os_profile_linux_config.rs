#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ScaleSetOsProfileLinuxConfig {
    /// Specifies whether password authentication should be disabled. Defaults to `false`. Changing this forces a new resource to be created.
    #[builder(into, default)]
    #[serde(rename = "disablePasswordAuthentication")]
    pub r#disable_password_authentication: Box<Option<bool>>,
    /// One or more `ssh_keys` blocks as defined below.
    /// 
    /// > **Note:** Please note that the only allowed `path` is `/home/<username>/.ssh/authorized_keys` due to a limitation of Azure.
    /// 
    /// > **NOTE:** At least one `ssh_keys` block is required if `disable_password_authentication` is set to `true`.
    #[builder(into, default)]
    #[serde(rename = "sshKeys")]
    pub r#ssh_keys: Box<Option<Vec<super::super::types::compute::ScaleSetOsProfileLinuxConfigSshKey>>>,
}
