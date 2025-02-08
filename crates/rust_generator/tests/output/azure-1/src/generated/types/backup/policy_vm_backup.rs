#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct PolicyVmBackup {
    /// Sets the backup frequency. Possible values are `Hourly`, `Daily` and `Weekly`.
    #[builder(into)]
    #[serde(rename = "frequency")]
    pub r#frequency: Box<String>,
    /// Duration of the backup window in hours. Possible values are between `4` and `24` This is used when `frequency` is `Hourly`.
    /// 
    /// > **NOTE:** `hour_duration` must be multiplier of `hour_interval`
    #[builder(into, default)]
    #[serde(rename = "hourDuration")]
    pub r#hour_duration: Box<Option<i32>>,
    /// Interval in hour at which backup is triggered. Possible values are `4`, `6`, `8` and `12`. This is used when `frequency` is `Hourly`.
    #[builder(into, default)]
    #[serde(rename = "hourInterval")]
    pub r#hour_interval: Box<Option<i32>>,
    /// The time of day to perform the backup in 24hour format.
    #[builder(into)]
    #[serde(rename = "time")]
    pub r#time: Box<String>,
    /// The days of the week to perform backups on. Must be one of `Sunday`, `Monday`, `Tuesday`, `Wednesday`, `Thursday`, `Friday` or `Saturday`. This is used when `frequency` is `Weekly`.
    #[builder(into, default)]
    #[serde(rename = "weekdays")]
    pub r#weekdays: Box<Option<Vec<String>>>,
}
