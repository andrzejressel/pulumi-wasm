#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GroupTag {
    /// Key
    #[builder(into)]
    #[serde(rename = "key")]
    pub r#key: Box<String>,
    /// Enables propagation of the tag to
    /// Amazon EC2 instances launched via this ASG
    /// 
    /// To declare multiple tags, additional `tag` blocks can be specified.
    /// 
    /// > **NOTE:** Other AWS APIs may automatically add special tags to their associated Auto Scaling Group for management purposes, such as ECS Capacity Providers adding the `AmazonECSManaged` tag. These generally should be included in the configuration so the provider does not attempt to remove them and so if the `min_size` was greater than zero on creation, that these tag(s) are applied to any initial EC2 Instances in the Auto Scaling Group. If these tag(s) were missing in the Auto Scaling Group configuration on creation, affected EC2 Instances missing the tags may require manual intervention of adding the tags to ensure they work properly with the other AWS service.
    #[builder(into)]
    #[serde(rename = "propagateAtLaunch")]
    pub r#propagate_at_launch: Box<bool>,
    /// Value
    #[builder(into)]
    #[serde(rename = "value")]
    pub r#value: Box<String>,
}
