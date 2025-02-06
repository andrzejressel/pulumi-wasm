#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct PreventionJobTriggerInspectJobAction {
    /// Create a de-identified copy of the requested table or files.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "deidentify")]
    pub r#deidentify: Box<Option<super::super::types::dataloss::PreventionJobTriggerInspectJobActionDeidentify>>,
    /// Sends an email when the job completes. The email goes to IAM project owners and technical Essential Contacts.
    #[builder(into, default)]
    #[serde(rename = "jobNotificationEmails")]
    pub r#job_notification_emails: Box<Option<super::super::types::dataloss::PreventionJobTriggerInspectJobActionJobNotificationEmails>>,
    /// Publish a message into a given Pub/Sub topic when the job completes.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "pubSub")]
    pub r#pub_sub: Box<Option<super::super::types::dataloss::PreventionJobTriggerInspectJobActionPubSub>>,
    /// Publish findings of a DlpJob to Data Catalog.
    #[builder(into, default)]
    #[serde(rename = "publishFindingsToCloudDataCatalog")]
    pub r#publish_findings_to_cloud_data_catalog: Box<Option<super::super::types::dataloss::PreventionJobTriggerInspectJobActionPublishFindingsToCloudDataCatalog>>,
    /// Publish the result summary of a DlpJob to the Cloud Security Command Center.
    #[builder(into, default)]
    #[serde(rename = "publishSummaryToCscc")]
    pub r#publish_summary_to_cscc: Box<Option<super::super::types::dataloss::PreventionJobTriggerInspectJobActionPublishSummaryToCscc>>,
    /// Enable Stackdriver metric dlp.googleapis.com/findingCount.
    #[builder(into, default)]
    #[serde(rename = "publishToStackdriver")]
    pub r#publish_to_stackdriver: Box<Option<super::super::types::dataloss::PreventionJobTriggerInspectJobActionPublishToStackdriver>>,
    /// If set, the detailed findings will be persisted to the specified OutputStorageConfig. Only a single instance of this action can be specified. Compatible with: Inspect, Risk
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "saveFindings")]
    pub r#save_findings: Box<Option<super::super::types::dataloss::PreventionJobTriggerInspectJobActionSaveFindings>>,
}
