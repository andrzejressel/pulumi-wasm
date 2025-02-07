#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct VMwareClusterControlPlaneNodeAutoResizeConfig {
    /// Whether to enable control plane node auto resizing.
    /// 
    /// <a name="nested_vsphere_config"></a>The `vsphere_config` block contains:
    #[builder(into)]
    #[serde(rename = "enabled")]
    pub r#enabled: Box<bool>,
}
