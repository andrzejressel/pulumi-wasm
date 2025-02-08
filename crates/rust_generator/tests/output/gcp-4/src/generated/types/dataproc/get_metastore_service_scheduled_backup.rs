#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetMetastoreServiceScheduledBackup {
    /// A Cloud Storage URI of a folder, in the format gs://<bucket_name>/<path_inside_bucket>. A sub-folder <backup_folder> containing backup files will be stored below it.
    #[builder(into)]
    #[serde(rename = "backupLocation")]
    pub r#backup_location: Box<String>,
    /// The scheduled interval in Cron format, see https://en.wikipedia.org/wiki/Cron The default is empty: scheduled backup is not enabled. Must be specified to enable scheduled backups.
    #[builder(into)]
    #[serde(rename = "cronSchedule")]
    pub r#cron_schedule: Box<String>,
    /// Defines whether the scheduled backup is enabled. The default value is false.
    #[builder(into)]
    #[serde(rename = "enabled")]
    pub r#enabled: Box<bool>,
    /// Specifies the time zone to be used when interpreting cronSchedule. Must be a time zone name from the time zone database (https://en.wikipedia.org/wiki/List_of_tz_database_time_zones), e.g. America/Los_Angeles or Africa/Abidjan. If left unspecified, the default is UTC.
    #[builder(into)]
    #[serde(rename = "timeZone")]
    pub r#time_zone: Box<String>,
}
