#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct DomainRuleBasedMatchingExportingConfig {
    #[builder(into, default)]
    #[serde(rename = "s3Exporting")]
    pub r#s_3_exporting: Box<Option<super::super::types::customerprofiles::DomainRuleBasedMatchingExportingConfigS3Exporting>>,
}
