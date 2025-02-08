#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct BucketReplicationConfigRule {
    /// Whether delete markers are replicated. This argument is only valid with V2 replication configurations (i.e., when `filter` is used)documented below.
    #[builder(into, default)]
    #[serde(rename = "deleteMarkerReplication")]
    pub r#delete_marker_replication: Box<Option<super::super::types::s3::BucketReplicationConfigRuleDeleteMarkerReplication>>,
    /// Specifies the destination for the rule. See below.
    #[builder(into)]
    #[serde(rename = "destination")]
    pub r#destination: Box<super::super::types::s3::BucketReplicationConfigRuleDestination>,
    /// Replicate existing objects in the source bucket according to the rule configurations. See below.
    #[builder(into, default)]
    #[serde(rename = "existingObjectReplication")]
    pub r#existing_object_replication: Box<Option<super::super::types::s3::BucketReplicationConfigRuleExistingObjectReplication>>,
    /// Filter that identifies subset of objects to which the replication rule applies. See below. If not specified, the `rule` will default to using `prefix`.
    #[builder(into, default)]
    #[serde(rename = "filter")]
    pub r#filter: Box<Option<super::super::types::s3::BucketReplicationConfigRuleFilter>>,
    /// Unique identifier for the rule. Must be less than or equal to 255 characters in length.
    #[builder(into, default)]
    #[serde(rename = "id")]
    pub r#id: Box<Option<String>>,
    /// Object key name prefix identifying one or more objects to which the rule applies. Must be less than or equal to 1024 characters in length. Defaults to an empty string (`""`) if `filter` is not specified.
    #[builder(into, default)]
    #[serde(rename = "prefix")]
    pub r#prefix: Box<Option<String>>,
    /// Priority associated with the rule. Priority should only be set if `filter` is configured. If not provided, defaults to `0`. Priority must be unique between multiple rules.
    #[builder(into, default)]
    #[serde(rename = "priority")]
    pub r#priority: Box<Option<i32>>,
    /// Specifies special object selection criteria. See below.
    #[builder(into, default)]
    #[serde(rename = "sourceSelectionCriteria")]
    pub r#source_selection_criteria: Box<Option<super::super::types::s3::BucketReplicationConfigRuleSourceSelectionCriteria>>,
    /// Status of the rule. Either `"Enabled"` or `"Disabled"`. The rule is ignored if status is not "Enabled".
    #[builder(into)]
    #[serde(rename = "status")]
    pub r#status: Box<String>,
}
