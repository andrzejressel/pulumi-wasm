#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ClusterScalingConfiguration {
    /// Whether to enable automatic pause. A DB cluster can be paused only when it's idle (it has no connections). If a DB cluster is paused for more than seven days, the DB cluster might be backed up with a snapshot. In this case, the DB cluster is restored when there is a request to connect to it. Defaults to `true`.
    #[builder(into, default)]
    #[serde(rename = "autoPause")]
    pub r#auto_pause: Box<Option<bool>>,
    /// Maximum capacity for an Aurora DB cluster in `serverless` DB engine mode. The maximum capacity must be greater than or equal to the minimum capacity. Valid Aurora MySQL capacity values are `1`, `2`, `4`, `8`, `16`, `32`, `64`, `128`, `256`. Valid Aurora PostgreSQL capacity values are (`2`, `4`, `8`, `16`, `32`, `64`, `192`, and `384`). Defaults to `16`.
    #[builder(into, default)]
    #[serde(rename = "maxCapacity")]
    pub r#max_capacity: Box<Option<i32>>,
    /// Minimum capacity for an Aurora DB cluster in `serverless` DB engine mode. The minimum capacity must be lesser than or equal to the maximum capacity. Valid Aurora MySQL capacity values are `1`, `2`, `4`, `8`, `16`, `32`, `64`, `128`, `256`. Valid Aurora PostgreSQL capacity values are (`2`, `4`, `8`, `16`, `32`, `64`, `192`, and `384`). Defaults to `1`.
    #[builder(into, default)]
    #[serde(rename = "minCapacity")]
    pub r#min_capacity: Box<Option<i32>>,
    /// Amount of time, in seconds, that Aurora Serverless v1 tries to find a scaling point to perform seamless scaling before enforcing the timeout action. Valid values are `60` through `600`. Defaults to `300`.
    #[builder(into, default)]
    #[serde(rename = "secondsBeforeTimeout")]
    pub r#seconds_before_timeout: Box<Option<i32>>,
    /// Time, in seconds, before an Aurora DB cluster in serverless mode is paused. Valid values are `300` through `86400`. Defaults to `300`.
    #[builder(into, default)]
    #[serde(rename = "secondsUntilAutoPause")]
    pub r#seconds_until_auto_pause: Box<Option<i32>>,
    /// Action to take when the timeout is reached. Valid values: `ForceApplyCapacityChange`, `RollbackCapacityChange`. Defaults to `RollbackCapacityChange`. See [documentation](https://docs.aws.amazon.com/AmazonRDS/latest/AuroraUserGuide/aurora-serverless-v1.how-it-works.html#aurora-serverless.how-it-works.timeout-action).
    #[builder(into, default)]
    #[serde(rename = "timeoutAction")]
    pub r#timeout_action: Box<Option<String>>,
}
