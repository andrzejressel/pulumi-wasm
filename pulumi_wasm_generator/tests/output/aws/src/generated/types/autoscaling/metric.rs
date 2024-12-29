#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, Debug, PartialEq, Clone)]
#[allow(dead_code)]
pub enum Metric {
    GroupMinSize,
    GroupMaxSize,
    GroupDesiredCapacity,
    GroupInServiceInstances,
    GroupInServiceCapacity,
    GroupPendingInstances,
    GroupPendingCapacity,
    GroupStandbyInstances,
    GroupStandbyCapacity,
    GroupTerminatingInstances,
    GroupTerminatingCapacity,
    GroupTotalInstances,
    GroupTotalCapacity,
    WarmPoolDesiredCapacity,
    WarmPoolWarmedCapacity,
    WarmPoolPendingCapacity,
    WarmPoolTerminatingCapacity,
    WarmPoolTotalCapacity,
    GroupAndWarmPoolDesiredCapacity,
    GroupAndWarmPoolTotalCapacity,
}
