#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetGroupWarmPoolInstanceReusePolicy {
    /// Indicates whether instances in the Auto Scaling group can be returned to the warm pool on scale in.
    #[builder(into)]
    #[serde(rename = "reuseOnScaleIn")]
    pub r#reuse_on_scale_in: Box<bool>,
}
