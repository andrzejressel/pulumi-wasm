#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct DomainMatchingAutoMerging {
    /// A block that specifies how the auto-merging process should resolve conflicts between different profiles. Documented below.
    #[builder(into, default)]
    #[serde(rename = "conflictResolution")]
    pub r#conflict_resolution: Box<Option<super::super::types::customerprofiles::DomainMatchingAutoMergingConflictResolution>>,
    /// A block that specifies a list of matching attributes that represent matching criteria. If two profiles meet at least one of the requirements in the matching attributes list, they will be merged. Documented below.
    /// * `min_allowed_confidence_score_for_merging ` - (Optional) A number between 0 and 1 that represents the minimum confidence score required for profiles within a matching group to be merged during the auto-merge process. A higher score means higher similarity required to merge profiles.
    #[builder(into, default)]
    #[serde(rename = "consolidation")]
    pub r#consolidation: Box<Option<super::super::types::customerprofiles::DomainMatchingAutoMergingConsolidation>>,
    /// The flag that enables the auto-merging of duplicate profiles.
    #[builder(into)]
    #[serde(rename = "enabled")]
    pub r#enabled: Box<bool>,
    #[builder(into, default)]
    #[serde(rename = "minAllowedConfidenceScoreForMerging")]
    pub r#min_allowed_confidence_score_for_merging: Box<Option<f64>>,
}
