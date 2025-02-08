#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct BucketReplicationConfigurationRule {
    /// Whether delete markers are replicated. The only valid value is `Enabled`. To disable, omit this argument. This argument is only valid with V2 replication configurations (i.e., when `filter` is used).
    #[builder(into, default)]
    #[serde(rename = "deleteMarkerReplicationStatus")]
    pub r#delete_marker_replication_status: Box<Option<String>>,
    /// Specifies the destination for the rule (documented below).
    #[builder(into)]
    #[serde(rename = "destination")]
    pub r#destination: Box<super::super::types::s3::BucketReplicationConfigurationRuleDestination>,
    /// Filter that identifies subset of objects to which the replication rule applies (documented below).
    #[builder(into, default)]
    #[serde(rename = "filter")]
    pub r#filter: Box<Option<super::super::types::s3::BucketReplicationConfigurationRuleFilter>>,
    /// Unique identifier for the rule. Must be less than or equal to 255 characters in length.
    #[builder(into, default)]
    #[serde(rename = "id")]
    pub r#id: Box<Option<String>>,
    /// Object keyname prefix identifying one or more objects to which the rule applies. Must be less than or equal to 1024 characters in length.
    #[builder(into, default)]
    #[serde(rename = "prefix")]
    pub r#prefix: Box<Option<String>>,
    /// The priority associated with the rule. Priority should only be set if `filter` is configured. If not provided, defaults to `0`. Priority must be unique between multiple rules.
    #[builder(into, default)]
    #[serde(rename = "priority")]
    pub r#priority: Box<Option<i32>>,
    /// Specifies special object selection criteria (documented below).
    #[builder(into, default)]
    #[serde(rename = "sourceSelectionCriteria")]
    pub r#source_selection_criteria: Box<Option<super::super::types::s3::BucketReplicationConfigurationRuleSourceSelectionCriteria>>,
    /// The status of the rule. Either `Enabled` or `Disabled`. The rule is ignored if status is not Enabled.
    /// 
    /// > **NOTE:** Replication to multiple destination buckets requires that `priority` is specified in the `rules` object. If the corresponding rule requires no filter, an empty configuration block `filter {}` must be specified.
    #[builder(into)]
    #[serde(rename = "status")]
    pub r#status: Box<String>,
}
