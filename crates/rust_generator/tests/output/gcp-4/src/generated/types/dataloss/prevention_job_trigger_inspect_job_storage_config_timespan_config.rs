#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct PreventionJobTriggerInspectJobStorageConfigTimespanConfig {
    /// When the job is started by a JobTrigger we will automatically figure out a valid startTime to avoid
    /// scanning files that have not been modified since the last time the JobTrigger executed. This will
    /// be based on the time of the execution of the last run of the JobTrigger or the timespan endTime
    /// used in the last run of the JobTrigger.
    #[builder(into, default)]
    #[serde(rename = "enableAutoPopulationOfTimespanConfig")]
    pub r#enable_auto_population_of_timespan_config: Box<Option<bool>>,
    /// Exclude files, tables, or rows newer than this value. If not set, no upper time limit is applied.
    #[builder(into, default)]
    #[serde(rename = "endTime")]
    pub r#end_time: Box<Option<String>>,
    /// Exclude files, tables, or rows older than this value. If not set, no lower time limit is applied.
    #[builder(into, default)]
    #[serde(rename = "startTime")]
    pub r#start_time: Box<Option<String>>,
    /// Specification of the field containing the timestamp of scanned items.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "timestampField")]
    pub r#timestamp_field: Box<Option<super::super::types::dataloss::PreventionJobTriggerInspectJobStorageConfigTimespanConfigTimestampField>>,
}
