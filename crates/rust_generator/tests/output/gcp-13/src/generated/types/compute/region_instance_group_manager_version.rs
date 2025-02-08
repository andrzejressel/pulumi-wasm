#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct RegionInstanceGroupManagerVersion {
    /// The full URL to an instance template from which all new instances of this version will be created.
    #[builder(into)]
    #[serde(rename = "instanceTemplate")]
    pub r#instance_template: Box<String>,
    /// Version name.
    #[builder(into, default)]
    #[serde(rename = "name")]
    pub r#name: Box<Option<String>>,
    /// The number of instances calculated as a fixed number or a percentage depending on the settings. Structure is documented below.
    /// 
    /// > Exactly one `version` you specify must not have a `target_size` specified. During a rolling update, the instance group manager will fulfill the `target_size`
    /// constraints of every other `version`, and any remaining instances will be provisioned with the version where `target_size` is unset.
    #[builder(into, default)]
    #[serde(rename = "targetSize")]
    pub r#target_size: Box<Option<super::super::types::compute::RegionInstanceGroupManagerVersionTargetSize>>,
}
