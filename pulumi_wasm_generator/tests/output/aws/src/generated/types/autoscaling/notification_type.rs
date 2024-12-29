#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, Debug, PartialEq, Clone)]
#[allow(dead_code)]
pub enum NotificationType {
    #[serde(rename = "autoscaling:EC2_INSTANCE_LAUNCH")]
    InstanceLaunch,
    #[serde(rename = "autoscaling:EC2_INSTANCE_TERMINATE")]
    InstanceTerminate,
    #[serde(rename = "autoscaling:EC2_INSTANCE_LAUNCH_ERROR")]
    InstanceLaunchError,
    #[serde(rename = "autoscaling:EC2_INSTANCE_TERMINATE_ERROR")]
    InstanceTerminateError,
    #[serde(rename = "autoscaling:TEST_NOTIFICATION")]
    TestNotification,
}
