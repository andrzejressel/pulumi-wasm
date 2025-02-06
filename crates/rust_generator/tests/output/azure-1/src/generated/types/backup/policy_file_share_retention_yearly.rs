#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct PolicyFileShareRetentionYearly {
    /// The number of yearly backups to keep. Must be between `1` and `10`
    #[builder(into)]
    #[serde(rename = "count")]
    pub r#count: Box<i32>,
    /// The days of the month to retain backups of. Must be between `1` and `31`.
    #[builder(into, default)]
    #[serde(rename = "days")]
    pub r#days: Box<Option<Vec<i32>>>,
    /// Including the last day of the month, default to `false`.
    /// 
    /// > **NOTE:**: Either `weekdays` and `weeks` or `days` and `include_last_days` must be specified.
    #[builder(into, default)]
    #[serde(rename = "includeLastDays")]
    pub r#include_last_days: Box<Option<bool>>,
    /// The months of the year to retain backups of. Must be one of `January`, `February`, `March`, `April`, `May`, `June`, `July`, `Augest`, `September`, `October`, `November` and `December`.
    #[builder(into)]
    #[serde(rename = "months")]
    pub r#months: Box<Vec<String>>,
    /// The weekday backups to retain . Must be one of `Sunday`, `Monday`, `Tuesday`, `Wednesday`, `Thursday`, `Friday` or `Saturday`.
    #[builder(into, default)]
    #[serde(rename = "weekdays")]
    pub r#weekdays: Box<Option<Vec<String>>>,
    /// The weeks of the month to retain backups of. Must be one of `First`, `Second`, `Third`, `Fourth`, `Last`.
    #[builder(into, default)]
    #[serde(rename = "weeks")]
    pub r#weeks: Box<Option<Vec<String>>>,
}
