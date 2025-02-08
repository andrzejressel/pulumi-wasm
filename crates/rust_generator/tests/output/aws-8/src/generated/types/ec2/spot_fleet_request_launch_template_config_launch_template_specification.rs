#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct SpotFleetRequestLaunchTemplateConfigLaunchTemplateSpecification {
    /// The ID of the launch template. Conflicts with `name`.
    #[builder(into, default)]
    #[serde(rename = "id")]
    pub r#id: Box<Option<String>>,
    /// The name of the launch template. Conflicts with `id`.
    #[builder(into, default)]
    #[serde(rename = "name")]
    pub r#name: Box<Option<String>>,
    /// Template version. Unlike the autoscaling equivalent, does not support `$Latest` or `$Default`, so use the launch_template resource's attribute, e.g., `"${aws_launch_template.foo.latest_version}"`. It will use the default version if omitted.
    /// 
    /// **Note:** The specified launch template can specify only a subset of the
    /// inputs of `aws.ec2.LaunchTemplate`.  There are limitations on
    /// what you can specify as spot fleet does not support all the attributes that are supported by autoscaling groups. [AWS documentation](https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/ec2-launch-templates.html#launch-templates-spot-fleet) is currently sparse, but at least `instance_initiated_shutdown_behavior` is confirmed unsupported.
    #[builder(into, default)]
    #[serde(rename = "version")]
    pub r#version: Box<Option<String>>,
}
