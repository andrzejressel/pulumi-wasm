#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct PatchDeploymentPatchConfig {
    /// Apt update settings. Use this setting to override the default apt patch rules.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "apt")]
    pub r#apt: Box<Option<super::super::types::osconfig::PatchDeploymentPatchConfigApt>>,
    /// goo update settings. Use this setting to override the default goo patch rules.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "goo")]
    pub r#goo: Box<Option<super::super::types::osconfig::PatchDeploymentPatchConfigGoo>>,
    /// Allows the patch job to run on Managed instance groups (MIGs).
    #[builder(into, default)]
    #[serde(rename = "migInstancesAllowed")]
    pub r#mig_instances_allowed: Box<Option<bool>>,
    /// The ExecStep to run after the patch update.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "postStep")]
    pub r#post_step: Box<Option<super::super::types::osconfig::PatchDeploymentPatchConfigPostStep>>,
    /// The ExecStep to run before the patch update.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "preStep")]
    pub r#pre_step: Box<Option<super::super::types::osconfig::PatchDeploymentPatchConfigPreStep>>,
    /// Post-patch reboot settings.
    /// Possible values are: `DEFAULT`, `ALWAYS`, `NEVER`.
    #[builder(into, default)]
    #[serde(rename = "rebootConfig")]
    pub r#reboot_config: Box<Option<String>>,
    /// Windows update settings. Use this setting to override the default Windows patch rules.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "windowsUpdate")]
    pub r#windows_update: Box<Option<super::super::types::osconfig::PatchDeploymentPatchConfigWindowsUpdate>>,
    /// Yum update settings. Use this setting to override the default yum patch rules.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "yum")]
    pub r#yum: Box<Option<super::super::types::osconfig::PatchDeploymentPatchConfigYum>>,
    /// zypper update settings. Use this setting to override the default zypper patch rules.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "zypper")]
    pub r#zypper: Box<Option<super::super::types::osconfig::PatchDeploymentPatchConfigZypper>>,
}
