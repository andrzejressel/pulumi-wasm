#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetLinuxWebAppBackup {
    /// Is the Backup enabled?
    #[builder(into)]
    #[serde(rename = "enabled")]
    pub r#enabled: Box<bool>,
    /// The name of this Linux Web App.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// A `schedule` block as defined below.
    #[builder(into)]
    #[serde(rename = "schedules")]
    pub r#schedules: Box<Vec<super::super::types::appservice::GetLinuxWebAppBackupSchedule>>,
    /// The SAS URL to the container.
    #[builder(into)]
    #[serde(rename = "storageAccountUrl")]
    pub r#storage_account_url: Box<String>,
}
