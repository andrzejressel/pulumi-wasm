#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct PolicyFileShareBackupHourly {
    /// Specifies the interval at which backup needs to be triggered. Possible values are `4`, `6`, `8` and `12`.
    #[builder(into)]
    #[serde(rename = "interval")]
    pub r#interval: Box<i32>,
    /// Specifies the start time of the hourly backup. The time format should be in 24-hour format. Times must be either on the hour or half hour (e.g. 12:00, 12:30, 13:00, etc.).
    #[builder(into)]
    #[serde(rename = "startTime")]
    pub r#start_time: Box<String>,
    /// Species the duration of the backup window in hours. Details could be found [here](https://learn.microsoft.com/en-us/azure/backup/backup-azure-files-faq#what-does-the-duration-attribute-in-azure-files-backup-policy-signify-).
    #[builder(into)]
    #[serde(rename = "windowDuration")]
    pub r#window_duration: Box<i32>,
}
