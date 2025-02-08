#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct ControlControlMappingSource {
    /// Description of the source.
    #[builder(into, default)]
    #[serde(rename = "sourceDescription")]
    pub r#source_description: Box<Option<String>>,
    /// Frequency of evidence collection. Valid values are `DAILY`, `WEEKLY`, or `MONTHLY`.
    #[builder(into, default)]
    #[serde(rename = "sourceFrequency")]
    pub r#source_frequency: Box<Option<String>>,
    #[builder(into, default)]
    #[serde(rename = "sourceId")]
    pub r#source_id: Box<Option<String>>,
    /// The keyword to search for in CloudTrail logs, Config rules, Security Hub checks, and Amazon Web Services API names. See `source_keyword` below.
    #[builder(into, default)]
    #[serde(rename = "sourceKeyword")]
    pub r#source_keyword: Box<Option<super::super::types::auditmanager::ControlControlMappingSourceSourceKeyword>>,
    /// Name of the source.
    #[builder(into)]
    #[serde(rename = "sourceName")]
    pub r#source_name: Box<String>,
    /// The setup option for the data source. This option reflects if the evidence collection is automated or manual. Valid values are `System_Controls_Mapping` (automated) and `Procedural_Controls_Mapping` (manual).
    #[builder(into)]
    #[serde(rename = "sourceSetUpOption")]
    pub r#source_set_up_option: Box<String>,
    /// Type of data source for evidence collection. If `source_set_up_option` is manual, the only valid value is `MANUAL`. If `source_set_up_option` is automated, valid values are `AWS_Cloudtrail`, `AWS_Config`, `AWS_Security_Hub`, or `AWS_API_Call`.
    /// 
    /// The following arguments are optional:
    #[builder(into)]
    #[serde(rename = "sourceType")]
    pub r#source_type: Box<String>,
    /// Instructions for troubleshooting the control.
    #[builder(into, default)]
    #[serde(rename = "troubleshootingText")]
    pub r#troubleshooting_text: Box<Option<String>>,
}
