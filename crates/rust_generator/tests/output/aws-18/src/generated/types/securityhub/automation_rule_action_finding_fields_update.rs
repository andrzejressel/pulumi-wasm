#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct AutomationRuleActionFindingFieldsUpdate {
    /// The rule action updates the `Confidence` field of a finding.
    #[builder(into, default)]
    #[serde(rename = "confidence")]
    pub r#confidence: Box<Option<i32>>,
    /// The rule action updates the `Criticality` field of a finding.
    #[builder(into, default)]
    #[serde(rename = "criticality")]
    pub r#criticality: Box<Option<i32>>,
    /// A resource block that updates the note. Documented below.
    #[builder(into, default)]
    #[serde(rename = "note")]
    pub r#note: Box<Option<super::super::types::securityhub::AutomationRuleActionFindingFieldsUpdateNote>>,
    /// A resource block that the rule action updates the `RelatedFindings` field of a finding. Documented below.
    #[builder(into, default)]
    #[serde(rename = "relatedFindings")]
    pub r#related_findings: Box<Option<Vec<super::super::types::securityhub::AutomationRuleActionFindingFieldsUpdateRelatedFinding>>>,
    /// A resource block that updates to the severity information for a finding. Documented below.
    #[builder(into, default)]
    #[serde(rename = "severity")]
    pub r#severity: Box<Option<super::super::types::securityhub::AutomationRuleActionFindingFieldsUpdateSeverity>>,
    /// The rule action updates the `Types` field of a finding.
    #[builder(into, default)]
    #[serde(rename = "types")]
    pub r#types: Box<Option<Vec<String>>>,
    /// The rule action updates the `UserDefinedFields` field of a finding.
    #[builder(into, default)]
    #[serde(rename = "userDefinedFields")]
    pub r#user_defined_fields: Box<Option<std::collections::HashMap<String, String>>>,
    /// The rule action updates the `VerificationState` field of a finding. The allowed values are the following `UNKNOWN`, `TRUE_POSITIVE`, `FALSE_POSITIVE` and `BENIGN_POSITIVE`.
    #[builder(into, default)]
    #[serde(rename = "verificationState")]
    pub r#verification_state: Box<Option<String>>,
    /// A resource block that is used to update information about the investigation into the finding. Documented below.
    #[builder(into, default)]
    #[serde(rename = "workflow")]
    pub r#workflow: Box<Option<super::super::types::securityhub::AutomationRuleActionFindingFieldsUpdateWorkflow>>,
}
